//! Bounded asynchronous SSI Data Hub subscription.

use std::time::Duration;

use futures_util::SinkExt as _;
use serde_json::Value;
use tokio_tungstenite::{
    connect_async,
    tungstenite::{
        Message,
        client::IntoClientRequest as _,
        http::{HeaderValue, header::AUTHORIZATION},
    },
};

use super::{
    channel::Channel,
    error::StreamError,
    message::StreamMessage,
    protocol::{
        NegotiateResponse, connect_url, connection_data, negotiate_url, start_url,
        switch_channels_frame,
    },
    session::{Subscription, validate_channel},
};
use crate::api::MarketDataClient;

const MAX_STREAM_MESSAGES: usize = 10_000;

/// A bounded streaming request suitable for both automation and manual use.
#[derive(Debug)]
pub struct StreamOptions {
    channel: String,
    max_messages: usize,
    timeout: Duration,
}

/// SSI `SignalR` streaming client layered on the authenticated REST client.
#[derive(Debug, Clone, Copy)]
pub struct StreamClient<'a> {
    client: &'a MarketDataClient,
}

impl StreamOptions {
    /// Validates a channel, message limit, and timeout.
    pub fn new(
        channel: String,
        max_messages: usize,
        timeout: Duration,
    ) -> Result<Self, StreamError> {
        validate_channel(&channel)?;
        if max_messages == 0 {
            return Err(StreamError::InvalidOptions(
                "max messages must be greater than zero",
            ));
        }
        if max_messages > MAX_STREAM_MESSAGES {
            return Err(StreamError::InvalidOptions(
                "max messages must not exceed 10000",
            ));
        }
        if timeout.is_zero() {
            return Err(StreamError::InvalidOptions(
                "timeout must be greater than zero",
            ));
        }
        Ok(Self {
            channel,
            max_messages,
            timeout,
        })
    }

    /// Builds bounded options from a typed channel.
    pub fn from_channel(
        channel: &Channel,
        max_messages: usize,
        timeout: Duration,
    ) -> Result<Self, StreamError> {
        Self::new(channel.as_str().to_owned(), max_messages, timeout)
    }
}

impl<'a> StreamClient<'a> {
    /// Creates a streaming client that reuses SSI authentication and HTTP settings.
    pub const fn new(client: &'a MarketDataClient) -> Self {
        Self { client }
    }

    /// Collects the requested number of broadcast payloads before the timeout.
    pub async fn collect(&self, options: &StreamOptions) -> Result<Vec<Value>, StreamError> {
        tokio::time::timeout(options.timeout, self.collect_inner(options))
            .await
            .map_err(|_| StreamError::TimedOut(options.timeout))?
    }

    /// Explicitly collects raw JSON broadcast envelopes.
    pub async fn collect_raw(&self, options: &StreamOptions) -> Result<Vec<Value>, StreamError> {
        self.collect(options).await
    }

    /// Collects and decodes the requested number of typed stream messages.
    pub async fn collect_typed(
        &self,
        options: &StreamOptions,
    ) -> Result<Vec<StreamMessage>, StreamError> {
        self.collect(options)
            .await?
            .into_iter()
            .map(StreamMessage::try_from)
            .collect::<Result<Vec<_>, _>>()
            .map_err(StreamError::from)
    }

    async fn collect_inner(&self, options: &StreamOptions) -> Result<Vec<Value>, StreamError> {
        let mut subscription = self
            .subscribe_inner(&options.channel, options.timeout)
            .await?;
        let mut payloads = Vec::with_capacity(options.max_messages.min(64));
        while payloads.len() < options.max_messages {
            let Some(payload) = subscription.recv().await? else {
                return Err(StreamError::Closed);
            };
            payloads.push(payload);
        }
        subscription.close().await?;
        Ok(payloads)
    }

    /// Opens a persistent subscription after completing the `SignalR` 1.3 handshake.
    pub async fn subscribe(
        &self,
        initial_channel: &str,
        handshake_timeout: Duration,
    ) -> Result<Subscription, StreamError> {
        validate_channel(initial_channel)?;
        if handshake_timeout.is_zero() {
            return Err(StreamError::InvalidOptions(
                "handshake timeout must be greater than zero",
            ));
        }
        tokio::time::timeout(
            handshake_timeout,
            self.subscribe_inner(initial_channel, handshake_timeout),
        )
        .await
        .map_err(|_| StreamError::TimedOut(handshake_timeout))?
    }

    /// Explicitly opens a persistent subscription with a raw channel string.
    pub async fn subscribe_raw(
        &self,
        initial_channel: &str,
        handshake_timeout: Duration,
    ) -> Result<Subscription, StreamError> {
        self.subscribe(initial_channel, handshake_timeout).await
    }

    /// Opens a persistent subscription with a typed channel.
    pub async fn subscribe_typed(
        &self,
        initial_channel: &Channel,
        handshake_timeout: Duration,
    ) -> Result<Subscription, StreamError> {
        self.subscribe(initial_channel.as_str(), handshake_timeout)
            .await
    }

    async fn subscribe_inner(
        &self,
        initial_channel: &str,
        control_timeout: Duration,
    ) -> Result<Subscription, StreamError> {
        let token = self.client.access_token().await?;
        let data = connection_data()?;
        let negotiate: NegotiateResponse = self
            .client
            .http()
            .get(negotiate_url(self.client.settings().stream_url(), &data)?)
            .bearer_auth(token.expose())
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        if !negotiate.try_web_sockets {
            return Err(StreamError::WebSocketsUnavailable);
        }
        if negotiate.protocol_version != super::protocol::CLIENT_PROTOCOL {
            return Err(StreamError::UnsupportedProtocol(negotiate.protocol_version));
        }

        let url = connect_url(
            self.client.settings().stream_url(),
            &data,
            &negotiate.connection_token,
        )?;
        let mut request = url.as_str().into_client_request()?;
        let authorization = HeaderValue::from_str(&format!("Bearer {}", token.expose()))?;
        request.headers_mut().insert(AUTHORIZATION, authorization);
        let (mut socket, _) = connect_async(request).await?;

        let started: StartResponse = self
            .client
            .http()
            .get(start_url(
                self.client.settings().stream_url(),
                &data,
                &negotiate.connection_token,
            )?)
            .bearer_auth(token.expose())
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;
        if !started.response.eq_ignore_ascii_case("started") {
            return Err(StreamError::UnexpectedStart);
        }

        socket
            .send(Message::Text(
                switch_channels_frame(initial_channel, 1).to_string().into(),
            ))
            .await?;
        Ok(Subscription::new(socket, control_timeout))
    }
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
struct StartResponse {
    response: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_unbounded_message_capacity() {
        // Given / When
        let result = StreamOptions::new("MI:VN30".to_owned(), usize::MAX, Duration::from_secs(15));

        // Then
        assert!(result.is_err());
    }
}

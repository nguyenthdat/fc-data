//! Bounded asynchronous SSI Data Hub subscription.

use std::time::Duration;

use futures_util::{SinkExt as _, StreamExt as _};
use serde_json::Value;
use thiserror::Error;
use tokio_tungstenite::{
    connect_async,
    tungstenite::{
        Message,
        client::IntoClientRequest as _,
        http::{HeaderValue, header::AUTHORIZATION},
    },
};

use super::protocol::{
    NegotiateResponse, ProtocolError, broadcast_payloads, connect_url, connection_data,
    negotiate_url, start_url, switch_channels_frame,
};
use crate::api::{ClientError, MarketDataClient};

const MAX_STREAM_MESSAGES: usize = 10_000;

/// A bounded streaming request suitable for both automation and manual use.
#[derive(Debug)]
pub struct StreamOptions {
    channel: String,
    max_messages: usize,
    timeout: Duration,
}

/// SSI legacy `SignalR` streaming failure.
#[derive(Debug, Error)]
pub enum StreamError {
    /// Access-token acquisition failed.
    #[error(transparent)]
    Client(#[from] ClientError),
    /// Legacy `SignalR` protocol construction or parsing failed.
    #[error(transparent)]
    Protocol(#[from] ProtocolError),
    /// An HTTP negotiate or start request failed.
    #[error("SSI streaming HTTP request failed: {0}")]
    Http(reqwest::Error),
    /// The WebSocket handshake or stream failed.
    #[error("SSI streaming WebSocket failed: {0}")]
    WebSocket(#[from] tokio_tungstenite::tungstenite::Error),
    /// The bearer token could not be represented as an HTTP header.
    #[error("SSI bearer token contained invalid header bytes")]
    AuthorizationHeader(#[from] tokio_tungstenite::tungstenite::http::header::InvalidHeaderValue),
    /// SSI did not permit WebSocket transport.
    #[error("SSI negotiate response does not permit WebSockets")]
    WebSocketsUnavailable,
    /// The negotiated protocol was not compatible with this client.
    #[error("SSI negotiated unsupported SignalR protocol {0}")]
    UnsupportedProtocol(String),
    /// The stream ended before enough data arrived.
    #[error("SSI closed the stream before the requested messages arrived")]
    Closed,
    /// No matching broadcast arrived before the configured timeout.
    #[error("no SSI broadcast arrived within {0:?}")]
    TimedOut(Duration),
    /// Stream options were empty or zero.
    #[error("invalid stream option: {0}")]
    InvalidOptions(&'static str),
    /// SSI returned an unexpected legacy start response.
    #[error("SSI streaming start response was not 'started'")]
    UnexpectedStart,
}

/// SSI legacy `SignalR` client layered on the authenticated REST client.
#[derive(Debug)]
pub struct LegacyStreamClient<'a> {
    client: &'a MarketDataClient,
}

impl StreamOptions {
    /// Validates a channel, message limit, and timeout.
    pub fn new(
        channel: String,
        max_messages: usize,
        timeout: Duration,
    ) -> Result<Self, StreamError> {
        if channel.trim().is_empty() {
            return Err(StreamError::InvalidOptions("channel must not be empty"));
        }
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
}

impl<'a> LegacyStreamClient<'a> {
    /// Creates a legacy streaming client that reuses SSI authentication and HTTP settings.
    pub const fn new(client: &'a MarketDataClient) -> Self {
        Self { client }
    }

    /// Collects the requested number of broadcast payloads before the timeout.
    pub async fn collect(&self, options: &StreamOptions) -> Result<Vec<Value>, StreamError> {
        tokio::time::timeout(options.timeout, self.collect_inner(options))
            .await
            .map_err(|_| StreamError::TimedOut(options.timeout))?
    }

    async fn collect_inner(&self, options: &StreamOptions) -> Result<Vec<Value>, StreamError> {
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
                switch_channels_frame(&options.channel, 1)
                    .to_string()
                    .into(),
            ))
            .await?;

        let mut payloads = Vec::with_capacity(options.max_messages.min(64));
        while let Some(message) = socket.next().await {
            match message? {
                Message::Text(text) => {
                    let remaining = options.max_messages.saturating_sub(payloads.len());
                    payloads.extend(
                        broadcast_payloads(text.as_str())
                            .map_err(ProtocolError::from)?
                            .into_iter()
                            .take(remaining),
                    );
                    if payloads.len() == options.max_messages {
                        socket.close(None).await?;
                        return Ok(payloads);
                    }
                }
                Message::Close(_) => return Err(StreamError::Closed),
                Message::Binary(_) | Message::Ping(_) | Message::Pong(_) | Message::Frame(_) => {}
            }
        }
        Err(StreamError::Closed)
    }
}

impl From<reqwest::Error> for StreamError {
    fn from(error: reqwest::Error) -> Self {
        Self::Http(error.without_url())
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

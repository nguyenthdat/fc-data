use std::{collections::VecDeque, time::Duration};

use futures_util::{SinkExt as _, StreamExt as _};
use serde_json::Value;
use tokio::net::TcpStream;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream, tungstenite::Message};

use super::{
    error::StreamError,
    protocol::{ProtocolError, ServerEvent, server_events, switch_channels_frame},
};

type Socket = WebSocketStream<MaybeTlsStream<TcpStream>>;

/// An open SSI `SignalR` subscription owned by one async caller.
#[derive(Debug)]
pub struct Subscription {
    socket: Socket,
    pending: VecDeque<ServerEvent>,
    next_invocation_id: u64,
    control_timeout: Duration,
}

impl Subscription {
    pub(super) const fn new(socket: Socket, control_timeout: Duration) -> Self {
        Self {
            socket,
            pending: VecDeque::new(),
            next_invocation_id: 2,
            control_timeout,
        }
    }

    /// Receives the next decoded Broadcast payload or reports a clean remote close.
    pub async fn recv(&mut self) -> Result<Option<Value>, StreamError> {
        loop {
            if let Some(event) = self.pending.pop_front() {
                return match event {
                    ServerEvent::Broadcast(payload) => Ok(Some(payload)),
                    ServerEvent::HubError(error) => Err(StreamError::Hub(error)),
                };
            }
            let Some(message) = self.socket.next().await else {
                return Ok(None);
            };
            match message? {
                Message::Text(text) => {
                    self.pending
                        .extend(server_events(text.as_str()).map_err(ProtocolError::from)?);
                }
                Message::Close(_) => return Ok(None),
                Message::Binary(_) | Message::Ping(_) | Message::Pong(_) | Message::Frame(_) => {}
            }
        }
    }

    /// Switches the active channel on the existing connection.
    pub async fn switch_channel(&mut self, channel: &str) -> Result<(), StreamError> {
        validate_channel(channel)?;
        let invocation_id = self.next_invocation_id;
        self.next_invocation_id = self
            .next_invocation_id
            .checked_add(1)
            .ok_or(StreamError::InvocationOverflow)?;
        let send = self.socket.send(Message::Text(
            switch_channels_frame(channel, invocation_id)
                .to_string()
                .into(),
        ));
        tokio::time::timeout(self.control_timeout, send)
            .await
            .map_err(|_| StreamError::TimedOut(self.control_timeout))??;
        Ok(())
    }

    /// Closes this subscription and consumes its open-state owner.
    pub async fn close(mut self) -> Result<(), StreamError> {
        let close = self.socket.close(None);
        tokio::time::timeout(self.control_timeout, close)
            .await
            .map_err(|_| StreamError::TimedOut(self.control_timeout))??;
        Ok(())
    }
}

pub(super) fn validate_channel(channel: &str) -> Result<(), StreamError> {
    if channel.trim().is_empty() {
        Err(StreamError::InvalidOptions("channel must not be empty"))
    } else {
        Ok(())
    }
}

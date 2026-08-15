use std::time::Duration;

use serde_json::Value;
use thiserror::Error;

use super::{message::StreamDecodeError, protocol::ProtocolError};
use crate::api::ClientError;

/// SSI `SignalR` streaming failure.
#[derive(Debug, Error)]
pub enum StreamError {
    /// Access-token acquisition failed.
    #[error(transparent)]
    Client(#[from] ClientError),
    /// Legacy `SignalR` protocol construction or parsing failed.
    #[error(transparent)]
    Protocol(#[from] ProtocolError),
    /// A typed stream envelope or payload could not be decoded.
    #[error(transparent)]
    Decode(#[from] StreamDecodeError),
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
    /// A bounded stream operation exceeded its deadline.
    #[error("SSI stream operation timed out after {0:?}")]
    TimedOut(Duration),
    /// Stream options were empty, zero, or outside supported bounds.
    #[error("invalid stream option: {0}")]
    InvalidOptions(&'static str),
    /// SSI returned an unexpected streaming start response.
    #[error("SSI streaming start response was not 'started'")]
    UnexpectedStart,
    /// SSI emitted its public hub Error method.
    #[error("SSI streaming hub error: {0}")]
    Hub(Value),
    /// The `SignalR` invocation counter exhausted its numeric range.
    #[error("SSI streaming invocation counter overflowed")]
    InvocationOverflow,
}

impl From<reqwest::Error> for StreamError {
    fn from(error: reqwest::Error) -> Self {
        Self::Http(error.without_url())
    }
}

//! Typed SSI stream envelopes and payloads.

mod models;
mod quote;

use serde::Deserialize;
use serde_json::Value;
use thiserror::Error;

pub use models::{ForeignRoom, MarketIndex, RealtimeBar, SecuritiesStatus, Trade};
pub use quote::Quote;

/// A decoded SSI stream message.
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum StreamMessage {
    /// Securities status payload (`F`).
    SecuritiesStatus(SecuritiesStatus),
    /// Ten-level quote payload (`X-QUOTE`).
    Quote(Quote),
    /// Matched-trade payload (`X-TRADE`).
    Trade(Trade),
    /// Foreign ownership room payload (`R`).
    ForeignRoom(ForeignRoom),
    /// Market-index payload (`MI`).
    Index(MarketIndex),
    /// Realtime OHLCV payload (`B`).
    Bar(RealtimeBar),
    /// A forward-compatible payload whose discriminator is not yet modeled.
    Unknown {
        /// Original wire discriminator.
        data_type: String,
        /// Parsed JSON content.
        content: Value,
    },
}

/// Typed stream envelope or payload decoding failure.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum StreamDecodeError {
    /// The outer `DataType` and `Content` envelope was invalid.
    #[error("invalid SSI stream envelope: {0}")]
    Envelope(serde_json::Error),
    /// String-encoded `Content` was not valid JSON.
    #[error("invalid JSON content for SSI stream data type {data_type}: {source}")]
    Content {
        /// Outer wire discriminator.
        data_type: String,
        /// JSON parser failure.
        #[source]
        source: serde_json::Error,
    },
    /// A known discriminator's payload did not match its capture-backed model.
    #[error("invalid payload for SSI stream data type {data_type}: {source}")]
    Payload {
        /// Original wire discriminator.
        data_type: String,
        /// Typed payload parser failure.
        #[source]
        source: serde_json::Error,
    },
}

#[derive(Deserialize)]
struct Envelope {
    #[serde(
        rename = "DataType",
        alias = "datatype",
        alias = "Datatype",
        alias = "dataType"
    )]
    data_type: String,
    #[serde(rename = "Content", alias = "content")]
    content: Value,
}

impl StreamMessage {
    /// Decodes a raw SSI `DataType` and `Content` envelope.
    pub fn decode(value: Value) -> Result<Self, StreamDecodeError> {
        Self::try_from(value)
    }
}

impl TryFrom<Value> for StreamMessage {
    type Error = StreamDecodeError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        let envelope: Envelope =
            serde_json::from_value(value).map_err(StreamDecodeError::Envelope)?;
        let data_type = envelope.data_type;
        let discriminator = data_type.to_ascii_uppercase();
        let content = match envelope.content {
            Value::String(serialized) => {
                serde_json::from_str(&serialized).map_err(|source| StreamDecodeError::Content {
                    data_type: data_type.clone(),
                    source,
                })?
            }
            structured => structured,
        };

        match discriminator.as_str() {
            "F" => decode_known(content, data_type, Self::SecuritiesStatus),
            "X-QUOTE" | "QUOTE" => decode_known(content, data_type, Self::Quote),
            "X-TRADE" | "TRADE" => decode_known(content, data_type, Self::Trade),
            "R" => decode_known(content, data_type, Self::ForeignRoom),
            "MI" => decode_known(content, data_type, Self::Index),
            "B" => decode_known(content, data_type, Self::Bar),
            _ => Ok(Self::Unknown { data_type, content }),
        }
    }
}

fn decode_known<T>(
    content: Value,
    data_type: String,
    variant: impl FnOnce(T) -> StreamMessage,
) -> Result<StreamMessage, StreamDecodeError>
where
    T: serde::de::DeserializeOwned,
{
    serde_json::from_value(content)
        .map(variant)
        .map_err(|source| StreamDecodeError::Payload { data_type, source })
}

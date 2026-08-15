//! Typed SSI stream channel selectors.

use std::fmt;

use thiserror::Error;

/// A validated channel suffix containing one or more names, or `ALL`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ChannelSelector(String);

/// A fully rendered SSI stream channel.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Channel(String);

/// Typed channel construction failure.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum ChannelError {
    /// A symbol or index selector was empty.
    #[error("channel selectors must not be empty")]
    Empty,
}

impl ChannelSelector {
    /// Selects every symbol or index supported by the channel.
    pub fn all() -> Self {
        Self("ALL".to_owned())
    }

    /// Joins one or more symbols or indexes with SSI's `-` separator.
    pub fn symbols<I, S>(values: I) -> Result<Self, ChannelError>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let values = values
            .into_iter()
            .map(|value| value.as_ref().trim().to_owned())
            .collect::<Vec<_>>();
        if values.is_empty() || values.iter().any(String::is_empty) {
            return Err(ChannelError::Empty);
        }
        Ok(Self(values.join("-")))
    }
}

impl Channel {
    /// Builds an `F` securities-status channel.
    pub fn securities_status(selector: &ChannelSelector) -> Self {
        Self::typed("F", selector)
    }

    /// Builds an `X-QUOTE` order-book channel.
    pub fn quote(selector: &ChannelSelector) -> Self {
        Self::typed("X-QUOTE", selector)
    }

    /// Builds an `X-TRADE` matched-trade channel.
    pub fn trade(selector: &ChannelSelector) -> Self {
        Self::typed("X-TRADE", selector)
    }

    /// Builds an `R` foreign-room channel.
    pub fn foreign_room(selector: &ChannelSelector) -> Self {
        Self::typed("R", selector)
    }

    /// Builds an `MI` market-index channel.
    pub fn index(selector: &ChannelSelector) -> Self {
        Self::typed("MI", selector)
    }

    /// Builds a `B` realtime-bar channel.
    pub fn bar(selector: &ChannelSelector) -> Self {
        Self::typed("B", selector)
    }

    /// Preserves an arbitrary channel for protocol extensions.
    pub fn raw(channel: impl Into<String>) -> Result<Self, ChannelError> {
        let channel = channel.into();
        if channel.trim().is_empty() {
            return Err(ChannelError::Empty);
        }
        Ok(Self(channel))
    }

    /// Returns the rendered channel accepted by `SwitchChannels`.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    fn typed(prefix: &str, selector: &ChannelSelector) -> Self {
        Self(format!("{prefix}:{}", selector.0))
    }
}

impl AsRef<str> for Channel {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for Channel {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

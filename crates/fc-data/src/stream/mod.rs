//! SSI-specific `SignalR` 1.3 streaming client.

mod channel;
mod client;
mod error;
mod message;
mod protocol;
mod reconnect;
mod session;

pub use channel::{Channel, ChannelError, ChannelSelector};
pub use client::{StreamClient, StreamOptions};
pub use error::StreamError;
pub use message::{
    ForeignRoom, MarketIndex, Quote, RealtimeBar, SecuritiesStatus, StreamDecodeError,
    StreamMessage, Trade,
};
pub use protocol::{broadcast_payloads, switch_channels_frame};
pub use reconnect::{ReconnectOptions, ReconnectPolicy, ResilientSubscription};
pub use session::Subscription;

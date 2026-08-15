//! SSI-specific `SignalR` 1.3 streaming client.

mod client;
mod error;
mod protocol;
mod session;

pub use client::{StreamClient, StreamOptions};
pub use error::StreamError;
pub use protocol::{broadcast_payloads, switch_channels_frame};
pub use session::Subscription;

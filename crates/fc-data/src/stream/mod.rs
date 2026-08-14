//! SSI-specific legacy ASP.NET `SignalR` streaming client.

mod client;
mod protocol;

pub use client::{LegacyStreamClient, StreamError, StreamOptions};
pub use protocol::{broadcast_payloads, switch_channels_frame};

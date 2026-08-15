//! Opt-in typed stream reconnection.

use std::time::Duration;

use super::{Channel, StreamClient, StreamError, StreamMessage, Subscription};

const DEFAULT_RETRIES: usize = 1;
const DEFAULT_DELAY: Duration = Duration::from_secs(3);

/// Bounded retry behavior applied after a streaming transport disconnect.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReconnectPolicy {
    max_retries: usize,
    delay: Duration,
}

/// Typed resilient-subscription settings.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReconnectOptions {
    handshake_timeout: Duration,
    policy: ReconnectPolicy,
}

/// An opt-in typed subscription that restores its latest sent channel.
#[derive(Debug)]
pub struct ResilientSubscription<'a> {
    stream: StreamClient<'a>,
    active: Subscription,
    channel: Channel,
    options: ReconnectOptions,
}

impl ReconnectPolicy {
    /// Creates a finite retry policy with an async delay before each attempt.
    pub const fn new(max_retries: usize, delay: Duration) -> Self {
        Self { max_retries, delay }
    }

    /// Returns the maximum reconnect attempts for one receive operation.
    pub const fn max_retries(self) -> usize {
        self.max_retries
    }

    /// Returns the async delay before each reconnect attempt.
    pub const fn delay(self) -> Duration {
        self.delay
    }
}

impl Default for ReconnectPolicy {
    fn default() -> Self {
        Self::new(DEFAULT_RETRIES, DEFAULT_DELAY)
    }
}

impl ReconnectOptions {
    /// Creates reconnect options using the default one-retry policy.
    pub fn new(handshake_timeout: Duration) -> Result<Self, StreamError> {
        if handshake_timeout.is_zero() {
            return Err(StreamError::InvalidOptions(
                "handshake timeout must be greater than zero",
            ));
        }
        Ok(Self {
            handshake_timeout,
            policy: ReconnectPolicy::default(),
        })
    }

    /// Replaces the reconnect policy.
    #[must_use]
    pub const fn with_policy(mut self, policy: ReconnectPolicy) -> Self {
        self.policy = policy;
        self
    }

    /// Returns the timeout for each full reconnect handshake.
    pub const fn handshake_timeout(self) -> Duration {
        self.handshake_timeout
    }

    /// Returns the bounded reconnect policy.
    pub const fn policy(self) -> ReconnectPolicy {
        self.policy
    }
}

impl<'a> StreamClient<'a> {
    /// Opens an opt-in typed subscription with bounded reconnect behavior.
    pub async fn subscribe_resilient_typed(
        &self,
        initial_channel: &Channel,
        options: ReconnectOptions,
    ) -> Result<ResilientSubscription<'a>, StreamError> {
        let active = self
            .subscribe_typed(initial_channel, options.handshake_timeout)
            .await?;
        Ok(ResilientSubscription {
            stream: *self,
            active,
            channel: initial_channel.clone(),
            options,
        })
    }
}

impl ResilientSubscription<'_> {
    /// Receives one typed message, reconnecting only after transport loss.
    pub async fn recv_typed(&mut self) -> Result<StreamMessage, StreamError> {
        let mut retries = 0;
        loop {
            let disconnect = match self.active.recv_typed().await {
                Ok(Some(message)) => return Ok(message),
                Ok(None) => StreamError::Closed,
                Err(StreamError::WebSocket(error)) => StreamError::WebSocket(error),
                Err(error) => return Err(error),
            };
            if retries == self.options.policy.max_retries {
                return Err(disconnect);
            }

            loop {
                retries += 1;
                tokio::time::sleep(self.options.policy.delay).await;
                match self
                    .stream
                    .subscribe_typed(&self.channel, self.options.handshake_timeout)
                    .await
                {
                    Ok(subscription) => {
                        self.active = subscription;
                        break;
                    }
                    Err(error) if retries == self.options.policy.max_retries => {
                        return Err(error);
                    }
                    Err(_) => {}
                }
            }
        }
    }

    /// Switches the active connection and records the channel after a successful send.
    pub async fn switch_typed(&mut self, channel: &Channel) -> Result<(), StreamError> {
        self.active.switch_typed(channel).await?;
        self.channel = channel.clone();
        Ok(())
    }

    /// Closes the active connection without starting a reconnect attempt.
    pub async fn close(self) -> Result<(), StreamError> {
        self.active.close().await
    }
}

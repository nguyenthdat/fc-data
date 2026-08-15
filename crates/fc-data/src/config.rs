//! Environment-backed SSI configuration.

use std::{env, fmt, io::ErrorKind};

use secrecy::SecretString;
use thiserror::Error;
use url::Url;

const DEFAULT_API_URL: &str = "https://fc-data.ssi.com.vn/";
const DEFAULT_STREAM_URL: &str = "https://fc-datahub.ssi.com.vn/";
const CONSUMER_ID_ENV: &str = "SSI_FCDATA_CONSUMER_ID";
const CONSUMER_SECRET_ENV: &str = "SSI_FCDATA_CONSUMER_SECRET";
const API_URL_ENV: &str = "SSI_FCDATA_API_URL";
const STREAM_URL_ENV: &str = "SSI_FCDATA_STREAM_URL";

/// Borrowed settings input parsed at the environment boundary.
#[derive(Clone, Copy)]
pub struct SettingsInput<'a> {
    /// SSI Consumer ID.
    pub consumer_id: &'a str,
    /// SSI Consumer Secret.
    pub consumer_secret: &'a str,
    /// REST API base URL.
    pub api_url: &'a str,
    /// Streaming Data Hub base URL.
    pub stream_url: &'a str,
}

/// Validated runtime settings.
#[derive(Debug)]
pub struct Settings {
    consumer_id: SecretString,
    consumer_secret: SecretString,
    api_url: Url,
    stream_url: Url,
}

/// Transport policy applied while parsing endpoint URLs.
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub enum TransportPolicy {
    /// Require encrypted HTTPS or WSS endpoints.
    SecureOnly,
    /// Explicitly allow HTTP or WS for isolated local test servers.
    AllowInsecure,
}

/// Configuration parsing failure.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum ConfigError {
    /// A required setting was empty.
    #[error("missing required setting {0}")]
    Missing(&'static str),
    /// A setting was not valid UTF-8.
    #[error("setting {name} is not valid UTF-8")]
    NonUnicode {
        /// Environment variable name.
        name: &'static str,
    },
    /// A URL setting could not be parsed.
    #[error("setting {name} is not a valid URL: {source}")]
    InvalidUrl {
        /// Environment variable name.
        name: &'static str,
        /// URL parser error.
        source: url::ParseError,
    },
    /// A URL used a scheme that is invalid for its endpoint.
    #[error("setting {name} uses unsupported URL scheme {scheme}")]
    UnsupportedScheme {
        /// Environment variable name.
        name: &'static str,
        /// Parsed URL scheme.
        scheme: String,
    },
    /// A dotenv file was present but invalid or unreadable.
    #[error("failed to load dotenv configuration; source content was redacted")]
    Dotenv,
}

impl Settings {
    /// Parses validated settings from explicit values.
    pub fn from_input(input: SettingsInput<'_>) -> Result<Self, ConfigError> {
        Self::from_input_with_policy(input, TransportPolicy::SecureOnly)
    }

    /// Parses settings with an explicit transport policy.
    pub fn from_input_with_policy(
        input: SettingsInput<'_>,
        policy: TransportPolicy,
    ) -> Result<Self, ConfigError> {
        let consumer_id = parse_secret(input.consumer_id, CONSUMER_ID_ENV)?;
        let consumer_secret = parse_secret(input.consumer_secret, CONSUMER_SECRET_ENV)?;
        let (api_schemes, stream_schemes): (&[&str], &[&str]) = match policy {
            TransportPolicy::SecureOnly => (&["https"], &["https", "wss"]),
            TransportPolicy::AllowInsecure => (&["http", "https"], &["http", "https", "ws", "wss"]),
        };
        let api_url = parse_url(input.api_url, API_URL_ENV, api_schemes)?;
        let stream_url = parse_url(input.stream_url, STREAM_URL_ENV, stream_schemes)?;

        Ok(Self {
            consumer_id,
            consumer_secret,
            api_url,
            stream_url,
        })
    }

    /// Loads settings from `.env` and the process environment.
    pub fn load() -> Result<Self, ConfigError> {
        match dotenvy::dotenv() {
            Ok(_) => {}
            Err(dotenvy::Error::Io(error)) if error.kind() == ErrorKind::NotFound => {}
            Err(_) => return Err(ConfigError::Dotenv),
        }

        let consumer_id = required_env(CONSUMER_ID_ENV)?;
        let consumer_secret = required_env(CONSUMER_SECRET_ENV)?;
        let api_url = optional_env(API_URL_ENV, DEFAULT_API_URL)?;
        let stream_url = optional_env(STREAM_URL_ENV, DEFAULT_STREAM_URL)?;

        Self::from_input(SettingsInput {
            consumer_id: &consumer_id,
            consumer_secret: &consumer_secret,
            api_url: &api_url,
            stream_url: &stream_url,
        })
    }

    /// Returns the validated REST API base URL.
    pub const fn api_url(&self) -> &Url {
        &self.api_url
    }

    /// Returns the validated streaming Data Hub base URL.
    pub const fn stream_url(&self) -> &Url {
        &self.stream_url
    }

    pub(crate) const fn consumer_id(&self) -> &SecretString {
        &self.consumer_id
    }

    pub(crate) const fn consumer_secret(&self) -> &SecretString {
        &self.consumer_secret
    }
}

impl fmt::Debug for SettingsInput<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("SettingsInput")
            .field("consumer_id", &"[REDACTED]")
            .field("consumer_secret", &"[REDACTED]")
            .field("api_url", &self.api_url)
            .field("stream_url", &self.stream_url)
            .finish()
    }
}

fn parse_secret(value: &str, name: &'static str) -> Result<SecretString, ConfigError> {
    let value = value.trim();
    if value.is_empty() {
        return Err(ConfigError::Missing(name));
    }
    Ok(SecretString::from(value.to_owned()))
}

fn parse_url(value: &str, name: &'static str, schemes: &[&str]) -> Result<Url, ConfigError> {
    let url = Url::parse(value).map_err(|source| ConfigError::InvalidUrl { name, source })?;
    if !schemes.contains(&url.scheme()) {
        return Err(ConfigError::UnsupportedScheme {
            name,
            scheme: url.scheme().to_owned(),
        });
    }
    Ok(url)
}

fn required_env(name: &'static str) -> Result<String, ConfigError> {
    match env::var(name) {
        Ok(value) => Ok(value),
        Err(env::VarError::NotPresent) => Err(ConfigError::Missing(name)),
        Err(env::VarError::NotUnicode(_)) => Err(ConfigError::NonUnicode { name }),
    }
}

fn optional_env(name: &'static str, default: &str) -> Result<String, ConfigError> {
    match env::var(name) {
        Ok(value) => Ok(value),
        Err(env::VarError::NotPresent) => Ok(default.to_owned()),
        Err(env::VarError::NotUnicode(_)) => Err(ConfigError::NonUnicode { name }),
    }
}

//! Authenticated SSI Market Data HTTP client.

use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use reqwest::header::{ACCEPT, HeaderMap, HeaderValue};
use secrecy::ExposeSecret as _;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use thiserror::Error;
use tokio::sync::Mutex;

use super::{AccessToken, ApiRequest, RequestError};
use crate::config::Settings;

const ACCESS_TOKEN_PATH: &str = "api/v2/Market/AccessToken";
const TOKEN_REFRESH_SKEW: Duration = Duration::from_mins(1);
const MAX_TOKEN_TTL: Duration = Duration::from_hours(8);
const FALLBACK_TOKEN_TTL: Duration = Duration::from_mins(475);

/// SSI Market Data client with validated settings and bounded HTTP timeouts.
#[derive(Debug)]
pub struct MarketDataClient {
    http: reqwest::Client,
    settings: Settings,
    token: Mutex<Option<CachedAccessToken>>,
}

#[derive(Debug)]
struct CachedAccessToken {
    token: AccessToken,
    refresh_at: Instant,
}

/// SSI client request failure.
#[derive(Debug, Error)]
pub enum ClientError {
    /// The HTTP client could not be constructed.
    #[error("failed to build HTTP client: {0}")]
    Build(#[source] reqwest::Error),
    /// An SSI endpoint URL or query could not be built.
    #[error(transparent)]
    Request(#[from] RequestError),
    /// An HTTP request or response failed.
    #[error("SSI HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),
    /// SSI rejected the configured credentials.
    #[error("SSI authentication failed with status {status}: {message}")]
    Authentication {
        /// Numeric SSI response status.
        status: u16,
        /// Non-secret SSI error message.
        message: String,
    },
    /// SSI returned a successful authentication envelope without a token.
    #[error("SSI authentication response did not contain data.accessToken")]
    MissingAccessToken,
    /// The configured API URL could not join the token endpoint.
    #[error("failed to join SSI access-token endpoint: {0}")]
    AccessTokenUrl(#[from] url::ParseError),
}

#[derive(Serialize)]
struct AuthRequest<'a> {
    #[serde(rename = "consumerID")]
    consumer_id: &'a str,
    #[serde(rename = "consumerSecret")]
    consumer_secret: &'a str,
}

#[derive(Deserialize)]
struct AuthResponse {
    status: u16,
    message: String,
    data: Option<AuthData>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct AuthData {
    access_token: String,
}

#[derive(Deserialize)]
struct JwtClaims {
    exp: u64,
}

impl MarketDataClient {
    /// Creates a client with explicit connection and request timeouts.
    pub fn new(settings: Settings) -> Result<Self, ClientError> {
        let mut headers = HeaderMap::new();
        headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
        let http = reqwest::Client::builder()
            .connect_timeout(Duration::from_secs(10))
            .timeout(Duration::from_secs(30))
            .user_agent(concat!(
                env!("CARGO_PKG_NAME"),
                "/",
                env!("CARGO_PKG_VERSION")
            ))
            .default_headers(headers)
            .build()
            .map_err(ClientError::Build)?;

        Ok(Self {
            http,
            settings,
            token: Mutex::new(None),
        })
    }

    /// Confirms that the configured credentials can obtain an SSI access token.
    pub async fn authenticate(&self) -> Result<(), ClientError> {
        self.access_token().await.map(drop)
    }

    /// Executes an authenticated SSI Market Data query and preserves its JSON envelope.
    pub async fn execute(&self, request: &ApiRequest) -> Result<Value, ClientError> {
        let token = self.access_token().await?;
        let url = request.url(self.settings.api_url())?;
        let response = self
            .http
            .get(url)
            .bearer_auth(token.expose())
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;
        Ok(response)
    }

    pub(crate) async fn access_token(&self) -> Result<AccessToken, ClientError> {
        let mut cached = self.token.lock().await;
        if let Some(token) = cached
            .as_ref()
            .filter(|token| Instant::now() < token.refresh_at)
        {
            return Ok(token.token.clone());
        }

        let token = self.request_access_token().await?;
        *cached = Some(CachedAccessToken {
            refresh_at: token_refresh_at(token.expose()),
            token: token.clone(),
        });
        drop(cached);
        Ok(token)
    }

    async fn request_access_token(&self) -> Result<AccessToken, ClientError> {
        let url = self.settings.api_url().join(ACCESS_TOKEN_PATH)?;
        let response: AuthResponse = self
            .http
            .post(url)
            .json(&AuthRequest {
                consumer_id: self.settings.consumer_id().expose_secret(),
                consumer_secret: self.settings.consumer_secret().expose_secret(),
            })
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        if response.status != 200 {
            return Err(ClientError::Authentication {
                status: response.status,
                message: response.message,
            });
        }

        let data = response.data.ok_or(ClientError::MissingAccessToken)?;
        Ok(AccessToken::new(data.access_token))
    }

    pub(crate) const fn http(&self) -> &reqwest::Client {
        &self.http
    }

    pub(crate) const fn settings(&self) -> &Settings {
        &self.settings
    }
}

fn token_refresh_at(token: &str) -> Instant {
    let ttl = token
        .split('.')
        .nth(1)
        .and_then(|payload| URL_SAFE_NO_PAD.decode(payload).ok())
        .and_then(|payload| serde_json::from_slice::<JwtClaims>(&payload).ok())
        .and_then(|claims| {
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .ok()
                .map(|now| claims.exp.saturating_sub(now.as_secs()))
        })
        .map_or(FALLBACK_TOKEN_TTL, Duration::from_secs)
        .min(MAX_TOKEN_TTL)
        .saturating_sub(TOKEN_REFRESH_SKEW);
    Instant::now() + ttl
}

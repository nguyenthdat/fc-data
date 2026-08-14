//! Legacy `SignalR` 1.3 query and frame protocol.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use thiserror::Error;
use url::Url;

pub(super) const CLIENT_PROTOCOL: &str = "1.3";
pub(super) const HUB_NAME: &str = "fcmarketdatav2hub";
const SIGNALR_PATH: &str = "v2.0/signalr/";

/// Legacy `SignalR` protocol encoding failure.
#[derive(Debug, Error)]
pub enum ProtocolError {
    /// A `SignalR` endpoint URL was invalid.
    #[error("failed to build SignalR endpoint: {0}")]
    Url(#[from] url::ParseError),
    /// A `SignalR` JSON frame was invalid.
    #[error("invalid SignalR JSON frame: {0}")]
    Json(#[from] serde_json::Error),
    /// The streaming base URL used an unsupported scheme.
    #[error("unsupported streaming URL scheme {0}")]
    UnsupportedScheme(String),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(super) struct NegotiateResponse {
    pub(super) connection_token: String,
    pub(super) protocol_version: String,
    pub(super) try_web_sockets: bool,
}

#[derive(Serialize)]
struct HubDefinition<'a> {
    name: &'a str,
}

#[derive(Deserialize)]
struct ServerFrame {
    #[serde(rename = "M", default)]
    messages: Vec<HubMessage>,
}

#[derive(Deserialize)]
struct HubMessage {
    #[serde(rename = "H")]
    hub: String,
    #[serde(rename = "M")]
    method: String,
    #[serde(rename = "A", default)]
    arguments: Vec<Value>,
}

/// Builds the legacy `SignalR` `SwitchChannels` invocation frame.
pub fn switch_channels_frame(channel: &str, invocation_id: u64) -> Value {
    serde_json::json!({
        "H": HUB_NAME,
        "M": "SwitchChannels",
        "A": [channel],
        "I": invocation_id,
    })
}

/// Extracts FC Market Data broadcast arguments from a `SignalR` text frame.
pub fn broadcast_payloads(frame: &str) -> Result<Vec<Value>, serde_json::Error> {
    let frame: ServerFrame = serde_json::from_str(frame)?;
    Ok(frame
        .messages
        .into_iter()
        .filter(|message| {
            message.hub.eq_ignore_ascii_case(HUB_NAME)
                && message.method.eq_ignore_ascii_case("Broadcast")
        })
        .flat_map(|message| message.arguments)
        .collect())
}

pub(super) fn connection_data() -> Result<String, ProtocolError> {
    Ok(serde_json::to_string(&[HubDefinition { name: HUB_NAME }])?)
}

pub(super) fn negotiate_url(base: &Url, data: &str) -> Result<Url, ProtocolError> {
    let mut url = action_url(base, "negotiate")?;
    url.query_pairs_mut()
        .append_pair("connectionData", data)
        .append_pair("clientProtocol", CLIENT_PROTOCOL);
    Ok(url)
}

pub(super) fn connect_url(
    base: &Url,
    data: &str,
    connection_token: &str,
) -> Result<Url, ProtocolError> {
    let mut url = action_url(base, "connect")?;
    let scheme = match url.scheme() {
        "http" | "ws" => "ws",
        "https" | "wss" => "wss",
        other => return Err(ProtocolError::UnsupportedScheme(other.to_owned())),
    };
    url.set_scheme(scheme)
        .map_err(|()| ProtocolError::UnsupportedScheme(scheme.to_owned()))?;
    url.query_pairs_mut()
        .append_pair("clientProtocol", CLIENT_PROTOCOL)
        .append_pair("transport", "webSockets")
        .append_pair("connectionToken", connection_token)
        .append_pair("connectionData", data)
        .append_pair("tid", "10");
    Ok(url)
}

pub(super) fn start_url(
    base: &Url,
    data: &str,
    connection_token: &str,
) -> Result<Url, ProtocolError> {
    let mut url = action_url(base, "start")?;
    match url.scheme() {
        "ws" => {
            url.set_scheme("http")
                .map_err(|()| ProtocolError::UnsupportedScheme("ws".to_owned()))?;
        }
        "wss" => {
            url.set_scheme("https")
                .map_err(|()| ProtocolError::UnsupportedScheme("wss".to_owned()))?;
        }
        "http" | "https" => {}
        other => return Err(ProtocolError::UnsupportedScheme(other.to_owned())),
    }
    url.query_pairs_mut()
        .append_pair("clientProtocol", CLIENT_PROTOCOL)
        .append_pair("transport", "webSockets")
        .append_pair("connectionData", data)
        .append_pair("connectionToken", connection_token);
    Ok(url)
}

fn action_url(base: &Url, action: &str) -> Result<Url, ProtocolError> {
    Ok(base.join(SIGNALR_PATH)?.join(action)?)
}

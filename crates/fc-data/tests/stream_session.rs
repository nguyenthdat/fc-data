#![doc = "End-to-end `SignalR` session contracts."]

use std::{collections::HashMap, error::Error, io, time::Duration};

use axum::{
    Json, Router,
    extract::{
        Query,
        ws::{Message, WebSocket, WebSocketUpgrade},
    },
    http::{HeaderMap, StatusCode, header::AUTHORIZATION},
    response::{IntoResponse as _, Response},
    routing::{get, post},
};
use futures_util::{SinkExt as _, StreamExt as _};
use serde_json::{Value, json};
use ssi_fc_data::{
    api::MarketDataClient,
    config::{Settings, SettingsInput, TransportPolicy},
    stream::{StreamClient, StreamError, StreamOptions},
};
use tokio::{net::TcpListener, sync::oneshot, task::JoinHandle};

const ACCESS_TOKEN: &str = "e30.eyJleHAiOjQxMDI0NDQ4MDB9.signature";
type TestResult<T = ()> = Result<T, Box<dyn Error + Send + Sync>>;

struct TestServer {
    base_url: String,
    shutdown: oneshot::Sender<()>,
    task: JoinHandle<io::Result<()>>,
}

impl TestServer {
    async fn start() -> TestResult<Self> {
        let app = Router::new()
            .route("/api/v2/Market/AccessToken", post(access_token))
            .route("/v2.0/signalr/negotiate", get(negotiate))
            .route("/v2.0/signalr/connect", get(connect))
            .route("/v2.0/signalr/start", get(start));
        let listener = TcpListener::bind("127.0.0.1:0").await?;
        let address = listener.local_addr()?;
        let (shutdown, receiver) = oneshot::channel();
        let task = tokio::spawn(async move {
            axum::serve(listener, app)
                .with_graceful_shutdown(async {
                    let _ = receiver.await;
                })
                .await
        });
        Ok(Self {
            base_url: format!("http://{address}/"),
            shutdown,
            task,
        })
    }

    async fn stop(self) -> TestResult {
        let _ = self.shutdown.send(());
        self.task.await??;
        Ok(())
    }

    fn client(&self) -> TestResult<MarketDataClient> {
        let settings = Settings::from_input_with_policy(
            SettingsInput {
                consumer_id: "test-consumer",
                consumer_secret: "test-secret",
                api_url: &self.base_url,
                stream_url: &self.base_url,
            },
            TransportPolicy::AllowInsecure,
        )?;
        Ok(MarketDataClient::new(settings)?)
    }
}

async fn access_token() -> Json<Value> {
    Json(json!({
        "status": 200,
        "message": "Success",
        "data": {"accessToken": ACCESS_TOKEN}
    }))
}

async fn negotiate(Query(query): Query<HashMap<String, String>>) -> Response {
    if query.get("clientProtocol").map(String::as_str) != Some("1.3") {
        return StatusCode::BAD_REQUEST.into_response();
    }
    Json(json!({
        "ConnectionToken": "test-connection-token",
        "ProtocolVersion": "1.3",
        "TryWebSockets": true
    }))
    .into_response()
}

async fn start(Query(query): Query<HashMap<String, String>>) -> Response {
    if query.get("connectionToken").map(String::as_str) != Some("test-connection-token") {
        return StatusCode::BAD_REQUEST.into_response();
    }
    Json(json!({"Response": "started"})).into_response()
}

async fn connect(ws: WebSocketUpgrade, headers: HeaderMap) -> Response {
    let expected = format!("Bearer {ACCESS_TOKEN}");
    if headers
        .get(AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        != Some(&expected)
    {
        return StatusCode::UNAUTHORIZED.into_response();
    }
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    let mut expected_invocation_id = 1_u64;
    while let Some(message) = socket.next().await {
        let Ok(Message::Text(text)) = message else {
            continue;
        };
        let Ok(frame) = serde_json::from_str::<Value>(&text) else {
            break;
        };
        let id = frame.get("I").and_then(Value::as_u64);
        let channel = frame
            .get("A")
            .and_then(Value::as_array)
            .and_then(|arguments| arguments.first())
            .and_then(Value::as_str);
        if id != Some(expected_invocation_id) {
            send_hub_error(&mut socket, "unexpected invocation id").await;
            continue;
        }
        expected_invocation_id += 1;
        let Some(channel) = channel else {
            send_hub_error(&mut socket, "missing channel").await;
            continue;
        };
        if channel == "REMOTE-CLOSE" {
            let _ = socket.close().await;
            return;
        }
        let payloads = if channel == "MULTI" {
            vec![json!({"channel": "MULTI-1"}), json!({"channel": "MULTI-2"})]
        } else {
            vec![json!({"channel": channel})]
        };
        let arguments = payloads
            .into_iter()
            .map(|payload| Value::String(payload.to_string()))
            .collect::<Vec<_>>();
        let frame = json!({
            "M": [{"H": "FcMarketDataV2Hub", "M": "Broadcast", "A": arguments}]
        });
        if socket
            .send(Message::Text(frame.to_string().into()))
            .await
            .is_err()
        {
            return;
        }
        if expected_invocation_id == 3 {
            send_hub_error(&mut socket, "channel denied").await;
        }
    }
}

async fn send_hub_error(socket: &mut WebSocket, message: &str) {
    let frame = json!({
        "M": [{"H": "FcMarketDataV2Hub", "M": "Error", "A": [message]}]
    });
    let _ = socket.send(Message::Text(frame.to_string().into())).await;
}

#[tokio::test]
async fn receives_switches_and_closes_on_one_subscription() -> TestResult {
    // Given
    let server = TestServer::start().await?;
    let client = server.client()?;
    let stream = StreamClient::new(&client);
    let mut subscription = stream.subscribe("MI:VN30", Duration::from_secs(5)).await?;

    // When
    let first = subscription.recv().await?;
    subscription.switch_channel("X-QUOTE:SSI").await?;
    let second = subscription.recv().await?;
    let hub_error = match subscription.recv().await {
        Err(error) => error,
        Ok(value) => {
            return Err(io::Error::other(format!("expected hub error, got {value:?}")).into());
        }
    };
    subscription.close().await?;

    // Then
    assert_eq!(first, Some(json!({"channel": "MI:VN30"})));
    assert_eq!(second, Some(json!({"channel": "X-QUOTE:SSI"})));
    assert!(matches!(hub_error, StreamError::Hub(_)));
    server.stop().await?;
    Ok(())
}

#[tokio::test]
async fn reports_clean_remote_close() -> TestResult {
    // Given
    let server = TestServer::start().await?;
    let client = server.client()?;
    let stream = StreamClient::new(&client);
    let mut subscription = stream
        .subscribe("REMOTE-CLOSE", Duration::from_secs(5))
        .await?;

    // When
    let result = subscription.recv().await?;

    // Then
    assert_eq!(result, None);
    server.stop().await?;
    Ok(())
}

#[tokio::test]
async fn bounded_collect_keeps_exact_message_count() -> TestResult {
    // Given
    let server = TestServer::start().await?;
    let client = server.client()?;
    let options = StreamOptions::new("MULTI".to_owned(), 1, Duration::from_secs(5))?;

    // When
    let payloads = StreamClient::new(&client).collect(&options).await?;

    // Then
    assert_eq!(payloads, vec![json!({"channel": "MULTI-1"})]);
    server.stop().await?;
    Ok(())
}

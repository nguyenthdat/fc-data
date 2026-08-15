#![doc = "Deterministic resilient SSI stream integration tests."]

use std::{
    error::Error,
    io,
    sync::{
        Arc, Mutex,
        atomic::{AtomicUsize, Ordering},
    },
    time::Duration,
};

use axum::{
    Json, Router,
    extract::{
        State,
        ws::{Message, WebSocket, WebSocketUpgrade},
    },
    http::StatusCode,
    response::{IntoResponse as _, Response},
    routing::{get, post},
};
use futures_util::{SinkExt as _, StreamExt as _};
use serde_json::{Value, json};
use ssi_fc_data::{
    api::MarketDataClient,
    config::{Settings, SettingsInput, TransportPolicy},
    stream::{
        Channel, ReconnectOptions, ReconnectPolicy, ResilientSubscription, StreamClient,
        StreamError, StreamMessage,
    },
};
use tokio::{net::TcpListener, sync::oneshot, task::JoinHandle};

const ACCESS_TOKEN: &str = "e30.eyJleHAiOjF9.signature";
type TestResult<T = ()> = Result<T, Box<dyn Error + Send + Sync>>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Scenario {
    CleanClose,
    DropThenFail,
}

#[derive(Debug)]
struct ServerState {
    scenario: Scenario,
    auth_requests: AtomicUsize,
    negotiate_requests: AtomicUsize,
    start_requests: AtomicUsize,
    connections: AtomicUsize,
    invocations: Mutex<Vec<(usize, u64, String)>>,
}

struct TestServer {
    base_url: String,
    state: Arc<ServerState>,
    shutdown: oneshot::Sender<()>,
    task: JoinHandle<io::Result<()>>,
}

impl TestServer {
    async fn start(scenario: Scenario) -> TestResult<Self> {
        let state = Arc::new(ServerState {
            scenario,
            auth_requests: AtomicUsize::new(0),
            negotiate_requests: AtomicUsize::new(0),
            start_requests: AtomicUsize::new(0),
            connections: AtomicUsize::new(0),
            invocations: Mutex::new(Vec::new()),
        });
        let app = Router::new()
            .route("/api/v2/Market/AccessToken", post(access_token))
            .route("/v2.0/signalr/negotiate", get(negotiate))
            .route("/v2.0/signalr/connect", get(connect))
            .route("/v2.0/signalr/start", get(start))
            .with_state(Arc::clone(&state));
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
            state,
            shutdown,
            task,
        })
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

    async fn stop(self) -> TestResult {
        let _ = self.shutdown.send(());
        self.task.await??;
        Ok(())
    }
}

async fn access_token(State(state): State<Arc<ServerState>>) -> Json<Value> {
    state.auth_requests.fetch_add(1, Ordering::SeqCst);
    Json(json!({
        "status": 200,
        "message": "Success",
        "data": {"accessToken": ACCESS_TOKEN}
    }))
}

async fn negotiate(State(state): State<Arc<ServerState>>) -> Response {
    let request = state.negotiate_requests.fetch_add(1, Ordering::SeqCst) + 1;
    if state.scenario == Scenario::DropThenFail && request > 2 {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    }
    Json(json!({
        "ConnectionToken": format!("connection-{request}"),
        "ProtocolVersion": "1.3",
        "TryWebSockets": true
    }))
    .into_response()
}

async fn start(State(state): State<Arc<ServerState>>) -> Json<Value> {
    state.start_requests.fetch_add(1, Ordering::SeqCst);
    Json(json!({"Response": "started"}))
}

async fn connect(State(state): State<Arc<ServerState>>, ws: WebSocketUpgrade) -> Response {
    let connection = state.connections.fetch_add(1, Ordering::SeqCst) + 1;
    ws.on_upgrade(move |socket| handle_socket(socket, state, connection))
}

async fn handle_socket(mut socket: WebSocket, state: Arc<ServerState>, connection: usize) {
    while let Some(Ok(Message::Text(text))) = socket.next().await {
        let Some((id, channel)) = parse_invocation(&text) else {
            return;
        };
        if let Ok(mut invocations) = state.invocations.lock() {
            invocations.push((connection, id, channel.clone()));
        }
        match state.scenario {
            Scenario::CleanClose if connection == 1 && id == 2 => {
                let _ = socket.close().await;
                return;
            }
            Scenario::DropThenFail if connection == 1 => return,
            Scenario::CleanClose | Scenario::DropThenFail => {
                if send_message(&mut socket, &channel).await.is_err()
                    || state.scenario == Scenario::DropThenFail
                {
                    return;
                }
            }
        }
    }
}

fn parse_invocation(text: &str) -> Option<(u64, String)> {
    let frame = serde_json::from_str::<Value>(text).ok()?;
    Some((
        frame.get("I")?.as_u64()?,
        frame.get("A")?.as_array()?.first()?.as_str()?.to_owned(),
    ))
}

async fn send_message(socket: &mut WebSocket, channel: &str) -> Result<(), axum::Error> {
    let payload = json!({"DataType": "TEST", "Content": {"channel": channel}});
    let frame = json!({
        "M": [{"H": "FcMarketDataV2Hub", "M": "Broadcast", "A": [payload.to_string()]}]
    });
    socket.send(Message::Text(frame.to_string().into())).await
}

async fn subscribe<'a>(
    client: &'a MarketDataClient,
    channel: &Channel,
) -> TestResult<ResilientSubscription<'a>> {
    let options = ReconnectOptions::new(Duration::from_secs(1))?
        .with_policy(ReconnectPolicy::new(1, Duration::ZERO));
    Ok(StreamClient::new(client)
        .subscribe_resilient_typed(channel, options)
        .await?)
}

fn has_channel(message: &StreamMessage, channel: &Channel) -> bool {
    matches!(message, StreamMessage::Unknown { content, .. }
        if content.get("channel").and_then(Value::as_str) == Some(channel.as_str()))
}

#[test]
fn reconnect_policy_defaults_to_one_retry_after_three_seconds() {
    // Given / When
    let policy = ReconnectPolicy::default();

    // Then
    assert_eq!(policy.max_retries(), 1);
    assert_eq!(policy.delay(), Duration::from_secs(3));
}

#[tokio::test]
async fn clean_close_restores_latest_channel_and_explicit_close_stops() -> TestResult {
    // Given
    let server = TestServer::start(Scenario::CleanClose).await?;
    let state = Arc::clone(&server.state);
    let client = server.client()?;
    let initial = Channel::raw("TEST:INITIAL")?;
    let latest = Channel::raw("TEST:LATEST")?;
    let mut subscription = subscribe(&client, &initial).await?;
    let _ = subscription.recv_typed().await?;
    subscription.switch_typed(&latest).await?;

    // When
    let message = subscription.recv_typed().await?;
    subscription.close().await?;
    server.stop().await?;

    // Then
    assert!(has_channel(&message, &latest));
    assert_eq!(state.auth_requests.load(Ordering::SeqCst), 2);
    assert_eq!(state.negotiate_requests.load(Ordering::SeqCst), 2);
    assert_eq!(state.start_requests.load(Ordering::SeqCst), 2);
    assert_eq!(state.connections.load(Ordering::SeqCst), 2);
    assert_eq!(
        state
            .invocations
            .lock()
            .expect("invocation lock")
            .as_slice(),
        [
            (1, 1, initial.to_string()),
            (1, 2, latest.to_string()),
            (2, 1, latest.to_string()),
        ]
    );
    Ok(())
}

#[tokio::test]
async fn websocket_disconnect_reconnects_then_surfaces_final_failure() -> TestResult {
    // Given
    let server = TestServer::start(Scenario::DropThenFail).await?;
    let state = Arc::clone(&server.state);
    let client = server.client()?;
    let channel = Channel::raw("TEST:DROPPED")?;
    let mut subscription = subscribe(&client, &channel).await?;

    // When
    let message = subscription.recv_typed().await?;
    let result = subscription.recv_typed().await;
    server.stop().await?;

    // Then
    assert!(has_channel(&message, &channel));
    assert!(matches!(result, Err(StreamError::Http(error))
        if error.status() == Some(StatusCode::SERVICE_UNAVAILABLE)));
    assert_eq!(state.auth_requests.load(Ordering::SeqCst), 3);
    assert_eq!(state.negotiate_requests.load(Ordering::SeqCst), 3);
    assert_eq!(state.start_requests.load(Ordering::SeqCst), 2);
    assert_eq!(state.connections.load(Ordering::SeqCst), 2);
    Ok(())
}

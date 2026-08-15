#![doc = "Capture-backed typed stream contracts."]

use std::{collections::HashMap, error::Error, io, time::Duration};

use axum::{
    Json, Router,
    extract::{
        Query,
        ws::{Message, WebSocket, WebSocketUpgrade},
    },
    response::{IntoResponse as _, Response},
    routing::{get, post},
};
use futures_util::StreamExt as _;
use serde_json::{Map, Value, json};
use ssi_fc_data::{
    api::MarketDataClient,
    config::{Settings, SettingsInput, TransportPolicy},
    stream::{Channel, ChannelSelector, StreamClient, StreamMessage, StreamOptions},
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

async fn access_token() -> Json<Value> {
    Json(json!({
        "status": 200,
        "message": "Success",
        "data": {"accessToken": ACCESS_TOKEN}
    }))
}

async fn negotiate(Query(query): Query<HashMap<String, String>>) -> Response {
    Json(json!({
        "ConnectionToken": query
            .get("clientProtocol")
            .map_or("invalid", String::as_str),
        "ProtocolVersion": "1.3",
        "TryWebSockets": true
    }))
    .into_response()
}

async fn start() -> Json<Value> {
    Json(json!({"Response": "started"}))
}

async fn connect(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    while let Some(Ok(Message::Text(_))) = socket.next().await {
        let content = json!({
            "RType":"B", "TradingDate":"14/08/2026", "Time":"14:45:00",
            "Symbol":"SSI", "Open":24500.0, "High":24500.0, "Low":24500.0,
            "Close":24500.0, "Volume":1800.0, "Value":0.0
        });
        let payload = envelope("B", &content);
        let frame = json!({
            "M": [{"H": "FcMarketDataV2Hub", "M": "Broadcast", "A": [payload.to_string()]}]
        });
        if socket
            .send(Message::Text(frame.to_string().into()))
            .await
            .is_err()
        {
            return;
        }
    }
}

fn envelope(data_type: &str, content: &Value) -> Value {
    json!({"DataType": data_type, "Content": content.to_string()})
}

fn quote_content() -> Value {
    let mut content = Map::from_iter([
        ("TradingDate".to_owned(), json!("14/08/2026")),
        ("Time".to_owned(), json!("14:45:00")),
        ("Exchange".to_owned(), json!("HOSE")),
        ("Symbol".to_owned(), json!("SSI")),
        ("RType".to_owned(), json!("X-QUOTE")),
        ("TradingSession".to_owned(), json!("C")),
    ]);
    for level in 1..=10 {
        content.insert(
            format!("AskPrice{level}"),
            json!(24_000.0 + f64::from(level)),
        );
        content.insert(format!("AskVol{level}"), json!(1_000.0 + f64::from(level)));
        content.insert(
            format!("BidPrice{level}"),
            json!(23_000.0 + f64::from(level)),
        );
        content.insert(format!("BidVol{level}"), json!(2_000.0 + f64::from(level)));
    }
    Value::Object(content)
}

fn trade_content(isin: &Value) -> Value {
    json!({
        "RType":"X-TRADE", "TradingDate":"14/08/2026", "Time":"14:45:00",
        "Isin":isin, "Symbol":"SSI", "Ceiling":26750.0, "Floor":23250.0,
        "RefPrice":25000.0, "AvgPrice":24642.228, "PriorVal":25000.0,
        "LastPrice":24500.0, "LastVol":1800.0, "TotalVal":838_234_940_000.0,
        "TotalVol":34_016_200.0, "MarketId":"HOSE", "Exchange":"HOSE",
        "TradingSession":"C", "TradingStatus":"N", "Change":-500.0,
        "RatioChange":-2.0, "EstMatchedPrice":0.0, "Highest":24950,
        "Lowest":24300, "Side":"SD"
    })
}

fn index_content(index_type: &Value, trading_session: &Value, market_id: &Value) -> Value {
    json!({
        "IndexId":"VN30", "IndexValEst":1877.0, "IndexValue":1876.81,
        "PriorIndexValue":1909.23, "TradingDate":"14/08/2026", "Time":"15:05:05",
        "TotalTrade":0.0, "TotalQtty":265_895_343.0,
        "TotalValue":9_560_237_967_200.0, "IndexName":"VN30", "Advances":0,
        "NoChanges":0, "Declines":0, "Ceilings":0, "Floors":0, "Change":-32.42,
        "RatioChange":-1.7, "TotalQttyPt":23_259_112.0,
        "TotalValuePt":1_638_191_691_900.0, "Exchange":"HOSE",
        "AllQty":289_154_455.0, "AllValue":11_198_429_659_100.0,
        "IndexType":index_type, "TradingSession":trading_session, "MarketId":market_id,
        "RType":"MI", "TotalQttyOd":0.0, "TotalValueOd":0.0
    })
}

#[test]
fn renders_every_documented_channel_selector() -> Result<(), Box<dyn std::error::Error>> {
    // Given
    let one = ChannelSelector::symbols(["SSI"])?;
    let many = ChannelSelector::symbols(["SSI", "PAN"])?;
    let indexes = ChannelSelector::symbols(["VN30", "HNXindex"])?;
    let all = ChannelSelector::all();

    // When
    let rendered = [
        Channel::securities_status(&many),
        Channel::quote(&all),
        Channel::trade(&one),
        Channel::foreign_room(&one),
        Channel::index(&indexes),
        Channel::bar(&all),
    ]
    .map(|channel| channel.to_string());

    // Then
    assert_eq!(
        rendered,
        [
            "F:SSI-PAN",
            "X-QUOTE:ALL",
            "X-TRADE:SSI",
            "R:SSI",
            "MI:VN30-HNXindex",
            "B:ALL",
        ]
    );
    Ok(())
}

#[test]
fn stream_options_accept_a_typed_channel() -> Result<(), Box<dyn std::error::Error>> {
    // Given
    let selector = ChannelSelector::symbols(["SSI"])?;
    let channel = Channel::quote(&selector);

    // When
    let result = StreamOptions::from_channel(&channel, 1, Duration::from_secs(5));

    // Then
    assert!(result.is_ok());
    Ok(())
}

#[test]
fn decodes_all_six_capture_backed_payloads() -> Result<(), Box<dyn std::error::Error>> {
    // Given
    let captures = [
        envelope(
            "F",
            &json!({
                "RType":"F", "MarketId":"HOSE", "TradingDate":"14/08/2026",
                "Time":"14:45:03", "Symbol":"SSI", "TradingSession":"C",
                "TradingStatus":"N", "Exchange":"HOSE", "TradingOlSession":""
            }),
        ),
        envelope("X-QUOTE", &quote_content()),
        envelope(
            "X-TRADE",
            &json!({
                "RType":"X-TRADE", "TradingDate":"14/08/2026", "Time":"14:45:00",
                "Isin":"SSI", "Symbol":"SSI", "Ceiling":26750.0, "Floor":23250.0,
                "RefPrice":25000.0, "AvgPrice":24642.228, "PriorVal":25000.0,
                "LastPrice":24500.0, "LastVol":1800.0, "TotalVal":838_234_940_000.0,
                "TotalVol":34_016_200.0, "MarketId":"HOSE", "Exchange":"HOSE",
                "TradingSession":"C", "TradingStatus":"N", "Change":-500.0,
                "RatioChange":-2.0, "EstMatchedPrice":0.0, "Highest":24950,
                "Lowest":24300, "Side":"SD"
            }),
        ),
        envelope(
            "R",
            &json!({
                "RType":"R", "TradingDate":"14/08/2026", "Time":"15:32:00",
                "Isin":"SSI", "Symbol":"SSI", "TotalRoom":2_503_089_220.0,
                "CurrentRoom":1_741_403_260.0, "BuyVol":1_933_300.0, "SellVol":1_587_415.0,
                "BuyVal":47_557_197_950.0, "SellVal":39_195_574_350.0,
                "MarketId":"HOSE", "Exchange":"HOSE"
            }),
        ),
        envelope(
            "MI",
            &json!({
                "IndexId":"VN30", "IndexValue":1876.81, "PriorIndexValue":1909.23,
                "TradingDate":"14/08/2026", "Time":"15:05:05", "TotalTrade":0.0,
                "TotalQtty":265_895_343.0, "TotalValue":9_560_237_967_200.0, "IndexName":"VN30",
                "Advances":0, "NoChanges":0, "Declines":0, "Ceilings":0, "Floors":0,
                "Change":-32.42, "RatioChange":-1.7, "TotalQttyPt":23_259_112.0,
                "TotalValuePt":1_638_191_691_900.0, "Exchange":"HOSE", "AllQty":289_154_455.0,
                "AllValue":11_198_429_659_100.0, "IndexType":"", "TradingSession":"C",
                "MarketId":"HOSE", "RType":"MI", "TotalQttyOd":0.0, "TotalValueOd":0.0
            }),
        ),
        envelope(
            "B",
            &json!({
                "RType":"B", "TradingDate":"14/08/2026", "Time":"14:45:00",
                "Symbol":"SSI", "Open":24500.0, "High":24500.0, "Low":24500.0,
                "Close":24500.0, "Volume":1800.0, "Value":0.0
            }),
        ),
    ];

    // When
    let messages = captures
        .into_iter()
        .map(StreamMessage::try_from)
        .collect::<Result<Vec<_>, _>>()?;
    let [status, quote, trade, room, index, bar]: [StreamMessage; 6] = messages
        .try_into()
        .map_err(|_| io::Error::other("expected all six stream payloads"))?;

    // Then
    assert!(matches!(status, StreamMessage::SecuritiesStatus(value) if value.symbol == "SSI"));
    assert!(
        matches!(quote, StreamMessage::Quote(value) if value.ask_prices.first().is_some_and(|price| (*price - 24_001.0).abs() < f64::EPSILON))
    );
    assert!(
        matches!(trade, StreamMessage::Trade(value) if (value.last_price - 24_500.0).abs() < f64::EPSILON)
    );
    assert!(
        matches!(room, StreamMessage::ForeignRoom(value) if (value.buy_vol - 1_933_300.0).abs() < f64::EPSILON)
    );
    assert!(matches!(index, StreamMessage::Index(value) if value.index_id == "VN30"));
    assert!(
        matches!(bar, StreamMessage::Bar(value) if (value.close - 24_500.0).abs() < f64::EPSILON)
    );
    Ok(())
}

#[test]
fn accepts_documented_aliases_and_structured_content() -> Result<(), Box<dyn std::error::Error>> {
    // Given
    let quote = json!({"datatype":"Quote", "content": quote_content()});
    let trade = json!({"Datatype":"Trade", "Content": {
        "Rtype":"Trade", "TradingDate":"14/08/2026", "Time":"14:45:00",
        "ISIN":"SSI", "Symbol":"SSI", "Ceiling":1, "Floor":1, "RefPrice":1,
        "AvgPrice":1, "PriorVal":1, "LastPrice":1, "LastVol":1, "TotalVal":1,
        "TotalVol":1, "MarketId":"HOSE", "Exchange":"HOSE", "TradingSession":"C",
        "TradingStatus":"N", "Change":0, "RatioChange":0, "EstMatchedPrice":0,
        "Highest":1, "Lowest":1, "Side":"BU"
    }});

    // When
    let quote = StreamMessage::try_from(quote)?;
    let trade = StreamMessage::try_from(trade)?;

    // Then
    assert!(matches!(quote, StreamMessage::Quote(_)));
    assert!(matches!(trade, StreamMessage::Trade(_)));
    Ok(())
}

#[test]
fn preserves_unknown_data_types() -> Result<(), Box<dyn std::error::Error>> {
    // Given
    let envelope = json!({"DataType":"Future-Type", "Content":"{\"sequence\":7}"});

    // When
    let message = StreamMessage::try_from(envelope)?;

    // Then
    assert!(
        matches!(message, StreamMessage::Unknown { data_type, content } if data_type == "Future-Type" && content == json!({"sequence":7}))
    );
    Ok(())
}

#[test]
fn trade_accepts_a_null_isin() -> Result<(), Box<dyn std::error::Error>> {
    // Given
    let envelope = envelope("X-TRADE", &trade_content(&Value::Null));

    // When
    let message = StreamMessage::try_from(envelope)?;

    // Then
    assert!(matches!(message, StreamMessage::Trade(value) if value.isin.is_none()));
    Ok(())
}

#[test]
fn market_index_accepts_documented_nullable_and_estimated_values()
-> Result<(), Box<dyn std::error::Error>> {
    // Given
    let content = index_content(&Value::Null, &Value::Null, &Value::Null);
    let envelope = envelope("MI", &content);

    // When
    let message = StreamMessage::try_from(envelope)?;

    // Then
    assert!(
        matches!(message, StreamMessage::Index(value) if value.index_type.is_none()
        && value.trading_session.is_none()
        && value.market_id.is_none()
        && value.index_val_est.is_some())
    );
    Ok(())
}

#[test]
fn quote_accepts_documented_stock_metadata() -> Result<(), Box<dyn std::error::Error>> {
    // Given
    let mut content = quote_content();
    let object = content
        .as_object_mut()
        .ok_or_else(|| io::Error::other("quote fixture must be an object"))?;
    object.insert("StockNo".to_owned(), json!("1138"));
    object.insert("StockType".to_owned(), json!("Future"));
    let envelope = envelope("X-QUOTE", &content);

    // When
    let message = StreamMessage::try_from(envelope)?;

    // Then
    assert!(
        matches!(message, StreamMessage::Quote(value) if value.stock_no.as_deref() == Some("1138")
        && value.stock_type.as_deref() == Some("Future"))
    );
    Ok(())
}

#[test]
fn securities_status_accepts_missing_odd_lot_session() -> Result<(), Box<dyn std::error::Error>> {
    // Given
    let content = json!({
        "RType":"F", "MarketId":"HNX", "TradingDate":"04/05/2020",
        "Time":"15:00:16", "Symbol":"DPS", "TradingSession":"C",
        "TradingStatus":"NT", "Exchange":"HNX"
    });
    let envelope = envelope("F", &content);

    // When
    let message = StreamMessage::try_from(envelope)?;

    // Then
    assert!(
        matches!(message, StreamMessage::SecuritiesStatus(value) if value.trading_ol_session.is_none())
    );
    Ok(())
}

#[tokio::test]
async fn typed_bounded_collect_decodes_messages() -> TestResult {
    // Given
    let server = TestServer::start().await?;
    let client = server.client()?;
    let selector = ChannelSelector::all();
    let channel = Channel::bar(&selector);
    let options = StreamOptions::from_channel(&channel, 1, Duration::from_secs(5))?;

    // When
    let messages = StreamClient::new(&client).collect_typed(&options).await?;

    // Then
    assert!(matches!(&messages[..], [StreamMessage::Bar(value)] if value.symbol == "SSI"));
    server.stop().await?;
    Ok(())
}

#[tokio::test]
async fn typed_persistent_subscription_receives_and_switches() -> TestResult {
    // Given
    let server = TestServer::start().await?;
    let client = server.client()?;
    let all = ChannelSelector::all();
    let one = ChannelSelector::symbols(["SSI"])?;
    let first_channel = Channel::bar(&all);
    let second_channel = Channel::bar(&one);
    let mut subscription = StreamClient::new(&client)
        .subscribe_typed(&first_channel, Duration::from_secs(5))
        .await?;

    // When
    let first = subscription.recv_typed().await?;
    subscription.switch_typed(&second_channel).await?;
    let second = subscription.recv_typed().await?;
    subscription.close().await?;

    // Then
    assert!(matches!(first, Some(StreamMessage::Bar(value)) if value.symbol == "SSI"));
    assert!(matches!(second, Some(StreamMessage::Bar(value)) if value.symbol == "SSI"));
    server.stop().await?;
    Ok(())
}

#![allow(clippy::expect_used, clippy::indexing_slicing)]
//! Official .NET parity contracts for intraday tick REST data.

use serde_json::{Map, Number, Value, json};
use ssi_fc_data::api::{
    ApiRequest, DailyIndexInput, DailyIndexOptions, DailyIndexQuery, IntradayByTick,
    IntradayByTickInput, IntradayByTickQuery, IntradayByTickResponse, IntradayOhlcInput,
    IntradayOhlcQuery, MarketDataClient, PageQuery, RestRequest,
};
use ssi_fc_data::config::{Settings, SettingsInput, TransportPolicy};
use url::Url;
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{header, method, path, query_param},
};

const TEST_ACCESS_TOKEN: &str = "e30.eyJleHAiOjQxMDI0NDQ4MDB9.signature";

#[test]
fn builds_official_intraday_by_tick_path_and_pascal_case_query() {
    // Given
    let query = tick_query();
    let base = Url::parse("https://fc-data.ssi.com.vn/").expect("valid fixture URL");

    // When
    let url = ApiRequest::IntradayByTick(query)
        .url(&base)
        .expect("valid request URL");

    // Then
    assert_eq!(
        url.as_str(),
        "https://fc-data.ssi.com.vn/api/v2/Market/IntradaybyTick?Symbol=SSI&FromDate=14%2F08%2F2026&ToDate=14%2F08%2F2026&PageIndex=1&PageSize=10"
    );
}

#[test]
fn rejects_live_invalid_intraday_ohlc_page_size_10000() {
    // Given / When
    let page = PageQuery::new(1, 10_000);

    // Then
    assert!(page.is_err());
}

#[test]
fn serializes_live_supported_intraday_ohlc_page_size_1000() {
    // Given
    let page = PageQuery::new(1, 1000).expect("valid intraday page");
    let query = IntradayOhlcQuery::parse(IntradayOhlcInput {
        symbol: "SSI".to_owned(),
        from_date: "14/08/2026".to_owned(),
        to_date: "14/08/2026".to_owned(),
        page,
        ascending: false,
        resolution: 10,
    })
    .expect("valid intraday OHLC query");

    // When
    let url = ApiRequest::IntradayOhlc(query)
        .url(&Url::parse("https://example.com/").expect("valid URL"))
        .expect("valid request URL");

    // Then
    assert!(
        url.query_pairs()
            .any(|(key, value)| key == "pageSize" && value == "1000")
    );
}

#[test]
fn serializes_daily_index_ascending_with_lowercase_key_and_value() {
    // Given
    let query = DailyIndexQuery::parse_with_options(DailyIndexOptions {
        input: DailyIndexInput {
            request_id: "request-1".to_owned(),
            index_id: "VN30".to_owned(),
            from_date: "13/08/2026".to_owned(),
            to_date: "14/08/2026".to_owned(),
            page: PageQuery::new(1, 10).expect("valid page"),
            order_by: "TradingDate".to_owned(),
            order: "asc".to_owned(),
        },
        ascending: true,
    })
    .expect("valid daily index query");

    // When
    let url = ApiRequest::DailyIndex(query)
        .url(&Url::parse("https://example.com/").expect("valid URL"))
        .expect("valid request URL");

    // Then
    assert!(
        url.query_pairs()
            .any(|(key, value)| key == "ascending" && value == "true")
    );
}

#[test]
fn decodes_official_tick_fields_and_nullable_depth_values() {
    // Given
    let envelope = tick_envelope();

    // When
    let response = serde_json::from_value::<IntradayByTickResponse>(envelope)
        .expect("official tick response must decode");

    // Then
    let tick = &response.data[0];
    assert_eq!(tick.symbol, "SSI");
    assert_eq!(
        tick.ask_price_1.as_ref().and_then(Number::as_f64),
        Some(101.25)
    );
    assert_eq!(tick.ask_price_10, None);
    assert_eq!(
        tick.ask_vol_10.as_ref().and_then(Number::as_f64),
        Some(110.0)
    );
    assert_eq!(
        tick.bid_price_10.as_ref().and_then(Number::as_f64),
        Some(90.5)
    );
    assert_eq!(tick.bid_vol_10, None);
    assert_eq!(tick.side.as_deref(), Some("B"));
    assert_eq!(
        tick.price_change_percent.as_ref().and_then(Number::as_f64),
        Some(1.25)
    );
    assert_eq!(tick.change_type.as_deref(), Some("Up"));
}

#[test]
fn associates_intraday_by_tick_with_its_typed_payload() {
    // Given
    let query = tick_query();

    // When / Then
    assert_payload::<_, IntradayByTick>(&query);
}

#[tokio::test]
async fn raw_and_typed_intraday_by_tick_share_the_wire_contract() {
    // Given
    let server = MockServer::start().await;
    mount_auth_mock(&server).await;
    Mock::given(method("GET"))
        .and(path("/api/v2/Market/IntradaybyTick"))
        .and(header(
            "authorization",
            format!("Bearer {TEST_ACCESS_TOKEN}"),
        ))
        .and(query_param("Symbol", "SSI"))
        .and(query_param("FromDate", "14/08/2026"))
        .and(query_param("ToDate", "14/08/2026"))
        .and(query_param("PageIndex", "1"))
        .and(query_param("PageSize", "10"))
        .respond_with(ResponseTemplate::new(200).set_body_json(tick_envelope()))
        .expect(2)
        .mount(&server)
        .await;
    let client = test_client(&server);

    // When
    let typed: IntradayByTickResponse = client
        .execute_typed(&tick_query())
        .await
        .expect("typed request succeeds");
    let raw = client
        .execute(&ApiRequest::IntradayByTick(tick_query()))
        .await
        .expect("raw request succeeds");

    // Then
    assert_eq!(typed.data[0].symbol, "SSI");
    assert_eq!(raw.get("status").and_then(Value::as_str), Some("Success"));
}

fn tick_query() -> IntradayByTickQuery {
    IntradayByTickQuery::parse(IntradayByTickInput {
        symbol: "SSI".to_owned(),
        from_date: "14/08/2026".to_owned(),
        to_date: "14/08/2026".to_owned(),
        page: PageQuery::new(1, 10).expect("valid page"),
    })
    .expect("valid tick query")
}

fn tick_envelope() -> Value {
    let mut tick = Map::from_iter([
        ("Symbol".to_owned(), json!("SSI")),
        ("Open".to_owned(), json!(100.0)),
        ("High".to_owned(), json!(102.0)),
        ("Low".to_owned(), json!(99.5)),
        ("Close".to_owned(), json!(101.25)),
        ("TradingDate".to_owned(), json!("14/08/2026")),
        ("Time".to_owned(), json!("09:15:01")),
        ("Volume".to_owned(), json!(1200.0)),
        ("side".to_owned(), json!("B")),
        ("priceChange".to_owned(), json!(1.25)),
        ("priceChangePercent".to_owned(), json!(1.25)),
        ("changeType".to_owned(), json!("Up")),
    ]);
    for level in 1..=10 {
        tick.insert(
            format!("AskPrice{level}"),
            json!(101.0 + f64::from(level) / 4.0),
        );
        tick.insert(format!("AskVol{level}"), json!(100 + level));
        tick.insert(format!("BidPrice{level}"), json!(100.5 - f64::from(level)));
        tick.insert(format!("BidVol{level}"), json!(200 + level));
    }
    tick.insert("AskPrice10".to_owned(), Value::Null);
    tick.insert("BidVol10".to_owned(), Value::Null);
    json!({
        "data": [Value::Object(tick)],
        "message": "Success",
        "status": "Success",
        "totalRecord": 1
    })
}

const fn assert_payload<R, P>(_: &R)
where
    R: RestRequest<Response = P>,
{
}

async fn mount_auth_mock(server: &MockServer) {
    Mock::given(method("POST"))
        .and(path("/api/v2/Market/AccessToken"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": 200,
            "message": "authenticated",
            "data": {"accessToken": TEST_ACCESS_TOKEN}
        })))
        .expect(1)
        .mount(server)
        .await;
}

fn test_client(server: &MockServer) -> MarketDataClient {
    let base_url = format!("{}/", server.uri());
    let settings = Settings::from_input_with_policy(
        SettingsInput {
            consumer_id: "test-consumer",
            consumer_secret: "test-secret",
            api_url: &base_url,
            stream_url: &base_url,
        },
        TransportPolicy::AllowInsecure,
    )
    .expect("valid test settings");
    MarketDataClient::new(settings).expect("valid test client")
}

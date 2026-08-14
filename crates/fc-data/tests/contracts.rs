#![doc = "Behavioral contracts for SSI request and `SignalR` encoding."]

use ssi_fc_data::{
    api::{
        ApiRequest, DailyIndexInput, DailyIndexQuery, IntradayOhlcInput, IntradayOhlcQuery,
        MarketDataClient, PageQuery, SecuritiesQuery,
    },
    config::{Settings, SettingsInput, TransportPolicy},
    stream::{StreamError, broadcast_payloads, switch_channels_frame},
};
use url::Url;
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{method, path},
};

#[test]
fn accepts_complete_settings_when_all_required_values_are_present() {
    // Given
    let input = SettingsInput {
        consumer_id: "consumer-id",
        consumer_secret: "consumer-secret",
        api_url: "https://fc-data.ssi.com.vn/",
        stream_url: "https://fc-datahub.ssi.com.vn/",
    };

    // When
    let result = Settings::from_input(input);

    // Then
    assert!(result.is_ok());
}

#[test]
fn redacts_credentials_when_settings_input_is_debugged() {
    // Given
    let input = SettingsInput {
        consumer_id: "consumer-id-secret",
        consumer_secret: "consumer-secret-value",
        api_url: "https://fc-data.ssi.com.vn/",
        stream_url: "https://fc-datahub.ssi.com.vn/",
    };

    // When
    let debug = format!("{input:?}");

    // Then
    assert!(!debug.contains("consumer-id-secret"));
    assert!(!debug.contains("consumer-secret-value"));
}

#[test]
fn rejects_cleartext_transport_by_default() {
    // Given
    let input = SettingsInput {
        consumer_id: "consumer-id",
        consumer_secret: "consumer-secret",
        api_url: "http://fc-data.ssi.com.vn/",
        stream_url: "ws://fc-datahub.ssi.com.vn/",
    };

    // When
    let result = Settings::from_input(input);

    // Then
    assert!(result.is_err());
}

#[test]
fn uses_flat_query_keys_when_building_a_securities_request() {
    // Given
    let page = PageQuery::new(1, 10).expect("valid page");
    let query = SecuritiesQuery::new(Some("HOSE".to_owned()), page).expect("valid query");
    let request = ApiRequest::Securities(query);
    let base = Url::parse("https://fc-data.ssi.com.vn/").expect("valid fixture URL");

    // When
    let url = request.url(&base).expect("request URL");

    // Then
    assert_eq!(
        url.as_str(),
        "https://fc-data.ssi.com.vn/api/v2/Market/Securities?market=HOSE&pageIndex=1&pageSize=10"
    );
}

#[test]
fn preserves_resolution_when_building_an_intraday_request() {
    // Given
    let page = PageQuery::new(1, 100).expect("valid page");
    let query = IntradayOhlcQuery::parse(IntradayOhlcInput {
        symbol: "SSI".to_owned(),
        from_date: "14/08/2026".to_owned(),
        to_date: "14/08/2026".to_owned(),
        page,
        ascending: true,
        resolution: 1,
    })
    .expect("valid query");
    let request = ApiRequest::IntradayOhlc(query);
    let base = Url::parse("https://fc-data.ssi.com.vn/").expect("valid fixture URL");

    // When
    let url = request.url(&base).expect("request URL");

    // Then
    assert!(
        url.query()
            .is_some_and(|query| query.contains("resolution=1"))
    );
}

#[test]
fn preserves_official_python_keys_when_building_a_daily_index_request() {
    // Given
    let page = PageQuery::new(1, 10).expect("valid page");
    let query = DailyIndexQuery::parse(DailyIndexInput {
        request_id: "request-1".to_owned(),
        index_id: "VN30".to_owned(),
        from_date: "13/08/2026".to_owned(),
        to_date: "14/08/2026".to_owned(),
        page,
        order_by: "TradingDate".to_owned(),
        order: "desc".to_owned(),
    })
    .expect("valid query");
    let request = ApiRequest::DailyIndex(query);
    let base = Url::parse("https://fc-data.ssi.com.vn/").expect("valid fixture URL");

    // When
    let url = request.url(&base).expect("request URL");
    let query = url.query().expect("query string");

    // Then
    assert!(query.contains("requestId=request-1"));
    assert!(query.contains("indexId=VN30"));
    assert!(query.contains("orderBy=TradingDate"));
}

#[test]
fn rejects_unsupported_pagination_in_the_library() {
    // Given / When
    let invalid_index = PageQuery::new(0, 10);
    let invalid_size = PageQuery::new(1, 25);

    // Then
    assert!(invalid_index.is_err());
    assert!(invalid_size.is_err());
}

#[test]
fn serializes_switch_channels_as_a_legacy_signalr_invocation() {
    // Given
    let channel = "X-QUOTE:ALL";

    // When
    let payload = switch_channels_frame(channel, 1);

    // Then
    assert_eq!(
        payload,
        serde_json::json!({
            "H": "fcmarketdatav2hub",
            "M": "SwitchChannels",
            "A": ["X-QUOTE:ALL"],
            "I": 1
        })
    );
}

#[test]
fn extracts_broadcast_arguments_from_a_legacy_signalr_frame() {
    // Given
    let frame = r#"{"C":"cursor","M":[{"H":"FcMarketDataV2Hub","M":"Broadcast","A":[{"DataType":"MI","Content":"{}"}]}]}"#;

    // When
    let payloads = broadcast_payloads(frame).expect("valid SignalR frame");

    // Then
    assert_eq!(
        payloads,
        vec![serde_json::json!({"DataType": "MI", "Content": "{}"})]
    );
}

#[tokio::test]
async fn reuses_a_cached_access_token_across_authentication_calls() {
    // Given
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/api/v2/Market/AccessToken"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "status": 200,
            "message": "Success",
            "data": {"accessToken": "e30.eyJleHAiOjQxMDI0NDQ4MDB9.signature"}
        })))
        .expect(1)
        .mount(&server)
        .await;
    let settings = Settings::from_input_with_policy(
        SettingsInput {
            consumer_id: "consumer-id",
            consumer_secret: "consumer-secret",
            api_url: &server.uri(),
            stream_url: &server.uri(),
        },
        TransportPolicy::AllowInsecure,
    )
    .expect("test settings");
    let client = MarketDataClient::new(settings).expect("test client");

    // When
    client.authenticate().await.expect("first authentication");
    client.authenticate().await.expect("cached authentication");

    // Then
    server.verify().await;
}

#[tokio::test]
async fn redacts_signalr_connection_tokens_from_http_errors() {
    // Given
    let secret = "connection-token-must-not-leak";
    let error = reqwest::Client::new()
        .get(format!("http://127.0.0.1:1/start?connectionToken={secret}"))
        .send()
        .await
        .expect_err("closed local port must fail");

    // When
    let displayed = StreamError::from(error).to_string();

    // Then
    assert!(!displayed.contains(secret));
}

#![allow(clippy::expect_used, clippy::indexing_slicing)]
//! Capture-backed REST request and response contracts.

use serde::de::DeserializeOwned;
use serde_json::{Value, json};
use ssi_fc_data::api::{
    ApiRequest, DailyIndex, DailyIndexInput, DailyIndexQuery, DailyIndexResponse, DailyOhlc,
    DailyOhlcInput, DailyOhlcQuery, DailyOhlcResponse, DailyStockPrice, DailyStockPriceInput,
    DailyStockPriceQuery, DailyStockPriceResponse, Index, IndexComponent, IndexComponents,
    IndexComponentsQuery, IndexComponentsResponse, IndexListQuery, IndexListResponse, IntradayOhlc,
    IntradayOhlcParams, IntradayOhlcQuery, IntradayOhlcResponse, MarketDataClient, PageQuery,
    RestRequest, SecuritiesDetails, SecuritiesDetailsQuery, SecuritiesDetailsResponse,
    SecuritiesQuery, SecuritiesResponse, Security, SsiDate,
};
use ssi_fc_data::config::{Settings, SettingsInput, TransportPolicy};
use url::Url;
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{header, method, path, query_param},
};

const TEST_ACCESS_TOKEN: &str = "e30.eyJleHAiOjQxMDI0NDQ4MDB9.signature";

#[test]
fn rejects_page_size_500_for_securities_endpoints() {
    // Given
    let page = PageQuery::new(1, 500).expect("shared page size is valid");

    // When
    let securities = SecuritiesQuery::new(None, page);
    let details = SecuritiesDetailsQuery::new(None, None, page);

    // Then
    assert!(securities.is_err());
    assert!(details.is_err());
}

#[test]
fn rejects_bond_for_the_securities_market() {
    // Given
    let page = PageQuery::new(1, 100).expect("valid page");

    // When
    let result = SecuritiesQuery::new(Some("BOND".to_owned()), page);

    // Then
    assert!(result.is_err());
}

#[test]
fn rejects_bond_for_the_securities_details_market() {
    // Given
    let page = PageQuery::new(1, 100).expect("valid page");

    // When
    let result = SecuritiesDetailsQuery::new(Some("BOND".to_owned()), None, page);

    // Then
    assert!(result.is_err());
}

#[test]
fn accepts_only_hose_and_hnx_for_index_list_exchange() {
    // Given
    let page = PageQuery::new(1, 100).expect("valid page");

    // When
    let hose = IndexListQuery::new(Some("HOSE".to_owned()), page);
    let hnx = IndexListQuery::new(Some("HNX".to_owned()), page);
    let upcom = IndexListQuery::new(Some("UPCOM".to_owned()), page);

    // Then
    assert!(hose.is_ok());
    assert!(hnx.is_ok());
    assert!(upcom.is_err());
}

#[test]
fn rejects_dates_without_exact_dd_mm_yyyy_width() {
    // Given
    let page = PageQuery::new(1, 100).expect("valid page");
    let input = DailyOhlcInput {
        symbol: None,
        from_date: "1/08/2026".to_owned(),
        to_date: "14/08/2026".to_owned(),
        page,
        ascending: true,
    };

    // When
    let result = DailyOhlcQuery::parse(input);

    // Then
    assert!(result.is_err());
}

#[test]
fn rejects_daily_ohlc_when_from_date_is_after_to_date() {
    // Given
    let page = PageQuery::new(1, 100).expect("valid page");
    let input = DailyOhlcInput {
        symbol: None,
        from_date: "15/08/2026".to_owned(),
        to_date: "14/08/2026".to_owned(),
        page,
        ascending: true,
    };

    // When
    let result = DailyOhlcQuery::parse(input);

    // Then
    assert!(result.is_err());
}

#[test]
fn accepts_thirty_calendar_days_for_daily_ohlc() {
    // Given
    let page = PageQuery::new(1, 100).expect("valid page");
    let input = DailyOhlcInput {
        symbol: None,
        from_date: "01/01/2026".to_owned(),
        to_date: "31/01/2026".to_owned(),
        page,
        ascending: true,
    };

    // When
    let result = DailyOhlcQuery::parse(input);

    // Then
    assert!(result.is_ok());
}

#[test]
fn rejects_more_than_thirty_calendar_days_for_daily_ohlc() {
    // Given
    let page = PageQuery::new(1, 100).expect("valid page");
    let input = DailyOhlcInput {
        symbol: None,
        from_date: "01/01/2026".to_owned(),
        to_date: "01/02/2026".to_owned(),
        page,
        ascending: true,
    };

    // When
    let result = DailyOhlcQuery::parse(input);

    // Then
    assert!(result.is_err());
}

#[test]
fn orders_ssi_dates_chronologically_across_months_and_years() {
    // Given
    let january = SsiDate::parse("31/01/2026").expect("valid January date");
    let february = SsiDate::parse("01/02/2026").expect("valid February date");
    let december = SsiDate::parse("31/12/2026").expect("valid December date");
    let next_year = SsiDate::parse("01/01/2027").expect("valid next-year date");

    // When / Then
    assert!(january < february);
    assert!(december < next_year);
}

#[test]
fn accepts_daily_ohlc_ranges_across_calendar_boundaries() {
    // Given
    let page = PageQuery::new(1, 100).expect("valid page");
    let cross_month = DailyOhlcInput {
        symbol: None,
        from_date: "31/01/2026".to_owned(),
        to_date: "01/02/2026".to_owned(),
        page,
        ascending: true,
    };
    let cross_year = DailyOhlcInput {
        symbol: None,
        from_date: "31/12/2026".to_owned(),
        to_date: "01/01/2027".to_owned(),
        page,
        ascending: true,
    };

    // When / Then
    assert!(DailyOhlcQuery::parse(cross_month).is_ok());
    assert!(DailyOhlcQuery::parse(cross_year).is_ok());
}

#[test]
fn rejects_reversed_cross_month_range_without_panicking() {
    // Given
    let page = PageQuery::new(1, 100).expect("valid page");
    let input = DailyOhlcInput {
        symbol: None,
        from_date: "01/02/2026".to_owned(),
        to_date: "31/01/2026".to_owned(),
        page,
        ascending: true,
    };

    // When
    let result = DailyOhlcQuery::parse(input);

    // Then
    assert!(result.is_err());
}

#[test]
fn public_ssi_date_roundtrips_exact_wire_format() {
    let date = SsiDate::parse("29/02/2024").expect("valid leap day");

    assert_eq!(date.to_string(), "29/02/2024");
    assert!(SsiDate::parse("29/02/2023").is_err());
    assert!(SsiDate::parse("1/02/2024").is_err());
}

#[test]
fn omits_absent_intraday_dates_from_the_wire_query() {
    let page = PageQuery::new(1, 100).expect("valid page");
    let query = IntradayOhlcQuery::new(IntradayOhlcParams {
        symbol: "SSI".to_owned(),
        from_date: None,
        to_date: None,
        page,
        ascending: true,
        resolution: 1,
    })
    .expect("valid optional-date query");

    let url = ApiRequest::IntradayOhlc(query)
        .url(&Url::parse("https://example.com/").expect("valid URL"))
        .expect("valid request URL");

    assert!(!url.query_pairs().any(|(key, _)| key == "fromDate"));
    assert!(!url.query_pairs().any(|(key, _)| key == "toDate"));
}

#[test]
fn deserializes_securities_capture_shape() {
    let response: SecuritiesResponse = decode(envelope(&json!({
        "Market": "market",
        "StockEnName": "english-name",
        "StockName": "name",
        "Symbol": "symbol"
    })));

    assert_eq!(response.data[0].symbol, "symbol");
}

#[test]
fn deserializes_null_security_names_without_defaulting() {
    // Given
    let capture = envelope(&json!({
        "Market": "market",
        "StockEnName": null,
        "StockName": null,
        "Symbol": "symbol"
    }));

    // When
    let response: SecuritiesResponse = decode(capture);

    // Then
    let security = response.data.first().expect("capture has one security");
    let serialized = serde_json::to_value(security).expect("security serializes");
    assert_eq!(serialized.get("StockEnName"), Some(&Value::Null));
    assert_eq!(serialized.get("StockName"), Some(&Value::Null));
}

#[test]
fn deserializes_securities_details_capture_shape() {
    let response: SecuritiesDetailsResponse = decode(envelope(&securities_details_item()));

    assert_eq!(response.data[0].repeated_info[0].isin, None);
    assert_eq!(response.data[0].repeated_info[0].tick_price4, None);
}

#[test]
fn deserializes_index_components_capture_shape() {
    let response: IndexComponentsResponse = decode(envelope(&json!({
        "Exchange": "exchange",
        "IndexCode": "code",
        "IndexComponent": [{"Isin": "isin", "StockSymbol": "symbol"}],
        "IndexName": "name",
        "TotalSymbolNo": "1"
    })));

    assert_eq!(response.data[0].index_component[0].stock_symbol, "symbol");
}

#[test]
fn deserializes_index_list_capture_shape() {
    let response: IndexListResponse = decode(envelope(&json!({
        "Exchange": "exchange",
        "IndexCode": "code",
        "IndexName": null
    })));

    assert_eq!(response.data[0].index_name, None);
}

#[test]
fn deserializes_daily_ohlc_capture_shape() {
    let response: DailyOhlcResponse = decode(envelope(&json!({
        "Close": "close",
        "High": "high",
        "Low": "low",
        "Market": "market",
        "Open": "open",
        "Symbol": "symbol",
        "Time": null,
        "TradingDate": "date",
        "Value": "value",
        "Volume": "volume"
    })));

    assert_eq!(response.data[0].time, None);
}

#[test]
fn deserializes_intraday_ohlc_capture_shape() {
    let response: IntradayOhlcResponse = decode(envelope(&json!({
        "Close": "close",
        "High": "high",
        "Low": "low",
        "Open": "open",
        "Symbol": "symbol",
        "Time": "time",
        "TradingDate": "date",
        "Value": "value",
        "Volume": "volume"
    })));

    assert_eq!(response.data[0].time, "time");
}

#[test]
fn deserializes_null_data_as_an_empty_record_list() {
    // Given
    let envelope = json!({
        "data": null,
        "message": "No data",
        "status": "Success",
        "totalRecord": 0
    });

    // When
    let response = serde_json::from_value::<IntradayOhlcResponse>(envelope)
        .expect("null data is a valid empty response");

    // Then
    assert!(response.data.is_empty());
}

#[test]
fn deserializes_daily_index_capture_shape() {
    let response: DailyIndexResponse = decode(envelope(&json!({
        "Advances": "advances",
        "Ceilings": "ceilings",
        "Change": "change",
        "Declines": "declines",
        "Floors": "floors",
        "IndexId": "id",
        "IndexName": "name",
        "IndexValue": "value",
        "NoChanges": "no-changes",
        "RatioChange": "ratio",
        "Time": null,
        "TotalDealVal": "deal-value",
        "TotalDealVol": "deal-volume",
        "TotalMatchVal": "match-value",
        "TotalMatchVol": "match-volume",
        "TotalTrade": "trades",
        "TotalVal": "total-value",
        "TotalVol": "total-volume",
        "TradingDate": "date",
        "TradingSession": "session",
        "TypeIndex": null
    })));

    assert_eq!(response.data[0].type_index, None);
}

#[test]
fn deserializes_daily_stock_price_capture_shape() {
    let response: DailyStockPriceResponse = decode(envelope(&daily_stock_price_item()));

    assert_eq!(response.data[0].time, None);
    assert_eq!(response.data[0].close_price_adjusted, "adjusted-close");
}

#[test]
fn associates_each_typed_request_with_its_capture_payload() {
    let page = PageQuery::new(1, 100).expect("valid page");
    let securities = SecuritiesQuery::new(None, page).expect("valid securities query");
    let details = SecuritiesDetailsQuery::new(None, None, page).expect("valid details query");
    let components =
        IndexComponentsQuery::new("VN30".to_owned(), page).expect("valid components query");
    let indexes = IndexListQuery::new(None, page).expect("valid index-list query");
    let daily_ohlc = daily_ohlc_query(page);
    let intraday = IntradayOhlcQuery::new(IntradayOhlcParams {
        symbol: "SSI".to_owned(),
        from_date: None,
        to_date: None,
        page,
        ascending: true,
        resolution: 1,
    })
    .expect("valid intraday query");
    let daily_index = daily_index_query(page);
    let daily_stock = daily_stock_query(page);

    assert_payload::<_, Security>(&securities);
    assert_payload::<_, SecuritiesDetails>(&details);
    assert_payload::<_, IndexComponents>(&components);
    assert_payload::<_, Index>(&indexes);
    assert_payload::<_, DailyOhlc>(&daily_ohlc);
    assert_payload::<_, IntradayOhlc>(&intraday);
    assert_payload::<_, DailyIndex>(&daily_index);
    assert_payload::<_, DailyStockPrice>(&daily_stock);

    let _: Option<IndexComponent> = None;
}

#[tokio::test]
async fn typed_and_raw_execution_share_the_authenticated_wire_contract() {
    let server = MockServer::start().await;
    mount_auth_mock(&server).await;
    Mock::given(method("GET"))
        .and(path("/api/v2/Market/Securities"))
        .and(header(
            "authorization",
            format!("Bearer {TEST_ACCESS_TOKEN}"),
        ))
        .and(query_param("pageIndex", "1"))
        .and(query_param("pageSize", "100"))
        .respond_with(ResponseTemplate::new(200).set_body_json(envelope(&json!({
            "Market": "market",
            "StockEnName": "english-name",
            "StockName": "name",
            "Symbol": "symbol"
        }))))
        .expect(2)
        .mount(&server)
        .await;
    let client = test_client(&server);
    let page = PageQuery::new(1, 100).expect("valid page");
    let typed_query = SecuritiesQuery::new(None, page).expect("valid typed query");
    let raw_query = SecuritiesQuery::new(None, page).expect("valid raw query");

    let typed: SecuritiesResponse = client
        .execute_typed(&typed_query)
        .await
        .expect("typed request succeeds");
    let raw = client
        .execute(&ApiRequest::Securities(raw_query))
        .await
        .expect("raw request succeeds");

    assert_eq!(typed.data[0].symbol, "symbol");
    assert_eq!(raw.get("status").and_then(Value::as_str), Some("status"));
}

fn decode<T: DeserializeOwned>(value: Value) -> T {
    serde_json::from_value(value).expect("capture shape must deserialize")
}

fn envelope(item: &Value) -> Value {
    json!({
        "data": [item],
        "message": "message",
        "status": "status",
        "totalRecord": 1
    })
}

const fn assert_payload<R, P>(_: &R)
where
    R: RestRequest<Response = P>,
{
}

fn daily_ohlc_query(page: PageQuery) -> DailyOhlcQuery {
    DailyOhlcQuery::parse(DailyOhlcInput {
        symbol: None,
        from_date: "01/01/2026".to_owned(),
        to_date: "02/01/2026".to_owned(),
        page,
        ascending: true,
    })
    .expect("valid daily OHLC query")
}

fn daily_index_query(page: PageQuery) -> DailyIndexQuery {
    DailyIndexQuery::parse(DailyIndexInput {
        request_id: "request".to_owned(),
        index_id: "VN30".to_owned(),
        from_date: "01/01/2026".to_owned(),
        to_date: "02/01/2026".to_owned(),
        page,
        order_by: "TradingDate".to_owned(),
        order: "desc".to_owned(),
    })
    .expect("valid daily index query")
}

fn daily_stock_query(page: PageQuery) -> DailyStockPriceQuery {
    DailyStockPriceQuery::parse(DailyStockPriceInput {
        symbol: None,
        from_date: "01/01/2026".to_owned(),
        to_date: "02/01/2026".to_owned(),
        page,
        market: None,
    })
    .expect("valid daily stock query")
}

fn securities_details_item() -> Value {
    json!({
        "RType": "type",
        "ReportDate": "date",
        "TotalNoSym": "1",
        "RepeatedInfo": [{
            "ContractMultiplier": "multiplier",
            "ExcerciseRatio": "ratio",
            "Exchange": "exchange",
            "ExercisePrice": "price",
            "ExerciseStyle": "style",
            "FirstTradingDate": "first-date",
            "Isin": null,
            "IssueDate": "issue-date",
            "Issuer": null,
            "LastTradingDate": "last-date",
            "ListedShare": "listed-share",
            "LotSize": "lot-size",
            "MarketId": "market-id",
            "MaturityDate": "maturity-date",
            "PutOrCall": null,
            "SecType": "security-type",
            "SettlMethod": "settlement",
            "Symbol": "symbol",
            "SymbolEngName": "english-name",
            "SymbolName": "name",
            "TickIncrement1": "increment-1",
            "TickIncrement2": "increment-2",
            "TickIncrement3": "increment-3",
            "TickIncrement4": null,
            "TickPrice1": "price-1",
            "TickPrice2": "price-2",
            "TickPrice3": "price-3",
            "TickPrice4": null,
            "Underlying": null
        }]
    })
}

fn daily_stock_price_item() -> Value {
    json!({
        "AveragePrice": "average",
        "CeilingPrice": "ceiling",
        "ClosePrice": "close",
        "ClosePriceAdjusted": "adjusted-close",
        "FloorPrice": "floor",
        "ForeignBuyValTotal": "foreign-buy-value",
        "ForeignBuyVolTotal": "foreign-buy-volume",
        "ForeignCurrentRoom": "foreign-room",
        "ForeignSellValTotal": "foreign-sell-value",
        "ForeignSellVolTotal": "foreign-sell-volume",
        "HighestPrice": "high",
        "LowestPrice": "low",
        "NetBuySellVal": "net-value",
        "NetBuySellVol": "net-volume",
        "OpenPrice": "open",
        "PerPriceChange": "percent-change",
        "PriceChange": "change",
        "RefPrice": "reference",
        "Symbol": "symbol",
        "Time": null,
        "TotalBuyTrade": "buy-trades",
        "TotalBuyTradeVol": "buy-volume",
        "TotalDealVal": "deal-value",
        "TotalDealVol": "deal-volume",
        "TotalMatchVal": "match-value",
        "TotalMatchVol": "match-volume",
        "TotalSellTrade": "sell-trades",
        "TotalSellTradeVol": "sell-volume",
        "TotalTradedValue": "traded-value",
        "TotalTradedVol": "traded-volume",
        "TradingDate": "date"
    })
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

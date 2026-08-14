//! REST endpoint request models.

use serde::Serialize;
use thiserror::Error;
use url::Url;

mod validation;

use validation as validate;

const SECURITIES_PATH: &str = "api/v2/Market/Securities";
const SECURITIES_DETAILS_PATH: &str = "api/v2/Market/SecuritiesDetails";
const INDEX_COMPONENTS_PATH: &str = "api/v2/Market/IndexComponents";
const INDEX_LIST_PATH: &str = "api/v2/Market/IndexList";
const DAILY_OHLC_PATH: &str = "api/v2/Market/DailyOhlc";
const INTRADAY_OHLC_PATH: &str = "api/v2/Market/IntradayOhlc";
const DAILY_INDEX_PATH: &str = "api/v2/Market/DailyIndex";
const DAILY_STOCK_PRICE_PATH: &str = "api/v2/Market/DailyStockPrice";

/// Invalid SSI Market Data request input.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum ValidationError {
    /// Page indexes must be between one and ten.
    #[error("page index must be between 1 and 10, received {0}")]
    InvalidPageIndex(u8),
    /// Page size was not accepted by SSI.
    #[error("unsupported page size {0}")]
    InvalidPageSize(u16),
    /// A required request field was empty.
    #[error("required request field {0} must not be empty")]
    Missing(&'static str),
    /// A market code was not accepted by SSI.
    #[error("unsupported SSI market {0}")]
    InvalidMarket(String),
    /// A date was not a valid DD/MM/YYYY calendar date.
    #[error("request field {0} must be a valid DD/MM/YYYY date")]
    InvalidDate(&'static str),
    /// Daily index ordering was neither ascending nor descending.
    #[error("order must be asc or desc, received {0}")]
    InvalidOrder(String),
    /// Intraday aggregation resolution was outside the supported range.
    #[error("resolution must be between 1 and 1440 minutes, received {0}")]
    InvalidResolution(u16),
}

/// Shared pagination parameters.
#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PageQuery {
    /// One-based page number.
    page_index: u8,
    /// Requested page size.
    page_size: u16,
}

impl PageQuery {
    /// Parses SSI pagination bounds.
    pub fn new(page_index: u8, page_size: u16) -> Result<Self, ValidationError> {
        validate::page(page_index, page_size)?;
        Ok(Self {
            page_index,
            page_size,
        })
    }
}

/// Securities list request.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SecuritiesQuery {
    /// Optional SSI market code.
    #[serde(skip_serializing_if = "Option::is_none")]
    market: Option<String>,
    /// Pagination.
    #[serde(flatten)]
    page: PageQuery,
}

impl SecuritiesQuery {
    /// Parses an optional market and validated pagination.
    pub fn new(market: Option<String>, page: PageQuery) -> Result<Self, ValidationError> {
        validate::market(market.as_deref())?;
        Ok(Self { market, page })
    }
}

/// Securities details request.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SecuritiesDetailsQuery {
    /// Optional SSI market code.
    #[serde(skip_serializing_if = "Option::is_none")]
    market: Option<String>,
    /// Optional instrument symbol.
    #[serde(skip_serializing_if = "Option::is_none")]
    symbol: Option<String>,
    /// Pagination.
    #[serde(flatten)]
    page: PageQuery,
}

impl SecuritiesDetailsQuery {
    /// Parses optional market and symbol filters with validated pagination.
    pub fn new(
        market: Option<String>,
        symbol: Option<String>,
        page: PageQuery,
    ) -> Result<Self, ValidationError> {
        validate::market(market.as_deref())?;
        validate::optional(symbol.as_deref(), "symbol")?;
        Ok(Self {
            market,
            symbol,
            page,
        })
    }
}

/// Index components request.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IndexComponentsQuery {
    /// SSI index code.
    index_code: String,
    /// Pagination.
    #[serde(flatten)]
    page: PageQuery,
}

impl IndexComponentsQuery {
    /// Parses a required index code with validated pagination.
    pub fn new(index_code: String, page: PageQuery) -> Result<Self, ValidationError> {
        validate::required(&index_code, "indexCode")?;
        Ok(Self { index_code, page })
    }
}

/// Index list request.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IndexListQuery {
    /// Optional exchange filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    exchange: Option<String>,
    /// Pagination.
    #[serde(flatten)]
    page: PageQuery,
}

impl IndexListQuery {
    /// Parses an optional exchange with validated pagination.
    pub fn new(exchange: Option<String>, page: PageQuery) -> Result<Self, ValidationError> {
        validate::market(exchange.as_deref())?;
        Ok(Self { exchange, page })
    }
}

/// Unvalidated daily OHLC input.
#[derive(Debug)]
pub struct DailyOhlcInput {
    /// Optional stock, index, or derivative symbol.
    pub symbol: Option<String>,
    /// Start date in DD/MM/YYYY form.
    pub from_date: String,
    /// End date in DD/MM/YYYY form.
    pub to_date: String,
    /// Validated pagination.
    pub page: PageQuery,
    /// Sort direction.
    pub ascending: bool,
}

/// Daily OHLC request.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DailyOhlcQuery {
    /// Optional stock, index, or derivative symbol.
    #[serde(skip_serializing_if = "Option::is_none")]
    symbol: Option<String>,
    /// Start date in DD/MM/YYYY form.
    from_date: String,
    /// End date in DD/MM/YYYY form.
    to_date: String,
    /// Pagination.
    #[serde(flatten)]
    page: PageQuery,
    /// Sort direction.
    ascending: bool,
}

impl DailyOhlcQuery {
    /// Parses daily OHLC dates and optional symbol input.
    pub fn parse(input: DailyOhlcInput) -> Result<Self, ValidationError> {
        validate::optional(input.symbol.as_deref(), "symbol")?;
        validate::date(&input.from_date, "fromDate")?;
        validate::date(&input.to_date, "toDate")?;
        Ok(Self {
            symbol: input.symbol,
            from_date: input.from_date,
            to_date: input.to_date,
            page: input.page,
            ascending: input.ascending,
        })
    }
}

/// Unvalidated intraday OHLC input.
#[derive(Debug)]
pub struct IntradayOhlcInput {
    /// Stock, derivative, or covered-warrant symbol.
    pub symbol: String,
    /// Start date in DD/MM/YYYY form.
    pub from_date: String,
    /// End date in DD/MM/YYYY form.
    pub to_date: String,
    /// Validated pagination.
    pub page: PageQuery,
    /// Sort direction.
    pub ascending: bool,
    /// Aggregation resolution in minutes.
    pub resolution: u16,
}

/// Intraday OHLC request.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IntradayOhlcQuery {
    /// Instrument symbol.
    symbol: String,
    /// Start date in DD/MM/YYYY form.
    from_date: String,
    /// End date in DD/MM/YYYY form.
    to_date: String,
    /// Pagination.
    #[serde(flatten)]
    page: PageQuery,
    /// Sort direction.
    ascending: bool,
    /// Aggregation resolution in minutes.
    resolution: u16,
}

impl IntradayOhlcQuery {
    /// Parses required intraday symbol, dates, and resolution.
    pub fn parse(input: IntradayOhlcInput) -> Result<Self, ValidationError> {
        validate::required(&input.symbol, "symbol")?;
        validate::date(&input.from_date, "fromDate")?;
        validate::date(&input.to_date, "toDate")?;
        if !(1..=1440).contains(&input.resolution) {
            return Err(ValidationError::InvalidResolution(input.resolution));
        }
        Ok(Self {
            symbol: input.symbol,
            from_date: input.from_date,
            to_date: input.to_date,
            page: input.page,
            ascending: input.ascending,
            resolution: input.resolution,
        })
    }
}

/// Unvalidated daily index input.
#[derive(Debug)]
pub struct DailyIndexInput {
    /// Caller-provided correlation ID.
    pub request_id: String,
    /// SSI index identifier.
    pub index_id: String,
    /// Start date in DD/MM/YYYY form.
    pub from_date: String,
    /// End date in DD/MM/YYYY form.
    pub to_date: String,
    /// Validated pagination.
    pub page: PageQuery,
    /// Server-side ordering field.
    pub order_by: String,
    /// Server-side ordering direction.
    pub order: String,
}

/// Daily index request used by the official Python v2.2.2 client.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DailyIndexQuery {
    /// Caller-provided correlation ID.
    request_id: String,
    /// SSI index identifier.
    index_id: String,
    /// Start date in DD/MM/YYYY form.
    from_date: String,
    /// End date in DD/MM/YYYY form.
    to_date: String,
    /// Pagination.
    #[serde(flatten)]
    page: PageQuery,
    /// Server-side ordering field.
    order_by: String,
    /// Server-side ordering direction.
    order: String,
}

impl DailyIndexQuery {
    /// Parses required daily index fields and ordering.
    pub fn parse(input: DailyIndexInput) -> Result<Self, ValidationError> {
        validate::required(&input.index_id, "indexId")?;
        validate::date(&input.from_date, "fromDate")?;
        validate::date(&input.to_date, "toDate")?;
        validate::required(&input.order_by, "orderBy")?;
        validate::order(&input.order)?;
        Ok(Self {
            request_id: input.request_id,
            index_id: input.index_id,
            from_date: input.from_date,
            to_date: input.to_date,
            page: input.page,
            order_by: input.order_by,
            order: input.order,
        })
    }
}

/// Unvalidated daily stock price input.
#[derive(Debug)]
pub struct DailyStockPriceInput {
    /// Optional instrument symbol.
    pub symbol: Option<String>,
    /// Start date in DD/MM/YYYY form.
    pub from_date: String,
    /// End date in DD/MM/YYYY form.
    pub to_date: String,
    /// Validated pagination.
    pub page: PageQuery,
    /// Optional SSI market code.
    pub market: Option<String>,
}

/// Daily stock price request.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DailyStockPriceQuery {
    /// Optional instrument symbol.
    #[serde(skip_serializing_if = "Option::is_none")]
    symbol: Option<String>,
    /// Start date in DD/MM/YYYY form.
    from_date: String,
    /// End date in DD/MM/YYYY form.
    to_date: String,
    /// Pagination.
    #[serde(flatten)]
    page: PageQuery,
    /// Optional SSI market code.
    #[serde(skip_serializing_if = "Option::is_none")]
    market: Option<String>,
}

impl DailyStockPriceQuery {
    /// Parses daily stock dates, filters, and endpoint-specific page size.
    pub fn parse(input: DailyStockPriceInput) -> Result<Self, ValidationError> {
        validate::optional(input.symbol.as_deref(), "symbol")?;
        validate::date(&input.from_date, "fromDate")?;
        validate::date(&input.to_date, "toDate")?;
        validate::market(input.market.as_deref())?;
        validate::stock_page_size(input.page.page_size)?;
        Ok(Self {
            symbol: input.symbol,
            from_date: input.from_date,
            to_date: input.to_date,
            page: input.page,
            market: input.market,
        })
    }
}

/// Supported REST request variants.
#[derive(Debug)]
pub enum ApiRequest {
    /// List securities.
    Securities(SecuritiesQuery),
    /// Get securities details.
    SecuritiesDetails(SecuritiesDetailsQuery),
    /// Get index components.
    IndexComponents(IndexComponentsQuery),
    /// List indexes.
    IndexList(IndexListQuery),
    /// Query daily OHLC data.
    DailyOhlc(DailyOhlcQuery),
    /// Query intraday OHLC data.
    IntradayOhlc(IntradayOhlcQuery),
    /// Query daily index data.
    DailyIndex(DailyIndexQuery),
    /// Query daily stock prices.
    DailyStockPrice(DailyStockPriceQuery),
}

/// REST request construction failure.
#[derive(Debug, Error)]
pub enum RequestError {
    /// The configured API base URL could not join an endpoint path.
    #[error("failed to join SSI API endpoint: {0}")]
    Url(#[from] url::ParseError),
    /// Query serialization failed.
    #[error("failed to serialize SSI API query: {0}")]
    Query(#[from] serde_urlencoded::ser::Error),
}

impl ApiRequest {
    /// Builds the endpoint URL with canonical flat query keys.
    pub fn url(&self, base: &Url) -> Result<Url, RequestError> {
        let endpoint = base.join(self.path())?;
        match self {
            Self::Securities(query) => build_url(endpoint, query),
            Self::SecuritiesDetails(query) => build_url(endpoint, query),
            Self::IndexComponents(query) => build_url(endpoint, query),
            Self::IndexList(query) => build_url(endpoint, query),
            Self::DailyOhlc(query) => build_url(endpoint, query),
            Self::IntradayOhlc(query) => build_url(endpoint, query),
            Self::DailyIndex(query) => build_url(endpoint, query),
            Self::DailyStockPrice(query) => build_url(endpoint, query),
        }
    }

    /// Returns the canonical SSI v2 endpoint path.
    pub const fn path(&self) -> &'static str {
        match self {
            Self::Securities(_) => SECURITIES_PATH,
            Self::SecuritiesDetails(_) => SECURITIES_DETAILS_PATH,
            Self::IndexComponents(_) => INDEX_COMPONENTS_PATH,
            Self::IndexList(_) => INDEX_LIST_PATH,
            Self::DailyOhlc(_) => DAILY_OHLC_PATH,
            Self::IntradayOhlc(_) => INTRADAY_OHLC_PATH,
            Self::DailyIndex(_) => DAILY_INDEX_PATH,
            Self::DailyStockPrice(_) => DAILY_STOCK_PRICE_PATH,
        }
    }
}

fn build_url<T: Serialize>(endpoint: Url, query: &T) -> Result<Url, RequestError> {
    let mut url = endpoint;
    url.set_query(Some(&serde_urlencoded::to_string(query)?));
    Ok(url)
}

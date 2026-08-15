use serde::Serialize;

use super::{PageQuery, SsiDate, ValidationError, validation as validate};

pub(super) const DAILY_INDEX_PATH: &str = "api/v2/Market/DailyIndex";
pub(super) const DAILY_STOCK_PRICE_PATH: &str = "api/v2/Market/DailyStockPrice";

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
    request_id: String,
    index_id: String,
    from_date: SsiDate,
    to_date: SsiDate,
    #[serde(flatten)]
    page: PageQuery,
    order_by: String,
    order: String,
}

impl DailyIndexQuery {
    /// Parses required daily index fields and ordering.
    pub fn parse(input: DailyIndexInput) -> Result<Self, ValidationError> {
        validate::required(&input.index_id, "indexId")?;
        let from_date = validate::date(&input.from_date, "fromDate")?;
        let to_date = validate::date(&input.to_date, "toDate")?;
        validate::required(&input.order_by, "orderBy")?;
        validate::order(&input.order)?;
        Ok(Self {
            request_id: input.request_id,
            index_id: input.index_id,
            from_date,
            to_date,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    symbol: Option<String>,
    from_date: SsiDate,
    to_date: SsiDate,
    #[serde(flatten)]
    page: PageQuery,
    #[serde(skip_serializing_if = "Option::is_none")]
    market: Option<String>,
}

impl DailyStockPriceQuery {
    /// Parses daily stock dates, filters, and endpoint-specific page size.
    pub fn parse(input: DailyStockPriceInput) -> Result<Self, ValidationError> {
        validate::optional(input.symbol.as_deref(), "symbol")?;
        let from_date = validate::date(&input.from_date, "fromDate")?;
        let to_date = validate::date(&input.to_date, "toDate")?;
        validate::market(input.market.as_deref())?;
        validate::stock_page_size(input.page.page_size)?;
        Ok(Self {
            symbol: input.symbol,
            from_date,
            to_date,
            page: input.page,
            market: input.market,
        })
    }
}

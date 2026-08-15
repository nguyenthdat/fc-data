use serde::Serialize;

use super::{PageQuery, SsiDate, ValidationError, validation as validate};

pub(super) const DAILY_OHLC_PATH: &str = "api/v2/Market/DailyOhlc";
pub(super) const INTRADAY_OHLC_PATH: &str = "api/v2/Market/IntradayOhlc";
const MAX_DAILY_RANGE_DAYS: u32 = 30;

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
    #[serde(skip_serializing_if = "Option::is_none")]
    symbol: Option<String>,
    from_date: SsiDate,
    to_date: SsiDate,
    #[serde(flatten)]
    page: PageQuery,
    ascending: bool,
}

impl DailyOhlcQuery {
    /// Parses daily OHLC dates and validates the 30-day range limit.
    pub fn parse(input: DailyOhlcInput) -> Result<Self, ValidationError> {
        validate::optional(input.symbol.as_deref(), "symbol")?;
        let from_date = validate::date(&input.from_date, "fromDate")?;
        let to_date = validate::date(&input.to_date, "toDate")?;
        validate_daily_range(from_date, to_date)?;
        Ok(Self {
            symbol: input.symbol,
            from_date,
            to_date,
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
    /// Optional start date; an empty value is omitted from the query.
    pub from_date: String,
    /// Optional end date; an empty value is omitted from the query.
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
    symbol: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    from_date: Option<SsiDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    to_date: Option<SsiDate>,
    #[serde(flatten)]
    page: PageQuery,
    ascending: bool,
    resolution: u16,
}

/// Typed intraday OHLC parameters with independently optional dates.
#[derive(Debug)]
pub struct IntradayOhlcParams {
    /// Stock, derivative, or covered-warrant symbol.
    pub symbol: String,
    /// Optional start date.
    pub from_date: Option<SsiDate>,
    /// Optional end date.
    pub to_date: Option<SsiDate>,
    /// Validated pagination.
    pub page: PageQuery,
    /// Sort direction.
    pub ascending: bool,
    /// Aggregation resolution in minutes.
    pub resolution: u16,
}

impl IntradayOhlcQuery {
    /// Creates an intraday request with independently optional dates.
    pub fn new(params: IntradayOhlcParams) -> Result<Self, ValidationError> {
        validate::required(&params.symbol, "symbol")?;
        validate::resolution(params.resolution)?;
        Ok(Self {
            symbol: params.symbol,
            from_date: params.from_date,
            to_date: params.to_date,
            page: params.page,
            ascending: params.ascending,
            resolution: params.resolution,
        })
    }

    /// Parses string dates while treating empty date values as omitted.
    pub fn parse(input: IntradayOhlcInput) -> Result<Self, ValidationError> {
        let from_date = validate::optional_date(&input.from_date, "fromDate")?;
        let to_date = validate::optional_date(&input.to_date, "toDate")?;
        Self::new(IntradayOhlcParams {
            symbol: input.symbol,
            from_date,
            to_date,
            page: input.page,
            ascending: input.ascending,
            resolution: input.resolution,
        })
    }
}

fn validate_daily_range(from_date: SsiDate, to_date: SsiDate) -> Result<(), ValidationError> {
    let from_ordinal = from_date.ordinal();
    let to_ordinal = to_date.ordinal();
    if from_ordinal > to_ordinal {
        return Err(ValidationError::InvalidDateRange);
    }
    if to_ordinal - from_ordinal > MAX_DAILY_RANGE_DAYS {
        return Err(ValidationError::DateRangeTooLong(MAX_DAILY_RANGE_DAYS));
    }
    Ok(())
}

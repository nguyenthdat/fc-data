use serde::Serialize;

use super::{PageQuery, SsiDate, ValidationError, validation as validate};

pub(super) const INTRADAY_BY_TICK_PATH: &str = "api/v2/Market/IntradaybyTick";

/// Unvalidated intraday-by-tick input.
#[derive(Debug)]
pub struct IntradayByTickInput {
    /// Security symbol.
    pub symbol: String,
    /// Start date in DD/MM/YYYY form.
    pub from_date: String,
    /// End date in DD/MM/YYYY form.
    pub to_date: String,
    /// Validated pagination.
    pub page: PageQuery,
}

/// Official .NET intraday-by-tick request.
#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct IntradayByTickQuery {
    symbol: String,
    from_date: SsiDate,
    to_date: SsiDate,
    page_index: u8,
    page_size: u16,
}

impl IntradayByTickQuery {
    /// Parses required symbol, dates, and pagination.
    pub fn parse(input: IntradayByTickInput) -> Result<Self, ValidationError> {
        validate::required(&input.symbol, "Symbol")?;
        let from_date = validate::date(&input.from_date, "FromDate")?;
        let to_date = validate::date(&input.to_date, "ToDate")?;
        Ok(Self {
            symbol: input.symbol,
            from_date,
            to_date,
            page_index: input.page.page_index,
            page_size: input.page.page_size,
        })
    }
}

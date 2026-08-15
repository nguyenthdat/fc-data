use serde::Serialize;

use super::{ValidationError, validation as validate};

pub(super) const BACKTEST_PATH: &str = "api/v2/Market/BackTest";

/// `BackTest` request from the Python 2.2.2 public client.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BacktestQuery {
    /// Selected trading date passed through to SSI.
    selected_date: String,
    /// Instrument symbol.
    symbol: String,
}

impl BacktestQuery {
    /// Parses the two required `BackTest` fields.
    pub fn new(selected_date: String, symbol: String) -> Result<Self, ValidationError> {
        validate::required(&selected_date, "selectedDate")?;
        validate::required(&symbol, "symbol")?;
        Ok(Self {
            selected_date,
            symbol,
        })
    }
}

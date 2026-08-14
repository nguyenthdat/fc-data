//! Typed SSI Market Data REST requests.

use secrecy::{ExposeSecret as _, SecretString};

pub(crate) mod client;
mod request;

pub use client::{ClientError, MarketDataClient};
pub use request::{
    ApiRequest, DailyIndexInput, DailyIndexQuery, DailyOhlcInput, DailyOhlcQuery,
    DailyStockPriceInput, DailyStockPriceQuery, IndexComponentsQuery, IndexListQuery,
    IntradayOhlcInput, IntradayOhlcQuery, PageQuery, RequestError, SecuritiesDetailsQuery,
    SecuritiesQuery, ValidationError,
};

#[derive(Debug, Clone)]
pub(crate) struct AccessToken(SecretString);

impl AccessToken {
    pub(crate) fn new(value: String) -> Self {
        Self(SecretString::from(value))
    }

    pub(crate) fn expose(&self) -> &str {
        self.0.expose_secret()
    }
}

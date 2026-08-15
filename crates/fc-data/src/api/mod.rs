//! Typed SSI Market Data REST client, requests, and responses.

use secrecy::{ExposeSecret as _, SecretString};

pub(crate) mod client;
mod request;
mod response;

pub use client::{ClientError, MarketDataClient};
pub use request::{
    ApiRequest, BacktestQuery, DailyIndexInput, DailyIndexQuery, DailyOhlcInput, DailyOhlcQuery,
    DailyStockPriceInput, DailyStockPriceQuery, IndexComponentsQuery, IndexListQuery,
    IntradayOhlcInput, IntradayOhlcParams, IntradayOhlcQuery, PageQuery, RequestError, RestRequest,
    SecuritiesDetailsQuery, SecuritiesQuery, SsiDate, SsiDateError, ValidationError,
};
pub use response::{
    DailyIndex, DailyIndexResponse, DailyOhlc, DailyOhlcResponse, DailyStockPrice,
    DailyStockPriceResponse, Index, IndexComponent, IndexComponents, IndexComponentsResponse,
    IndexListResponse, IntradayOhlc, IntradayOhlcResponse, RestResponse, SecuritiesDetails,
    SecuritiesDetailsResponse, SecuritiesResponse, Security, SecurityDetails,
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

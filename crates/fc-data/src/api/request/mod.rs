//! REST endpoint request models.

use serde::{Serialize, de::DeserializeOwned};
use url::Url;

use super::response::{
    DailyIndex, DailyOhlc, DailyStockPrice, Index, IndexComponents, IntradayOhlc,
    SecuritiesDetails, Security,
};

mod backtest;
mod common;
mod daily;
mod date;
mod ohlc;
mod reference;
mod validation;

pub use backtest::BacktestQuery;
pub use common::{PageQuery, RequestError, ValidationError};
pub use daily::{DailyIndexInput, DailyIndexQuery, DailyStockPriceInput, DailyStockPriceQuery};
pub use date::{SsiDate, SsiDateError};
pub use ohlc::{
    DailyOhlcInput, DailyOhlcQuery, IntradayOhlcInput, IntradayOhlcParams, IntradayOhlcQuery,
};
pub use reference::{
    IndexComponentsQuery, IndexListQuery, SecuritiesDetailsQuery, SecuritiesQuery,
};

use backtest::BACKTEST_PATH;
use common::build_url;
use daily::{DAILY_INDEX_PATH, DAILY_STOCK_PRICE_PATH};
use ohlc::{DAILY_OHLC_PATH, INTRADAY_OHLC_PATH};
use reference::{INDEX_COMPONENTS_PATH, INDEX_LIST_PATH, SECURITIES_DETAILS_PATH, SECURITIES_PATH};

mod sealed {
    pub trait Sealed {
        const PATH: &'static str;
    }
}

/// Concrete SSI request with one statically associated response payload.
pub trait RestRequest: sealed::Sealed + Serialize + Sync {
    /// Capture-backed payload record returned in the response data array.
    type Response: DeserializeOwned + Serialize;
}

macro_rules! impl_rest_request {
    ($request:ty, $response:ty, $path:expr) => {
        impl sealed::Sealed for $request {
            const PATH: &'static str = $path;
        }

        impl RestRequest for $request {
            type Response = $response;
        }
    };
}

impl_rest_request!(SecuritiesQuery, Security, SECURITIES_PATH);
impl_rest_request!(
    SecuritiesDetailsQuery,
    SecuritiesDetails,
    SECURITIES_DETAILS_PATH
);
impl_rest_request!(IndexComponentsQuery, IndexComponents, INDEX_COMPONENTS_PATH);
impl_rest_request!(IndexListQuery, Index, INDEX_LIST_PATH);
impl_rest_request!(DailyOhlcQuery, DailyOhlc, DAILY_OHLC_PATH);
impl_rest_request!(IntradayOhlcQuery, IntradayOhlc, INTRADAY_OHLC_PATH);
impl_rest_request!(DailyIndexQuery, DailyIndex, DAILY_INDEX_PATH);
impl_rest_request!(
    DailyStockPriceQuery,
    DailyStockPrice,
    DAILY_STOCK_PRICE_PATH
);

pub(crate) fn typed_url<R: RestRequest>(request: &R, base: &Url) -> Result<Url, RequestError> {
    build_url(base.join(<R as sealed::Sealed>::PATH)?, request)
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
    /// Query historical `BackTest` data.
    Backtest(BacktestQuery),
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
            Self::Backtest(query) => build_url(endpoint, query),
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
            Self::Backtest(_) => BACKTEST_PATH,
        }
    }
}

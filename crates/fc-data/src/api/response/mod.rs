//! Capture-backed SSI REST response models.

mod envelope;
mod history;
mod intraday_tick;
mod reference;

pub use envelope::RestResponse;
pub use history::{DailyIndex, DailyOhlc, DailyStockPrice, IntradayOhlc};
pub use intraday_tick::IntradayByTick;
pub use reference::{
    Index, IndexComponent, IndexComponents, SecuritiesDetails, Security, SecurityDetails,
};

/// Securities endpoint response.
pub type SecuritiesResponse = RestResponse<Security>;
/// Securities details endpoint response.
pub type SecuritiesDetailsResponse = RestResponse<SecuritiesDetails>;
/// Index components endpoint response.
pub type IndexComponentsResponse = RestResponse<IndexComponents>;
/// Index list endpoint response.
pub type IndexListResponse = RestResponse<Index>;
/// Daily OHLC endpoint response.
pub type DailyOhlcResponse = RestResponse<DailyOhlc>;
/// Intraday OHLC endpoint response.
pub type IntradayOhlcResponse = RestResponse<IntradayOhlc>;
/// Intraday-by-tick endpoint response.
pub type IntradayByTickResponse = RestResponse<IntradayByTick>;
/// Daily index endpoint response.
pub type DailyIndexResponse = RestResponse<DailyIndex>;
/// Daily stock price endpoint response.
pub type DailyStockPriceResponse = RestResponse<DailyStockPrice>;

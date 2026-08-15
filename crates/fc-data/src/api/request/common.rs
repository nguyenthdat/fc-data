use serde::Serialize;
use thiserror::Error;
use url::Url;

use super::validation as validate;

/// Invalid SSI Market Data request input.
#[derive(Debug, Error, PartialEq, Eq)]
#[non_exhaustive]
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
    /// An exchange code was not accepted by the endpoint.
    #[error("unsupported SSI exchange {0}")]
    InvalidExchange(String),
    /// A date was not a valid exact-width DD/MM/YYYY calendar date.
    #[error("request field {0} must be a valid DD/MM/YYYY date")]
    InvalidDate(&'static str),
    /// A request date range was reversed.
    #[error("fromDate must not be after toDate")]
    InvalidDateRange,
    /// A request date range exceeded the endpoint limit.
    #[error("date range must not exceed {0} calendar days")]
    DateRangeTooLong(u32),
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
    pub(super) page_index: u8,
    pub(super) page_size: u16,
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

/// REST request construction failure.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum RequestError {
    /// The configured API base URL could not join an endpoint path.
    #[error("failed to join SSI API endpoint: {0}")]
    Url(#[from] url::ParseError),
    /// Query serialization failed.
    #[error("failed to serialize SSI API query: {0}")]
    Query(#[from] serde_urlencoded::ser::Error),
}

pub(super) fn build_url<T: Serialize>(endpoint: Url, query: &T) -> Result<Url, RequestError> {
    let mut url = endpoint;
    url.set_query(Some(&serde_urlencoded::to_string(query)?));
    Ok(url)
}

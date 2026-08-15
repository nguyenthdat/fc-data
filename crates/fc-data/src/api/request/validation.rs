use super::{SsiDate, ValidationError};

const MARKETS: &[&str] = &["HOSE", "HNX", "UPCOM", "DER", "BOND"];
const SECURITIES_MARKETS: &[&str] = &["HOSE", "HNX", "UPCOM", "DER"];
const INDEX_EXCHANGES: &[&str] = &["HOSE", "HNX"];
const PAGE_SIZES: &[u16] = &[10, 20, 50, 100, 500, 1000];
const SECURITIES_PAGE_SIZES: &[u16] = &[10, 20, 50, 100, 1000];
const STOCK_PAGE_SIZES: &[u16] = &[10, 20, 50, 100];

pub(super) fn page(page_index: u8, page_size: u16) -> Result<(), ValidationError> {
    if !(1..=10).contains(&page_index) {
        return Err(ValidationError::InvalidPageIndex(page_index));
    }
    if !PAGE_SIZES.contains(&page_size) {
        return Err(ValidationError::InvalidPageSize(page_size));
    }
    Ok(())
}

pub(super) fn stock_page_size(page_size: u16) -> Result<(), ValidationError> {
    if STOCK_PAGE_SIZES.contains(&page_size) {
        Ok(())
    } else {
        Err(ValidationError::InvalidPageSize(page_size))
    }
}

pub(super) fn securities_page_size(page_size: u16) -> Result<(), ValidationError> {
    if SECURITIES_PAGE_SIZES.contains(&page_size) {
        Ok(())
    } else {
        Err(ValidationError::InvalidPageSize(page_size))
    }
}

pub(super) fn required(value: &str, field: &'static str) -> Result<(), ValidationError> {
    if value.trim().is_empty() {
        Err(ValidationError::Missing(field))
    } else {
        Ok(())
    }
}

pub(super) fn optional(value: Option<&str>, field: &'static str) -> Result<(), ValidationError> {
    value.map_or(Ok(()), |value| required(value, field))
}

pub(super) fn market(value: Option<&str>) -> Result<(), ValidationError> {
    match value {
        Some(value) if !MARKETS.contains(&value) => {
            Err(ValidationError::InvalidMarket(value.to_owned()))
        }
        Some(_) | None => Ok(()),
    }
}

pub(super) fn securities_market(value: Option<&str>) -> Result<(), ValidationError> {
    validate_code(value, SECURITIES_MARKETS, |value| {
        ValidationError::InvalidMarket(value.to_owned())
    })
}

pub(super) fn index_exchange(value: Option<&str>) -> Result<(), ValidationError> {
    validate_code(value, INDEX_EXCHANGES, |value| {
        ValidationError::InvalidExchange(value.to_owned())
    })
}

pub(super) fn date(value: &str, field: &'static str) -> Result<SsiDate, ValidationError> {
    SsiDate::parse(value).map_err(|_| ValidationError::InvalidDate(field))
}

pub(super) fn optional_date(
    value: &str,
    field: &'static str,
) -> Result<Option<SsiDate>, ValidationError> {
    if value.is_empty() {
        Ok(None)
    } else {
        date(value, field).map(Some)
    }
}

pub(super) fn order(value: &str) -> Result<(), ValidationError> {
    match value {
        "asc" | "desc" => Ok(()),
        _ => Err(ValidationError::InvalidOrder(value.to_owned())),
    }
}

pub(super) fn resolution(value: u16) -> Result<(), ValidationError> {
    if (1..=1440).contains(&value) {
        Ok(())
    } else {
        Err(ValidationError::InvalidResolution(value))
    }
}

fn validate_code<E>(value: Option<&str>, allowed: &[&str], error: E) -> Result<(), ValidationError>
where
    E: FnOnce(&str) -> ValidationError,
{
    match value {
        Some(value) if !allowed.contains(&value) => Err(error(value)),
        Some(_) | None => Ok(()),
    }
}

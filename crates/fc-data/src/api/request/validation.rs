use super::ValidationError;

const MARKETS: &[&str] = &["HOSE", "HNX", "UPCOM", "DER", "BOND"];
const PAGE_SIZES: &[u16] = &[10, 20, 50, 100, 500, 1000];
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

pub(super) fn date(value: &str, field: &'static str) -> Result<(), ValidationError> {
    let mut parts = value.split('/');
    let parsed = match (parts.next(), parts.next(), parts.next(), parts.next()) {
        (Some(day), Some(month), Some(year), None) => Some((day, month, year)),
        _ => None,
    };
    let Some((day, month, year)) = parsed else {
        return Err(ValidationError::InvalidDate(field));
    };
    let day = day
        .parse::<u8>()
        .map_err(|_| ValidationError::InvalidDate(field))?;
    let month = month
        .parse::<u8>()
        .map_err(|_| ValidationError::InvalidDate(field))?;
    let year = year
        .parse::<u16>()
        .map_err(|_| ValidationError::InvalidDate(field))?;
    let max_day = match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if is_leap_year(year) => 29,
        2 => 28,
        _ => return Err(ValidationError::InvalidDate(field)),
    };
    if year == 0 || day == 0 || day > max_day {
        return Err(ValidationError::InvalidDate(field));
    }
    Ok(())
}

pub(super) fn order(value: &str) -> Result<(), ValidationError> {
    match value {
        "asc" | "desc" => Ok(()),
        _ => Err(ValidationError::InvalidOrder(value.to_owned())),
    }
}

const fn is_leap_year(year: u16) -> bool {
    year.is_multiple_of(4) && (!year.is_multiple_of(100) || year.is_multiple_of(400))
}

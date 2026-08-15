use std::{fmt, str::FromStr};

use serde::{Serialize, Serializer};
use thiserror::Error;

/// Exact SSI calendar date encoded as `DD/MM/YYYY`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct SsiDate {
    year: u16,
    month: u8,
    day: u8,
}

/// SSI date parsing failure.
#[derive(Debug, Clone, Copy, Error, PartialEq, Eq)]
#[error("date must be a valid exact-width DD/MM/YYYY value")]
pub struct SsiDateError;

impl SsiDate {
    /// Parses an exact-width SSI calendar date.
    pub fn parse(value: &str) -> Result<Self, SsiDateError> {
        let [
            day_tens,
            day_ones,
            b'/',
            month_tens,
            month_ones,
            b'/',
            y1,
            y2,
            y3,
            y4,
        ] = value.as_bytes()
        else {
            return Err(SsiDateError);
        };
        let digits = [day_tens, day_ones, month_tens, month_ones, y1, y2, y3, y4];
        if !digits.into_iter().all(u8::is_ascii_digit) {
            return Err(SsiDateError);
        }

        let day = (day_tens - b'0') * 10 + (day_ones - b'0');
        let month = (month_tens - b'0') * 10 + (month_ones - b'0');
        let year = u16::from(y1 - b'0') * 1000
            + u16::from(y2 - b'0') * 100
            + u16::from(y3 - b'0') * 10
            + u16::from(y4 - b'0');
        let date = Self { year, month, day };
        if year == 0 || day == 0 || day > days_in_month(month, year) {
            return Err(SsiDateError);
        }
        Ok(date)
    }

    pub(super) fn ordinal(self) -> u32 {
        let previous_year = u32::from(self.year) - 1;
        let leap_days = previous_year / 4 - previous_year / 100 + previous_year / 400;
        let leap_day = u32::from(self.month > 2 && is_leap_year(self.year));
        previous_year * 365
            + leap_days
            + days_before_month(self.month)
            + leap_day
            + u32::from(self.day)
    }
}

impl FromStr for SsiDate {
    type Err = SsiDateError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::parse(value)
    }
}

impl fmt::Display for SsiDate {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{:02}/{:02}/{:04}",
            self.day, self.month, self.year
        )
    }
}

impl Serialize for SsiDate {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_str(self)
    }
}

const fn days_in_month(month: u8, year: u16) -> u8 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if is_leap_year(year) => 29,
        2 => 28,
        _ => 0,
    }
}

const fn days_before_month(month: u8) -> u32 {
    match month {
        2 => 31,
        3 => 59,
        4 => 90,
        5 => 120,
        6 => 151,
        7 => 181,
        8 => 212,
        9 => 243,
        10 => 273,
        11 => 304,
        12 => 334,
        _ => 0,
    }
}

const fn is_leap_year(year: u16) -> bool {
    year.is_multiple_of(4) && (!year.is_multiple_of(100) || year.is_multiple_of(400))
}

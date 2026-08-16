//! Parsing dates and fetching the current date.
//!
//! This module provides two notable functions: [`parse`] and [`today`].

use jiff::{Error as DateError, Zoned, civil};
use non_empty_str::NonEmptyStr;
use thiserror::Error;
use tracing::debug;

pub type Date = civil::Date;

#[derive(Debug, Error)]
#[error("failed to parse date (make sure it is in the `YYYY-MM-DD` format)")]
pub struct Error {
    #[from]
    #[source]
    pub error: DateError,
}

impl Error {
    /// Constructs [`Self`].
    pub const fn new(error: DateError) -> Self {
        Self { error }
    }
}

/// Returns the current [`Date`].
pub fn today() -> Date {
    Zoned::now().date()
}

pub fn parse<S: AsRef<NonEmptyStr>>(string: S) -> Result<Date, Error> {
    let date = string.as_ref().parse()?;

    Ok(date)
}

/// Similar to [`parse`], but defaults to the current date via [`today`] if the input is `None`.
///
/// # Errors
///
/// Returns [`struct@Error`] on invalid strings.
pub fn parse_or_today<S: AsRef<NonEmptyStr>>(option: Option<S>) -> Result<Date, Error> {
    let date = option.map(parse).transpose()?.unwrap_or_else(today);

    debug!("using date `{date}`");

    Ok(date)
}

//! Parsing dates and fetching the current date.
//!
//! This module provides two notable functions: [`parse`] and [`today`].

use time::{error::Parse as ParseError, macros::format_description, Date, OffsetDateTime};

/// The error type returned by [`parse`].
pub type Error = ParseError;

/// Returns the current [`Date`].
pub fn today() -> Date {
    OffsetDateTime::now_utc().date()
}

/// Parses [`str`] slices in the `[year]-[month]-[day]` (aka `YYYY-MM-DD`) format into [`Date`]
/// values.
///
/// # Errors
///
/// Returns [`Error`] on invalid dates.
pub fn parse_slice(string: &str) -> Result<Date, Error> {
    let description = format_description!("[year]-[month]-[day]");

    Date::parse(string, description)
}

/// Parses strings in the `[year]-[month]-[day]` (aka `YYYY-MM-DD`) format into [`Date`] values.
///
/// # Errors
///
/// Returns [`Error`] on invalid dates.
pub fn parse<S: AsRef<str>>(string: S) -> Result<Date, Error> {
    parse_slice(string.as_ref())
}

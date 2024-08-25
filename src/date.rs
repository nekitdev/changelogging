//! Parsing dates and fetching the current date.
//!
//! This module provides two notable functions: [`parse`] and [`today`].

use miette::Diagnostic;
use thiserror::Error;
use time::{macros::format_description, Date, OffsetDateTime};

/// Represents errors that can occur when parsing dates.
#[derive(Debug, Error, Diagnostic)]
#[error("failed to parse `{string}` into date")]
#[diagnostic(
    code(changelogging::date::date),
    help("dates must be in `[year]-[month]-[day]` (aka `YYYY-MM-DD`) format")
)]
pub struct Error {
    /// The string passed to the [`parse`] function.
    pub string: String,
}

impl Error {
    /// Constructs [`Self`].
    pub fn new(string: String) -> Self {
        Self { string }
    }
}

/// Returns the current [`Date`].
pub fn today() -> Date {
    OffsetDateTime::now_utc().date()
}

/// Parses strings in the `[year]-[month]-[day]` (aka `YYYY-MM-DD`) format into [`Date`] values.
///
/// # Errors
///
/// Returns [`struct@Error`] on invalid dates.
pub fn parse_str(string: &str) -> Result<Date, Error> {
    let description = format_description!("[year]-[month]-[day]");

    Date::parse(string, description).map_err(|_| Error::new(string.to_owned()))
}

/// Similar to [`parse_str`], except the input is [`AsRef<str>`].
///
/// # Errors
///
/// Returns [`struct@Error`] on invalid dates.
pub fn parse<S: AsRef<str>>(string: S) -> Result<Date, Error> {
    parse_str(string.as_ref())
}

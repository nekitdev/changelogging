//! Parsing dates and fetching the current date.
//!
//! This module provides two notable functions: [`parse`] and [`today`].

use miette::Diagnostic;
use thiserror::Error;
use time::{macros::format_description, Date, OffsetDateTime};

/// Represents errors that can occur when parsing dates.
#[derive(Debug, Error, Diagnostic)]
#[error("failed to parse `{string}`")]
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
    pub fn new<S: AsRef<str>>(string: S) -> Self {
        let string = string.as_ref().to_owned();

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
pub fn parse<S: AsRef<str>>(string: S) -> Result<Date, Error> {
    let string = string.as_ref();

    let description = format_description!("[year]-[month]-[day]");

    Date::parse(string, description).map_err(|_| Error::new(string))
}

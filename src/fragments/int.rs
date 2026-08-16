use std::{fmt, num::ParseIntError, str::FromStr};

use non_empty_iter::NonEmptyIterator;
use non_empty_str::{EmptyStr, FromNonEmptyStr, NonEmptyStr, NonEmptyString};
use ownership::IntoOwned;
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub type Value = u32;

#[derive(Debug, Error)]
#[error("failed to parse int from non-empty `{string}`: {error}")]
pub struct NonEmptyError {
    error: ParseIntError,
    string: NonEmptyString,
}

impl NonEmptyError {
    pub const fn new(error: ParseIntError, string: NonEmptyString) -> Self {
        Self { error, string }
    }

    pub fn get(self) -> (ParseIntError, NonEmptyString) {
        (self.error, self.string)
    }
}

#[derive(Debug, Error)]
#[error("can not parse int from empty string: {error}")]
pub struct EmptyError {
    #[from]
    error: EmptyStr,
}

impl EmptyError {
    pub const fn new(error: EmptyStr) -> Self {
        Self { error }
    }

    pub const fn get(self) -> EmptyStr {
        self.error
    }
}

#[derive(Debug, Error)]
#[error(transparent)]
pub enum Error {
    Empty(#[from] EmptyError),
    NonEmpty(#[from] NonEmptyError),
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Default,
    Serialize,
    Deserialize,
    IntoOwned,
)]
#[serde(transparent)]
#[repr(transparent)]
pub struct Int {
    value: Value,
}

impl fmt::Display for Int {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.value().fmt(formatter)
    }
}

pub fn is_format<S: AsRef<NonEmptyStr>>(string: S) -> bool {
    string.as_ref().bytes().all(|byte| byte.is_ascii_digit())
}

impl Int {
    pub const fn new(value: Value) -> Self {
        Self { value }
    }

    pub const fn value(&self) -> Value {
        self.value
    }

    pub const fn get(self) -> Value {
        self.value
    }
}

pub(crate) fn check_non_empty(string: &str) -> Result<&NonEmptyStr, EmptyError> {
    let non_empty = string.try_into()?;

    Ok(non_empty)
}

impl FromStr for Int {
    type Err = Error;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let int = check_non_empty(string)?.parse_non_empty()?;

        Ok(int)
    }
}

impl FromNonEmptyStr for Int {
    type Error = NonEmptyError;

    fn from_non_empty_str(string: &NonEmptyStr) -> Result<Self, Self::Error> {
        let value = string
            .parse()
            .map_err(|error| Self::Error::new(error, string.to_owned()))?;

        let int = Self::new(value);

        Ok(int)
    }
}

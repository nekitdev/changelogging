//! Fragment IDs.

use std::{fmt, str::FromStr};

use non_empty_str::{EmptyStr, FromNonEmptyStr, NonEmptyCowStr, NonEmptyStr, NonEmptyString};
use ownership::IntoOwned;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::fragments::int::{Int, NonEmptyError as IntError, is_format};

pub type Str<'s> = NonEmptyCowStr<'s>;

/// The prefix used for string IDs.
pub const PREFIX: &str = "+";

/// Represents fragment IDs.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, IntoOwned)]
#[serde(untagged)]
pub enum Id<'i> {
    /// Integer fragment ID, like `42`.
    Int(Int),
    /// String fragment ID, like `+str` except only `str` is stored.
    Str(Str<'i>),
}

impl From<Int> for Id<'_> {
    fn from(int: Int) -> Self {
        Self::Int(int)
    }
}

impl<'s> From<Str<'s>> for Id<'s> {
    fn from(string: Str<'s>) -> Self {
        Self::Str(string)
    }
}

impl Id<'_> {
    /// Checks whether [`Self`] is [`Self::Int`].
    pub fn is_int(&self) -> bool {
        matches!(self, Self::Int(_))
    }

    /// Checks whether [`Self`] is [`Self::Str`].
    pub fn is_str(&self) -> bool {
        matches!(self, Self::Str(_))
    }
}

impl fmt::Display for Id<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Int(int) => int.fmt(formatter),
            Self::Str(string) => {
                formatter.write_str(PREFIX)?;

                string.fmt(formatter)
            }
        }
    }
}

#[derive(Debug, Error)]
#[error("`{string}` is not in ID format; either `{PREFIX}{{string}}` or `{{int}}` is expected")]
pub struct FormatError {
    string: NonEmptyString,
}

impl FormatError {
    pub const fn new(string: NonEmptyString) -> Self {
        Self { string }
    }

    pub fn get(self) -> NonEmptyString {
        self.string
    }
}

#[derive(Debug, Error)]
#[error("string ID can not be empty: {error}")]
pub struct StrError {
    #[from]
    error: EmptyStr,
}

/// Represents sources of errors that can occur when parsing fragment IDs.
#[derive(Debug, Error)]
#[error(transparent)]
pub enum NonEmptyError {
    /// Empty string error.
    Str(#[from] StrError),
    /// Parse integer error.
    Int(#[from] IntError),
    /// ID format error.
    Format(#[from] FormatError),
}

#[derive(Debug, Error)]
#[error("ID can not be empty: {error}")]
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

pub(crate) fn check_non_empty(string: &str) -> Result<&NonEmptyStr, EmptyError> {
    let non_empty = string.try_into()?;

    Ok(non_empty)
}

impl FromStr for Id<'_> {
    type Err = Error;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let id = check_non_empty(string)?.parse_non_empty()?;

        Ok(id)
    }
}

pub(crate) fn check_non_empty_str(string: &str) -> Result<&NonEmptyStr, StrError> {
    let non_empty = string.try_into()?;

    Ok(non_empty)
}

impl FromNonEmptyStr for Id<'_> {
    type Error = NonEmptyError;

    fn from_non_empty_str(string: &NonEmptyStr) -> Result<Self, Self::Error> {
        if let Some(stripped) = string.strip_prefix(PREFIX) {
            let string_id = check_non_empty_str(stripped)?.to_owned();

            Ok(Self::Str(string_id.into()))
        } else if is_format(string) {
            let int_id = string.parse_non_empty()?;

            Ok(Self::Int(int_id))
        } else {
            let error = FormatError::new(string.to_owned());

            Err(error.into())
        }
    }
}

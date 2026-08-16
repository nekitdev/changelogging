use std::{fmt, path::Path, str::FromStr};

use bon::Builder;
use from_path::FromPath;
use non_empty_str::{EmptyStr, FromNonEmptyStr, NonEmptyStr};
use ownership::IntoOwned;
use path_name::{Error as PathError, PathName};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::fragments::{
    id::{Error as IdError, Id},
    types::{Name, NameError},
};

/// Represents partial fragments.
#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Builder, IntoOwned,
)]
pub struct Partial<'p> {
    /// The ID of the fragment.
    pub id: Id<'p>,
    /// The type of the fragment.
    pub name: Name<'p>,
}

impl<'p> Partial<'p> {
    pub const fn id(&self) -> &Id<'p> {
        &self.id
    }

    pub const fn name(&self) -> &Name<'p> {
        &self.name
    }
}

/// The separator between the ID and the type.
pub const SEPARATOR: &str = ".";

impl fmt::Display for Partial<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.id().fmt(formatter)?;

        formatter.write_str(SEPARATOR)?;

        self.name().fmt(formatter)?;

        Ok(())
    }
}

/// Represents errors that are returned when unexpected EOF occurs.
#[derive(Debug, Error)]
#[error("unexpected eof while parsing fragment name; names start with `{{id}}{SEPARATOR}{{name}}`")]
pub struct EofError;

/// Represents sources of errors that can occur when parsing partial fragments.
#[derive(Debug, Error)]
#[error(transparent)]
pub enum NonEmptyError {
    /// Unexpected EOF error.
    Eof(#[from] EofError),
    /// ID error.
    Id(#[from] IdError),
    /// Name error.
    Name(#[from] NameError),
}

#[derive(Debug, Error)]
#[error("fragment name must be non-empty: {error}")]
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

pub fn check_non_empty_str(string: &str) -> Result<&NonEmptyStr, EmptyError> {
    let non_empty = string.try_into()?;

    Ok(non_empty)
}

impl FromStr for Partial<'_> {
    type Err = Error;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let partial = check_non_empty_str(string)?.parse_non_empty()?;

        Ok(partial)
    }
}

impl FromNonEmptyStr for Partial<'_> {
    type Error = NonEmptyError;

    fn from_non_empty_str(string: &NonEmptyStr) -> Result<Self, Self::Error> {
        let mut split = string.split(SEPARATOR);

        let id = split.next().ok_or(EofError)?.parse()?;

        let name = split.next().ok_or(EofError)?.parse()?;

        let partial = Self::builder().id(id).name(name).build();

        Ok(partial)
    }
}

pub fn check_non_empty<S: AsRef<NonEmptyStr>>(string: S) -> Result<(), NonEmptyError> {
    let _: Partial<'_> = string.as_ref().parse_non_empty()?;

    Ok(())
}

#[derive(Debug, Error)]
#[error(transparent)]
pub enum LoadError {
    Partial(#[from] NonEmptyError),
    Path(#[from] PathError),
}

impl FromPath for Partial<'_> {
    type Error = LoadError;

    fn from_path<P: AsRef<Path>>(path: P) -> Result<Self, Self::Error> {
        let partial = path.path_name()?.parse_non_empty()?;

        Ok(partial)
    }
}

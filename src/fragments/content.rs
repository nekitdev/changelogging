use std::{
    borrow::Cow,
    path::{Path, PathBuf},
};

use from_path::FromPath;
use non_empty_str::{NonEmptyCowStr, NonEmptyStr, NonEmptyString, const_non_empty_str};
use ownership::IntoOwned;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::file::{ReadNonEmptyError, read_non_empty};

#[derive(Debug, Error)]
#[error("failed to read `{path}`")]
pub struct Error {
    error: ReadNonEmptyError,
    path: PathBuf,
}

impl Error {
    pub const fn new(error: ReadNonEmptyError, path: PathBuf) -> Self {
        Self { error, path }
    }

    pub fn get(self) -> (ReadNonEmptyError, PathBuf) {
        (self.error, self.path)
    }
}

/// Represents non-empty content.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, IntoOwned)]
#[serde(transparent)]
#[repr(transparent)]
pub struct Content<'c> {
    string: NonEmptyCowStr<'c>,
}

impl<'c> Content<'c> {
    /// Constructs [`Self`].
    pub const fn new(string: NonEmptyCowStr<'c>) -> Self {
        Self { string }
    }

    pub const fn borrowed(string: &'c NonEmptyStr) -> Self {
        Self::new(Cow::Borrowed(string))
    }

    pub const fn owned(string: NonEmptyString) -> Self {
        Self::new(Cow::Owned(string))
    }

    pub fn get(self) -> NonEmptyCowStr<'c> {
        self.string
    }
}

impl Content<'_> {
    pub fn string(&self) -> &NonEmptyStr {
        self.string.as_ref()
    }
}

impl Default for Content<'_> {
    fn default() -> Self {
        Self::placeholder()
    }
}

/// Represents placeholder content.
pub const PLACEHOLDER: &NonEmptyStr = const_non_empty_str!("Add the fragment content here.");

impl Content<'_> {
    /// Constructs [`Self`] with [`PLACEHOLDER`] content.
    pub const PLACEHOLDER: Self = Self::new(Cow::Borrowed(PLACEHOLDER));

    /// Returns [`Self::PLACEHOLDER`].
    pub const fn placeholder() -> Self {
        Self::PLACEHOLDER
    }
}

impl FromPath for Content<'_> {
    type Error = Error;

    fn from_path<P: AsRef<Path>>(path: P) -> Result<Self, Self::Error> {
        let path = path.as_ref();

        let non_empty = read_non_empty(path).map_err(|error| Error::new(error, path.to_owned()))?;

        let content = Self::owned(non_empty);

        Ok(content)
    }
}

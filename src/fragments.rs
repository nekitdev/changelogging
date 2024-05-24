//! Changelog fragments.

use std::{
    borrow::Cow, collections::HashMap, fs::read_to_string, num::ParseIntError, path::Path,
    str::FromStr,
};

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::paths::{name_str, FromPath};

/// Represents IDs of fragments.
pub type FragmentId = u32;

/// Represents errors that can occur while parsing into [`PartialFragment`].
#[derive(Debug, Error)]
#[error(transparent)]
pub enum ParseError {
    /// Parse ID error.
    Id(#[from] ParseIntError),
    /// Unexpected EOF.
    #[error("unexpected EOF when parsing fragment info")]
    UnexpectedEof,
}

/// Represents partial fragments.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct PartialFragment<'p> {
    /// The ID of the fragment.
    pub id: FragmentId,
    /// The type of the fragment.
    pub type_name: Cow<'p, str>,
}

impl<'p> PartialFragment<'p> {
    /// Constructs [`Self`].
    pub fn new(id: FragmentId, type_name: Cow<'p, str>) -> Self {
        Self { id, type_name }
    }
}

impl FromStr for PartialFragment<'_> {
    type Err = ParseError;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let mut split = string.split(DOT);

        let id = split.next().ok_or(Self::Err::UnexpectedEof)?.parse()?;

        let type_name = split.next().ok_or(Self::Err::UnexpectedEof)?.to_owned();

        Ok(Self::new(id, type_name.into()))
    }
}

/// Checks that the `string` represents some partial fragment.
///
/// This function parses the string provided, discarding the resulting partial fragment.
///
/// # Errors
///
/// Returns [`ParseError`] if `string` is invalid.
pub fn validate<S: AsRef<str>>(string: S) -> Result<(), ParseError> {
    let _check: PartialFragment<'_> = string.as_ref().parse()?;

    Ok(())
}

/// Represents errors that can occur when loading [`Fragment`].
#[derive(Debug, Error)]
#[error(transparent)]
pub enum Error {
    /// I/O error.
    Io(#[from] std::io::Error),
    /// Parse error.
    Parse(#[from] ParseError),
    /// Invalid UTF-8.
    #[error("invalid utf-8")]
    InvalidUtf8,
}

/// Represents fragments.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Fragment<'f> {
    /// The partial fragment.
    ///
    /// This field is flattened during (de)serialization.
    #[serde(flatten)]
    pub partial: PartialFragment<'f>,
    /// The fragment content.
    pub content: Cow<'f, str>,
}

const DOT: char = '.';

impl FromPath for Fragment<'_> {
    type Error = Error;

    fn from_path<P: AsRef<Path>>(path: P) -> Result<Self, Self::Error> {
        let name = name_str(path.as_ref()).ok_or(Self::Error::InvalidUtf8)?;

        let info = name.parse()?;

        let content = read_to_string(path)?.trim().to_owned();

        Ok(Self::new(info, content.into()))
    }
}

impl<'f> Fragment<'f> {
    /// Constructs [`Self`].
    pub fn new(partial: PartialFragment<'f>, content: Cow<'f, str>) -> Self {
        Self { partial, content }
    }
}

impl Fragment<'_> {
    /// References the `partial` field.
    pub fn partial(&self) -> &PartialFragment<'_> {
        &self.partial
    }

    /// References the `content` field.
    pub fn content(&self) -> &str {
        self.content.as_ref()
    }
}

/// Represents arrays of fragments.
pub type Fragments<'f> = [Fragment<'f>];

/// Represents sections.
pub type Sections<'s> = HashMap<Cow<'s, str>, Vec<Fragment<'s>>>;

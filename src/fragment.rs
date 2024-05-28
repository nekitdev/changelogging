//! Changelog fragments.

use std::{
    borrow::Cow,
    collections::HashMap,
    fs::read_to_string,
    num::ParseIntError,
    path::{Path, PathBuf},
    str::FromStr,
};

use miette::Diagnostic;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Represents IDs of fragments.
pub type FragmentId = u32;

/// Represents errors that can occur when parsing fragment IDs.
#[derive(Debug, Error, Diagnostic)]
#[error("invalid ID")]
#[diagnostic(
    code(changelogging::fragment::invalid_id),
    help("fragment IDs are integers")
)]
pub struct InvalidIdError(#[from] pub ParseIntError);

/// Represents errors that can occur when there are not enough parts to parse.
#[derive(Debug, Error, Diagnostic)]
#[error("unexpected EOF")]
#[diagnostic(
    code(changelogging::fragment::unexpected_eof),
    help("make sure the name starts with `{{id}}.{{type}}`")
)]
pub struct UnexpectedEofError;

/// Represents sources of errors that can occur while parsing into [`PartialFragment`].
#[derive(Debug, Error, Diagnostic)]
#[error(transparent)]
#[diagnostic(transparent)]
pub enum ParseErrorSource {
    /// Parse ID errors.
    InvalidId(#[from] InvalidIdError),
    /// Unexpected EOF errors.
    UnexpectedEof(#[from] UnexpectedEofError),
}

/// Represents errors that can occur while parsing into [`PartialFragment`].
#[derive(Debug, Error, Diagnostic)]
#[error("failed to parse `{name}`")]
#[diagnostic(
    code(changelogging::fragment::parse),
    help("fragment names must start with `{{id}}.{{type}}`")
)]
pub struct ParseError {
    /// The source of this error.
    #[source]
    #[diagnostic_source]
    pub source: ParseErrorSource,
    /// The name provided.
    pub name: String,
}

impl ParseError {
    /// Constructs [`Self`].
    pub fn new<S: AsRef<str>>(source: ParseErrorSource, name: S) -> Self {
        let name = name.as_ref().to_owned();

        Self { source, name }
    }

    /// Constructs [`Self`] from [`InvalidIdError`].
    pub fn invalid_id<S: AsRef<str>>(source: InvalidIdError, name: S) -> Self {
        Self::new(source.into(), name)
    }

    /// Constructs [`Self`] from [`UnexpectedEofError`].
    pub fn unexpected_eof<S: AsRef<str>>(source: UnexpectedEofError, name: S) -> Self {
        Self::new(source.into(), name)
    }

    /// Constructs [`InvalidIdError`] and constructs [`Self`] from it.
    pub fn new_invalid_id<S: AsRef<str>>(source: ParseIntError, name: S) -> Self {
        Self::invalid_id(InvalidIdError(source), name)
    }

    /// Constructs [`UnexpectedEofError`] and constructs [`Self`] from it.
    pub fn new_unexpected_eof<S: AsRef<str>>(name: S) -> Self {
        Self::unexpected_eof(UnexpectedEofError, name)
    }
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

const DOT: char = '.';

impl FromStr for PartialFragment<'_> {
    type Err = ParseError;

    fn from_str(name: &str) -> Result<Self, Self::Err> {
        let mut split = name.split(DOT);

        let id = split
            .next()
            .ok_or_else(|| ParseError::new_unexpected_eof(name))?
            .parse()
            .map_err(|error| ParseError::new_invalid_id(error, name))?;

        let type_name = split
            .next()
            .ok_or_else(|| ParseError::new_unexpected_eof(name))?
            .to_owned();

        Ok(Self::new(id, type_name.into()))
    }
}

/// Validates that the `string` represents some partial fragment.
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

/// Checks if the `string` represents some partial fragment.
///
/// This function is equivalent to using [`validate`] and checking that the result is [`Ok`].
pub fn is_valid<S: AsRef<str>>(string: S) -> bool {
    validate(string).is_ok()
}

/// Checks if the [`path_name`] of the given path represents some partial fragment.
pub fn is_valid_path<P: AsRef<Path>>(path: P) -> bool {
    path_name(path.as_ref()).filter(|name| is_valid(name)).is_some()
}

/// Returns the [`file_name`] of the given path if it is valid UTF-8.
///
/// [`file_name`]: std::path::Path::file_name
pub fn path_name(path: &Path) -> Option<&str> {
    path.file_name().and_then(|os_string| os_string.to_str())
}

/// Represents errors that can occur when reading files.
#[derive(Debug, Error, Diagnostic)]
#[error("read failed")]
#[diagnostic(
    code(changelogging::fragment::read),
    help("check whether the file exists and is accessible")
)]
pub struct ReadError(#[from] pub std::io::Error);

/// Represents errors that can occur when encountering invalid UTF-8 fragment names.
#[derive(Debug, Error, Diagnostic)]
#[error("invalid UTF-8 name")]
#[diagnostic(
    code(changelogging::fragment::invalid_utf8),
    help("fragment file names must be valid UTF-8")
)]
pub struct InvalidUtf8Error;

/// Represents sources of errors that can occur when loading [`Fragment`] values.
#[derive(Debug, Error, Diagnostic)]
#[error(transparent)]
#[diagnostic(transparent)]
pub enum ErrorSource {
    /// Invalid UTF-8 errors.
    InvalidUtf8(#[from] InvalidUtf8Error),
    /// Parse errors.
    Parse(#[from] ParseError),
    /// Read errors.
    Read(#[from] ReadError),
}

/// Represents errors that can occur when loading [`Fragment`] values.
#[derive(Debug, Error, Diagnostic)]
#[error("failed to load `{path}`")]
#[diagnostic(
    code(changelogging::fragment::load),
    help("make sure the file is accessible and its name starts with `{{id}}.{{type}}`")
)]
pub struct Error {
    /// The source of this error.
    #[source]
    #[diagnostic_source]
    pub source: ErrorSource,
    /// The path provided.
    pub path: PathBuf,
}

impl Error {
    /// Constructs [`Self`].
    pub fn new<P: AsRef<Path>>(source: ErrorSource, path: P) -> Self {
        let path = path.as_ref().to_owned();

        Self { source, path }
    }

    /// Constructs [`Self`] from [`InvalidUtf8Error`].
    pub fn invalid_utf8<P: AsRef<Path>>(source: InvalidUtf8Error, path: P) -> Self {
        Self::new(source.into(), path)
    }

    /// Constructs [`Self`] from [`ParseError`].
    pub fn parse<P: AsRef<Path>>(source: ParseError, path: P) -> Self {
        Self::new(source.into(), path)
    }

    /// Constructs [`Self`] from [`ReadError`].
    pub fn read<P: AsRef<Path>>(source: ReadError, path: P) -> Self {
        Self::new(source.into(), path)
    }

    /// Constructs [`InvalidUtf8Error`] and constructs [`Self`] from it.
    pub fn new_invalid_utf8<P: AsRef<Path>>(path: P) -> Self {
        Self::new(InvalidUtf8Error.into(), path)
    }

    /// Constructs [`ReadError`] and constructs [`Self`] from it.
    pub fn new_read<P: AsRef<Path>>(source: std::io::Error, path: P) -> Self {
        Self::new(ReadError(source).into(), path)
    }
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

impl<'f> Fragment<'f> {
    /// Constructs [`Self`].
    pub fn new(partial: PartialFragment<'f>, content: Cow<'f, str>) -> Self {
        Self { partial, content }
    }
}

impl Fragment<'_> {
    /// Loads [`Self`] from the given path.
    ///
    /// # Errors
    ///
    /// Returns [`struct@Error`] when reading the file contents or parsing the name fails.
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let path = path.as_ref();

        let name = path_name(path).ok_or_else(|| Error::new_invalid_utf8(path))?;

        let info = name.parse().map_err(|error| Error::parse(error, path))?;

        let content = read_to_string(path)
            .map_err(|error| Error::new_read(error, path))?
            .trim()
            .to_owned();

        Ok(Self::new(info, content.into()))
    }
}

/// Represents arrays of fragments.
pub type Fragments<'f> = [Fragment<'f>];

/// Represents sections.
pub type Sections<'s> = HashMap<Cow<'s, str>, Vec<Fragment<'s>>>;

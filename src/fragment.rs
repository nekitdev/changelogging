//! Changelog fragments.

use std::{
    borrow::Cow,
    collections::HashMap,
    fs::read_to_string,
    path::{Path, PathBuf},
    str::FromStr,
};

use miette::Diagnostic;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::load::Load;

/// Represents integer IDs of fragments.
pub type Integer = u32;

/// Represents fragment IDs.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Id<'i> {
    /// Integer fragment ID.
    Integer(Integer),
    /// String fragment ID.
    String(Cow<'i, str>),
}

impl<'i> Id<'i> {
    /// Constructs [`Self`] from [`Integer`].
    pub fn integer(value: Integer) -> Self {
        Self::Integer(value)
    }

    /// Constructs [`Self`] from [`String`].
    pub fn owned(string: String) -> Self {
        Self::String(Cow::Owned(string))
    }

    /// Constructs [`Self`] from [`str`].
    pub fn borrowed(string: &'i str) -> Self {
        Self::String(Cow::Borrowed(string))
    }
}

impl Id<'_> {
    /// Checks if [`Self`] is [`Integer`].
    pub fn is_integer(&self) -> bool {
        matches!(self, Self::Integer(_))
    }

    /// Checks if [`Self`] is [`String`].
    pub fn is_string(&self) -> bool {
        matches!(self, Self::String(_))
    }
}

impl FromStr for Id<'_> {
    type Err = InvalidIdError;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        if let Some(stripped) = string.strip_prefix(STRING_PREFIX) {
            Ok(Self::owned(stripped.to_owned()))
        } else {
            string
                .parse()
                .map(Self::integer)
                .map_err(|_| Self::Err::new(string.to_owned()))
        }
    }
}

/// Represents errors that can occur when parsing fragment IDs.
#[derive(Debug, Error, Diagnostic)]
#[error("failed to parse `{string}` into fragment ID")]
#[diagnostic(
    code(changelogging::fragment::invalid_id),
    help("fragment IDs are either integers or strings in the `{STRING_PREFIX}string` form")
)]
pub struct InvalidIdError {
    /// The string that could not be parsed into any valid ID.
    pub string: String,
}

impl InvalidIdError {
    /// Constructs [`Self`].
    pub fn new(string: String) -> Self {
        Self { string }
    }
}

/// The prefix used for non-integer fragment IDs.
pub const STRING_PREFIX: char = '~';

/// Represents errors that can occur when there are not enough parts to parse.
#[derive(Debug, Error, Diagnostic)]
#[error("unexpected EOF")]
#[diagnostic(
    code(changelogging::fragment::unexpected_eof),
    help("make sure the name starts with `{{id}}.{{type}}`")
)]
pub struct UnexpectedEofError;

/// Represents sources of errors that can occur while parsing into [`Partial`].
#[derive(Debug, Error, Diagnostic)]
#[error(transparent)]
#[diagnostic(transparent)]
pub enum ParseErrorSource {
    /// Parse ID errors.
    InvalidId(#[from] InvalidIdError),
    /// Unexpected EOF errors.
    UnexpectedEof(#[from] UnexpectedEofError),
}

/// Represents errors that can occur while parsing into [`Partial`].
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
    pub fn new(source: ParseErrorSource, name: String) -> Self {
        Self { source, name }
    }

    /// Constructs [`Self`] from [`InvalidIdError`].
    pub fn invalid_id(error: InvalidIdError, name: String) -> Self {
        Self::new(error.into(), name)
    }

    /// Constructs [`Self`] from [`UnexpectedEofError`].
    pub fn unexpected_eof(error: UnexpectedEofError, name: String) -> Self {
        Self::new(error.into(), name)
    }

    /// Constructs [`InvalidIdError`] and constructs [`Self`] from it.
    pub fn new_invalid_id(string: String, name: String) -> Self {
        Self::invalid_id(InvalidIdError::new(string), name)
    }

    /// Constructs [`UnexpectedEofError`] and constructs [`Self`] from it.
    pub fn new_unexpected_eof(name: String) -> Self {
        Self::unexpected_eof(UnexpectedEofError, name)
    }
}

/// Represents partial fragments.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Partial<'p> {
    /// The ID of the fragment.
    pub id: Id<'p>,
    /// The type of the fragment.
    pub type_name: Cow<'p, str>,
}

impl<'p> Partial<'p> {
    /// Constructs [`Self`].
    pub fn new(id: Id<'p>, type_name: Cow<'p, str>) -> Self {
        Self { id, type_name }
    }

    /// Constructs [`Self`] with the owned type.
    pub fn owned(id: Id<'p>, type_name: String) -> Self {
        Self::new(id, Cow::Owned(type_name))
    }

    /// Constructs [`Self`] with the borrowed type.
    pub fn borrowed(id: Id<'p>, type_name: &'p str) -> Self {
        Self::new(id, Cow::Borrowed(type_name))
    }
}

const DOT: char = '.';

impl FromStr for Partial<'_> {
    type Err = ParseError;

    fn from_str(name: &str) -> Result<Self, Self::Err> {
        let mut split = name.split(DOT);

        let id = split
            .next()
            .ok_or_else(|| ParseError::new_unexpected_eof(name.to_owned()))?
            .parse()
            .map_err(|error| ParseError::invalid_id(error, name.to_owned()))?;

        let type_name = split
            .next()
            .ok_or_else(|| ParseError::new_unexpected_eof(name.to_owned()))?
            .to_owned();

        Ok(Self::owned(id, type_name))
    }
}

/// Validates that the `string` represents some partial fragment.
///
/// This function parses the string provided, discarding the resulting partial fragment.
///
/// # Errors
///
/// Returns [`ParseError`] if `string` is invalid.
pub fn validate_str(string: &str) -> Result<(), ParseError> {
    let _: Partial<'_> = string.parse()?;

    Ok(())
}

/// Similar to [`validate_str`], except the input is [`AsRef<str>`]
///
/// # Errors
///
/// Returns [`ParseError`] if `string` is invalid.
pub fn validate<S: AsRef<str>>(string: S) -> Result<(), ParseError> {
    validate_str(string.as_ref())
}

/// Checks if the [`path_name`] of the given path represents some partial fragment.
pub fn is_valid_path_ref(path: &Path) -> bool {
    path_name(path)
        .filter(|name| validate_str(name).is_ok())
        .is_some()
}

/// Similar to [`is_valid_path_ref`], except the input is [`AsRef<Path>`].
pub fn is_valid_path<P: AsRef<Path>>(path: P) -> bool {
    is_valid_path_ref(path.as_ref())
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
    pub fn new(source: ErrorSource, path: PathBuf) -> Self {
        Self { source, path }
    }

    /// Constructs [`Self`] from [`InvalidUtf8Error`].
    pub fn invalid_utf8(error: InvalidUtf8Error, path: PathBuf) -> Self {
        Self::new(error.into(), path)
    }

    /// Constructs [`Self`] from [`ParseError`].
    pub fn parse(error: ParseError, path: PathBuf) -> Self {
        Self::new(error.into(), path)
    }

    /// Constructs [`Self`] from [`ReadError`].
    pub fn read(error: ReadError, path: PathBuf) -> Self {
        Self::new(error.into(), path)
    }

    /// Constructs [`InvalidUtf8Error`] and constructs [`Self`] from it.
    pub fn new_invalid_utf8(path: PathBuf) -> Self {
        Self::invalid_utf8(InvalidUtf8Error, path)
    }

    /// Constructs [`ReadError`] and constructs [`Self`] from it.
    pub fn new_read(error: std::io::Error, path: PathBuf) -> Self {
        Self::read(ReadError(error), path)
    }
}

/// Represents fragments.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Fragment<'f> {
    /// The partial fragment.
    ///
    /// This field is flattened during (de)serialization.
    #[serde(flatten)]
    pub partial: Partial<'f>,
    /// The fragment content.
    pub content: Cow<'f, str>,
}

impl<'f> Fragment<'f> {
    /// Constructs [`Self`].
    pub fn new(partial: Partial<'f>, content: Cow<'f, str>) -> Self {
        Self { partial, content }
    }

    /// Constructs [`Self`] with the owned content.
    pub fn owned(partial: Partial<'f>, content: String) -> Self {
        Self::new(partial, Cow::Owned(content))
    }

    /// Constructs [`Self`] with the borrowed content.
    pub fn borrowed(partial: Partial<'f>, content: &'f str) -> Self {
        Self::new(partial, Cow::Borrowed(content))
    }
}

impl Load for Fragment<'_> {
    type Error = Error;

    fn load<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let path = path.as_ref();

        let name = path_name(path).ok_or_else(|| Error::new_invalid_utf8(path.to_owned()))?;

        let info = name
            .parse()
            .map_err(|error| Error::parse(error, path.to_owned()))?;

        let content = read_to_string(path)
            .map_err(|error| Error::new_read(error, path.to_owned()))?
            .trim()
            .to_owned();

        Ok(Self::new(info, content.into()))
    }
}

/// Represents arrays of fragments.
pub type Fragments<'f> = [Fragment<'f>];

/// Represents sections.
pub type Sections<'s> = HashMap<Cow<'s, str>, Vec<Fragment<'s>>>;

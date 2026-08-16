//! Parsing TOML strings.
//!
//! This module simply wraps [`toml`] functionality to provide ergonomics an diagnostics.

use std::path::PathBuf;

use serde::de::DeserializeOwned;
use thiserror::Error;
use toml::de::Error as TomlError;

#[derive(Debug, Error)]
#[error("failed to parse string into TOML: {error}")]
pub struct Error {
    #[from]
    error: TomlError,
}

impl Error {
    pub const fn new(error: TomlError) -> Self {
        Self { error }
    }

    pub fn get(self) -> TomlError {
        self.error
    }

    pub const fn with_path(self, path: PathBuf) -> PathError {
        PathError::new(self, path)
    }
}

#[derive(Debug, Error)]
#[error("failed to parse TOML at `{path}`: {error}")]
pub struct PathError {
    #[source]
    error: Error,
    path: PathBuf,
}

impl PathError {
    pub const fn new(error: Error, path: PathBuf) -> Self {
        Self { error, path }
    }

    pub fn get(self) -> (Error, PathBuf) {
        (self.error, self.path)
    }
}

pub fn parse_str<T: DeserializeOwned>(string: &str) -> Result<T, Error> {
    let value = toml::from_str(string)?;

    Ok(value)
}

/// Wraps [`toml::from_str`] to rewrap errors.
///
/// # Errors
///
/// Returns [`struct@Error`] if the string could not be parsed.
pub fn parse<T: DeserializeOwned, S: AsRef<str>>(string: S) -> Result<T, Error> {
    parse_str(string.as_ref())
}

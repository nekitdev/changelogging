//! Loading TOML files.
//!
//! The [`load`] function is used to load TOML files into values
//! of types that implement [`DeserializeOwned`].
//!
//! Parsing errors are rewrapped into [`ParseError`], and the [`struct@Error`] type
//! is used to handle all possible errors that can occur during loading.

use std::path::Path;

use serde::de::DeserializeOwned;
use thiserror::Error;

use crate::{
    file::{ReadError, read},
    toml::parse::{PathError, parse},
};

/// Represents sources of errors that can occur during TOML loading.
#[derive(Debug, Error)]
#[error(transparent)]
pub enum Error {
    /// File error.
    Read(#[from] ReadError),
    /// Parse error.
    Parse(#[from] PathError),
}

pub fn load_path<T: DeserializeOwned>(path: &Path) -> Result<T, Error> {
    let string = read(path)?;

    let value = parse(string).map_err(|error| error.with_path(path.to_owned()))?;

    Ok(value)
}

/// Loads TOML from the given path into the value of specified or inferred type.
///
/// # Errors
///
/// Returns [`enum@Error`] if the file can not be read or its contents can not be parsed.
pub fn load<T: DeserializeOwned, P: AsRef<Path>>(path: P) -> Result<T, Error> {
    load_path(path.as_ref())
}

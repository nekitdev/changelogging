//! Application initialization.
//!
//! This module provides the [`init`] function that handles initialization of `changelogging`.

use std::{
    env::set_current_dir,
    path::{Path, PathBuf},
};

use miette::Diagnostic;
use thiserror::Error;

/// Represents errors that can occur during changing the current directory.
#[derive(Debug, Error, Diagnostic)]
#[error("failed to change the current directory to `{path}`")]
#[diagnostic(
    code(changelogging::init::change_current_directory),
    help("check whether the directory exists and is accessible")
)]
pub struct ChangeCurrentDirectoryError {
    /// The underlying I/O error.
    pub source: std::io::Error,
    /// The path provided.
    pub path: PathBuf,
}

impl ChangeCurrentDirectoryError {
    /// Constructs [`Self`].
    pub fn new(source: std::io::Error, path: PathBuf) -> Self {
        Self { source, path }
    }
}

/// Represents sources of errors that can occur during initialization.
#[derive(Debug, Error, Diagnostic)]
#[error(transparent)]
#[diagnostic(transparent)]
pub enum ErrorSource {
    /// Change current directory errors.
    ChangeCurrentDirectory(#[from] ChangeCurrentDirectoryError),
}

/// Represents errors that can occur during initialization.
#[derive(Debug, Error, Diagnostic)]
#[error("failed to initialize")]
#[diagnostic(
    code(changelogging::init::init),
    help("see the report for more information")
)]
pub struct Error {
    /// The source of this error.
    #[source]
    #[diagnostic_source]
    pub source: ErrorSource,
}

impl Error {
    /// Constructs [`Self`].
    pub fn new(source: ErrorSource) -> Self {
        Self { source }
    }

    /// Constructs [`Self`] from [`ChangeCurrentDirectoryError`].
    pub fn change_current_directory(source: ChangeCurrentDirectoryError) -> Self {
        Self::new(source.into())
    }

    /// Constructs [`ChangeCurrentDirectoryError`] and constructs [`Self`] from it.
    pub fn new_change_current_directory(source: std::io::Error, path: PathBuf) -> Self {
        Self::change_current_directory(ChangeCurrentDirectoryError::new(source, path))
    }
}

/// Initializes `changelogging`, optionally changing the current directory.
///
/// # Errors
///
/// [`struct@Error`] is returned when changing the current directory fails.
pub fn init<D: AsRef<Path>>(directory: Option<D>) -> Result<(), Error> {
    if let Some(path) = directory {
        let path = path.as_ref();

        set_current_dir(path)
            .map_err(|error| Error::new_change_current_directory(error, path.to_owned()))?;
    };

    Ok(())
}

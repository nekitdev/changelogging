//! Application initialization.
//!
//! This module provides the [`init`] function that handles initialization of `changelogging`.

use std::{
    env::set_current_dir,
    io::Error as IoError,
    path::{Path, PathBuf},
};

use bon::builder;
use thiserror::Error;
use tracing::debug;

/// Represents errors that can occur during changing the current directory.
#[derive(Debug, Error)]
#[error("failed to change the current directory to `{path}`")]
pub struct ChangeCurrentDirError {
    /// The underlying I/O error.
    pub source: IoError,
    /// The path provided.
    pub path: PathBuf,
}

impl ChangeCurrentDirError {
    /// Constructs [`Self`].
    pub fn new(source: IoError, path: PathBuf) -> Self {
        Self { source, path }
    }
}

/// Changes the current directory to the provided one.
///
/// # Errors
///
/// Returns [`ChangeCurrentDirError`] if any I/O error occurs.
pub fn change_current_dir<P: AsRef<Path>>(dir: P) -> Result<(), ChangeCurrentDirError> {
    fn change_current_dir_inner(dir: &Path) -> Result<(), ChangeCurrentDirError> {
        debug!(
            "changing current directory to `{display}`",
            display = dir.display()
        );

        set_current_dir(dir).map_err(|error| ChangeCurrentDirError::new(error, dir.to_owned()))
    }

    change_current_dir_inner(dir.as_ref())
}

/// Represents errors that can occur during initialization.
#[derive(Debug, Error)]
#[error("failed to initialize")]
pub enum Error {
    /// Change current directory error.
    ChangeCurrentDir(#[from] ChangeCurrentDirError),
}

/// Initializes `changelogging`, optionally changing the current directory.
///
/// # Errors
///
/// [`struct@Error`] is returned when changing the current directory fails.
#[builder]
pub fn init<D: AsRef<Path>>(directory: Option<D>) -> Result<(), Error> {
    if let Some(path) = directory {
        change_current_dir(path)?;
    };

    Ok(())
}

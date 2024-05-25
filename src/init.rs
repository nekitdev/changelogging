//! Application initialization.

use std::{
    env::set_current_dir,
    path::{Path, PathBuf},
};

use miette::Diagnostic;
use thiserror::Error;

#[derive(Debug, Error, Diagnostic)]
#[error("failed to change the current directory to `{path}`")]
#[diagnostic(
    code(changelogging::init::change_current_directory),
    help("check whether the directory exists and is accessible")
)]
pub struct ChangeCurrentDirectoryError {
    pub source: std::io::Error,
    pub path: PathBuf,
}

impl ChangeCurrentDirectoryError {
    pub fn new<P: AsRef<Path>>(source: std::io::Error, path: P) -> Self {
        let path = path.as_ref().to_owned();

        Self { source, path }
    }
}

#[derive(Debug, Error, Diagnostic)]
#[error(transparent)]
#[diagnostic(transparent)]
pub enum ErrorSource {
    ChangeCurrentDirectory(#[from] ChangeCurrentDirectoryError),
}

#[derive(Debug, Error, Diagnostic)]
#[error("failed to initialize")]
#[diagnostic(
    code(changelogging::init::init),
    help("see the report for more information")
)]
pub struct Error {
    #[source]
    #[diagnostic_source]
    source: ErrorSource,
}

impl Error {
    pub fn new(source: ErrorSource) -> Self {
        Self { source }
    }

    pub fn change_current_directory(source: ChangeCurrentDirectoryError) -> Self {
        Self::new(source.into())
    }

    pub fn new_change_current_directory<P: AsRef<Path>>(source: std::io::Error, path: P) -> Self {
        Self::change_current_directory(ChangeCurrentDirectoryError::new(source, path))
    }
}

/// Initializes `changelogging`, optionally changing current directory.
pub fn init<D: AsRef<Path>>(directory: Option<D>) -> Result<(), Error> {
    if let Some(path) = directory {
        let path = path.as_ref();

        set_current_dir(path).map_err(|error| Error::new_change_current_directory(error, path))?;
    };

    Ok(())
}

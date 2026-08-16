//! Preparing `changelogging` for initialization.

use std::{
    error::Error as CoreError,
    fs::File,
    io::{Error as IoError, stderr},
    path::Path,
};

use bon::builder;
use clap_verbosity_flag::{LogLevel, Verbosity};
use thiserror::Error;
use tracing_subscriber::fmt;

pub type BoxedError = Box<dyn CoreError + Send + Sync + 'static>;

#[derive(Debug, Error)]
#[error("failed to initialize tracing: {error}")]
pub struct InitError {
    error: BoxedError,
}

impl InitError {
    pub const fn new(error: BoxedError) -> Self {
        Self { error }
    }

    pub fn get(self) -> BoxedError {
        self.error
    }
}

#[derive(Debug, Error)]
#[error("failed to open file for tracing: {error}")]
pub struct OpenError {
    error: IoError,
}

impl OpenError {
    pub const fn new(error: IoError) -> Self {
        Self { error }
    }

    pub fn get(self) -> IoError {
        self.error
    }
}

/// Represents errors that can occur during preparation.
#[derive(Debug, Error)]
#[error(transparent)]
pub enum Error {
    Init(#[from] InitError),
    Open(#[from] OpenError),
}

pub fn prepare_file<L: LogLevel, P: AsRef<Path>>(
    verbosity: Verbosity<L>,
    path: P,
) -> Result<(), Error> {
    let file = File::options()
        .create(true)
        .append(true)
        .open(path)
        .map_err(OpenError::new)?;

    fmt()
        .with_writer(file)
        .with_max_level(verbosity)
        .try_init()
        .map_err(InitError::new)?;

    Ok(())
}

pub fn prepare_stderr<L: LogLevel>(verbosity: Verbosity<L>) -> Result<(), Error> {
    fmt()
        .with_writer(stderr)
        .with_max_level(verbosity)
        .try_init()
        .map_err(InitError::new)?;

    Ok(())
}

/// Prepares `changelogging` for initialization.
///
/// This function currently handles tracing initialization.
///
/// # Errors
///
/// Returns [`struct@Error`] if tracing is already initialized.
#[builder]
pub fn prepare<L: LogLevel, P: AsRef<Path>>(
    verbosity: Verbosity<L>,
    path: Option<P>,
) -> Result<(), Error> {
    if let Some(path) = path {
        prepare_file(verbosity, path)
    } else {
        prepare_stderr(verbosity)
    }
}

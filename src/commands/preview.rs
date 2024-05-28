//! Previewing changelog entries.
//!
//! The [`preview`] function implements the `preview` subcommand.

use miette::Diagnostic;
use thiserror::Error;

use crate::{
    builder::{BuildError, Builder, InitError},
    date::{parse, today},
    workspace::Workspace,
};

/// Represents sources of errors that can occur during changelog entry previewing.
#[derive(Debug, Error, Diagnostic)]
#[error(transparent)]
#[diagnostic(transparent)]
pub enum ErrorSource {
    /// Date parsing errors.
    Date(#[from] crate::date::Error),
    /// Initialization errors.
    Init(#[from] InitError),
    /// Build errors.
    Build(#[from] BuildError),
}

/// Represents errors that can occur during changelog entry previewing.
#[derive(Debug, Error, Diagnostic)]
#[error("failed to preview")]
#[diagnostic(
    code(changelogging::commands::preview),
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

    /// Constructs [`Self`] from [`Error`].
    ///
    /// [`Error`]: crate::date::Error
    pub fn date(source: crate::date::Error) -> Self {
        Self::new(source.into())
    }

    /// Constructs [`Self`] from [`InitError`].
    pub fn init(source: InitError) -> Self {
        Self::new(source.into())
    }

    /// Constructs [`Self`] from [`BuildError`].
    pub fn build(source: BuildError) -> Self {
        Self::new(source.into())
    }
}

/// Previews changelog entries.
///
/// # Errors
///
/// Returns [`struct@Error`] if parsing the date, initializing the builder or previewing fails.
pub fn preview<S: AsRef<str>>(workspace: Workspace<'_>, date: Option<S>) -> Result<(), Error> {
    let date = match date {
        Some(string) => parse(string).map_err(|error| Error::date(error))?,
        None => today(),
    };

    let builder = Builder::from_workspace(workspace, date).map_err(|error| Error::init(error))?;

    builder.preview().map_err(|error| Error::build(error))?;

    Ok(())
}

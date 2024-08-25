//! Building changelogs from fragments.
//!
//! The [`build`] function implements the `build` subcommand.

use std::iter::once;

use miette::Diagnostic;
use thiserror::Error;

use crate::{
    builder::{Builder, CollectError, InitError, WriteError},
    date::{parse, today},
    git,
    workspace::Workspace,
};

/// Represents sources of errors that can occur during building.
#[derive(Debug, Error, Diagnostic)]
#[error(transparent)]
#[diagnostic(transparent)]
pub enum ErrorSource {
    /// Date parsing errors.
    Date(#[from] crate::date::Error),
    /// Initialization errors.
    Init(#[from] InitError),
    /// Build and write errors.
    Write(#[from] WriteError),
    /// Collection errors.
    Collect(#[from] CollectError),
    /// `git` errors.
    Git(#[from] crate::git::Error),
}

/// Represents errors that can occur during building.
#[derive(Debug, Error, Diagnostic)]
#[error("failed to build")]
#[diagnostic(
    code(changelogging::commands::build),
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
    pub fn date(error: crate::date::Error) -> Self {
        Self::new(error.into())
    }

    /// Constructs [`Self`] from [`InitError`].
    pub fn init(error: InitError) -> Self {
        Self::new(error.into())
    }

    /// Constructs [`Self`] from [`WriteError`].
    pub fn write(error: WriteError) -> Self {
        Self::new(error.into())
    }

    /// Constructs [`Self`] from [`CollectError`].
    pub fn collect(error: CollectError) -> Self {
        Self::new(error.into())
    }

    /// Constructs [`Self`] from [`Error`].
    ///
    /// [`Error`]: crate::git::Error
    pub fn git(error: crate::git::Error) -> Self {
        Self::new(error.into())
    }
}

/// Builds changelogs from fragments.
///
/// # Errors
///
/// Returns [`struct@Error`] when parsing dates, initializing the builder,
/// building and writing the changelog or collecting paths fails. Also returned if `git` fails.
pub fn build<S: AsRef<str>>(
    workspace: Workspace<'_>,
    date: Option<S>,
    stage: bool,
    remove: bool,
) -> Result<(), Error> {
    let date = match date {
        Some(string) => parse(string).map_err(Error::date)?,
        None => today(),
    };

    let builder = Builder::from_workspace(workspace, date).map_err(Error::init)?;

    builder.write().map_err(Error::write)?;

    if stage {
        let path = builder.config.paths.output.as_ref();

        git::add(once(path)).map_err(Error::git)?;
    }

    if remove {
        let paths = builder.collect_paths().map_err(Error::collect)?;

        git::remove(paths).map_err(Error::git)?;
    }

    Ok(())
}

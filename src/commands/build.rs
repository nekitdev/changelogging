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
    pub fn date(source: crate::date::Error) -> Self {
        Self::new(source.into())
    }

    /// Constructs [`Self`] from [`InitError`].
    pub fn init(source: InitError) -> Self {
        Self::new(source.into())
    }

    /// Constructs [`Self`] from [`WriteError`].
    pub fn write(source: WriteError) -> Self {
        Self::new(source.into())
    }

    /// Constructs [`Self`] from [`CollectError`].
    pub fn collect(source: CollectError) -> Self {
        Self::new(source.into())
    }

    /// Constructs [`Self`] from [`Error`].
    ///
    /// [`Error`]: crate::git::Error
    pub fn git(source: crate::git::Error) -> Self {
        Self::new(source.into())
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
        Some(string) => parse(string).map_err(|error| Error::date(error))?,
        None => today(),
    };

    let builder = Builder::from_workspace(workspace, date).map_err(|error| Error::init(error))?;

    builder.write().map_err(|error| Error::write(error))?;

    if stage {
        let path = builder.config.paths.output.as_ref();

        git::add(once(path)).map_err(|error| Error::git(error))?;
    }

    if remove {
        let paths = builder
            .collect_paths()
            .map_err(|error| Error::collect(error))?;

        git::remove(paths).map_err(|error| Error::git(error))?;
    }

    Ok(())
}

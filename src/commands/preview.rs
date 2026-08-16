//! Previewing changelog entries.
//!
//! The [`preview`] function implements the `preview` subcommand.

use bon::builder;
use non_empty_str::NonEmptyStr;
use thiserror::Error;

use crate::{
    build::{
        builder::{BuildError, Builder, PartialBuilder},
        renderer::RegisterError,
    },
    date::{Error as DateError, parse_or_today},
    workspace::Workspace,
};

/// Represents errors that can occur during changelog entry previewing.
#[derive(Debug, Error)]
#[error(transparent)]
pub enum Error {
    /// Date parsing errors.
    Date(#[from] DateError),
    /// Registering errors.
    Register(#[from] RegisterError),
    /// Build errors.
    Build(#[from] BuildError),
}

/// Previews changelog entries.
///
/// # Errors
///
/// Returns [`struct@Error`] if parsing the date, initializing the builder or previewing fails.
#[builder]
pub fn preview<S: AsRef<NonEmptyStr>>(
    workspace: Workspace<'_>,
    date: Option<S>,
) -> Result<(), Error> {
    let date = parse_or_today(date)?;

    let (context, config) = workspace.get();

    let partial = PartialBuilder::builder()
        .context(context)
        .config(config)
        .date(date)
        .build();

    let builder = Builder::registering(partial)?;

    builder.preview()?;

    Ok(())
}

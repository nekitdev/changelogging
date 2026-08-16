//! Building changelogs from fragments.
//!
//! The [`build`] function implements the `build` subcommand.

use std::path::Path;

use bon::builder;
use non_empty_str::NonEmptyStr;
use thiserror::Error;

use crate::{
    build::{
        builder::{Builder, CollectorError, PartialBuilder, WriteError},
        renderer::RegisterError,
    },
    date::{Error as DateError, parse_or_today},
    git::{Error as GitError, add_one, remove as remove_all},
    workspace::Workspace,
};

/// Represents errors that can occur during building.
#[derive(Debug, Error)]
#[error(transparent)]
pub enum Error {
    /// Date parsing errors.
    Date(#[from] DateError),
    /// Registering errors.
    Register(#[from] RegisterError),
    /// Writing errors.
    Write(#[from] WriteError),
    /// `git` errors.
    Git(#[from] GitError),
    /// Collection errors.
    Collector(#[from] CollectorError),
}

/// Builds changelogs from fragments.
///
/// # Errors
///
/// Returns [`struct@Error`] when parsing dates, initializing the builder,
/// building and writing the changelog or collecting paths fails. Also returned if `git` fails.
#[builder]
pub fn build<G: AsRef<Path>, S: AsRef<NonEmptyStr>>(
    git: G,
    workspace: Workspace<'_>,
    date: Option<S>,
    stage: bool,
    remove: bool,
) -> Result<(), Error> {
    let date = parse_or_today(date)?;

    let (context, config) = workspace.get();

    let partial = PartialBuilder::builder()
        .context(context)
        .config(config)
        .date(date)
        .build();

    let builder = Builder::registering(partial)?;

    builder.write()?;

    if stage {
        add_one(
            git.as_ref(),
            builder.partial().config().paths().output_path(),
        )?;
    }

    if remove {
        let paths = builder.collect_paths()?;

        remove_all(git, paths)?;
    }

    Ok(())
}

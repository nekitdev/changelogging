//! Creating changelog fragments.
//!
//! The [`create`] function implements the `create` subcommand.

use std::path::Path;

use bon::builder;
use non_empty_str::NonEmptyStr;
use thiserror::Error;

use crate::{
    file::{EditError, WriteError, edit as edit_path, write_non_empty},
    fragments::{
        content::PLACEHOLDER,
        partial::{NonEmptyError as CheckError, check_non_empty},
    },
    git::{Error as GitError, add_one},
    workspace::Workspace,
};

/// Represents sources of errors that can occur during fragment creation.
#[derive(Debug, Error)]
#[error(transparent)]
pub enum Error {
    /// Check errors.
    Check(#[from] CheckError),
    /// Write errors.
    Write(#[from] WriteError),
    /// Edit errors.
    Edit(#[from] EditError),
    /// `git` errors.
    Git(#[from] GitError),
}

/// Creates changelog fragments.
///
/// # Errors
///
/// Returns [`struct@Error`] if parsing the fragment name, creating the fragment file
/// and writing to it fails. Also returned if starting the default editor fails.
#[builder]
#[allow(clippy::should_implement_trait)] // `add` confused
pub fn create<G: AsRef<Path>, S: AsRef<NonEmptyStr>, C: AsRef<NonEmptyStr>>(
    git: G,
    workspace: Workspace<'_>,
    name: S,
    content: Option<C>,
    edit: bool,
    add: bool,
) -> Result<(), Error> {
    let non_empty = name.as_ref();

    check_non_empty(non_empty)?;

    let path = workspace.config.paths.directory_path();

    let joined = path.join(non_empty);

    if let Some(string) = content {
        write_non_empty(joined.as_path(), string)?;
    } else {
        write_non_empty(joined.as_path(), PLACEHOLDER)?;
    }

    if edit {
        edit_path(joined.as_path())?;
    }

    if add {
        add_one(git, joined.as_path())?;
    }

    Ok(())
}

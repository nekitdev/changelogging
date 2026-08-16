//! `git` functionality.

use std::{
    io::Error as IoError,
    iter::once,
    path::Path,
    process::{Command, ExitStatus},
};

use thiserror::Error;

/// Represents `git` errors.
#[derive(Debug, Error)]
#[error("git error: {error}")]
pub struct Error {
    #[from]
    error: IoError,
}

impl Error {
    /// Constructs [`Self`].
    pub const fn new(error: IoError) -> Self {
        Self { error }
    }

    /// Returns the underlying I/O error.
    pub fn get(self) -> IoError {
        self.error
    }
}

/// The `git` command.
pub const GIT: &str = "git";

/// The `add` subcommand.
pub const ADD: &str = "add";

/// The `rm` (remove) subcommand.
pub const REMOVE: &str = "rm";

/// The `-f` (force) flag.
pub const FORCE: &str = "-f";

/// The `-q` (quiet) flag.
pub const QUIET: &str = "-q";

/// Adds paths from the provided iterable via `git add`.
///
/// # Errors
///
/// Returns [`struct@Error`] when the command fails to execute.
pub fn add<G: AsRef<Path>, P: AsRef<Path>, I: IntoIterator<Item = P>>(
    git: G,
    iterable: I,
) -> Result<ExitStatus, Error> {
    let mut command = Command::new(git.as_ref());

    command.arg(ADD);

    iterable.into_iter().for_each(|path| {
        command.arg(path.as_ref());
    });

    let status = command.status()?;

    Ok(status)
}

pub fn add_one<G: AsRef<Path>, P: AsRef<Path>>(git: G, path: P) -> Result<ExitStatus, Error> {
    add(git, once(path))
}

/// Removes paths from the provided iterable via `git rm`, forcefully and quietly.
///
/// # Errors
///
/// Returns [`struct@Error`] when the command fails to execute.
pub fn remove<G: AsRef<Path>, P: AsRef<Path>, I: IntoIterator<Item = P>>(
    git: G,
    iterable: I,
) -> Result<ExitStatus, Error> {
    let mut command = Command::new(git.as_ref());

    command.arg(REMOVE).arg(FORCE).arg(QUIET);

    iterable.into_iter().for_each(|path| {
        command.arg(path.as_ref());
    });

    let status = command.status()?;

    Ok(status)
}

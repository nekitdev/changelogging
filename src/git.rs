//! `git` functionality.

use std::{
    path::Path,
    process::{Command, ExitStatus},
};

use miette::Diagnostic;
use thiserror::Error;

/// Represents `git` errors.
#[derive(Debug, Error, Diagnostic)]
#[error("git error")]
#[diagnostic(code(changelogging::git), help("make sure git is present"))]
pub struct Error(#[from] pub std::io::Error);

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

/// Adds paths from the provided iterator via `git add`.
///
/// # Errors
///
/// Returns [`struct@Error`] when the command fails to execute.
pub fn add<P: AsRef<Path>, I: IntoIterator<Item = P>>(iterator: I) -> Result<ExitStatus, Error> {
    let mut command = Command::new(GIT);

    command.arg(ADD);

    for path in iterator {
        command.arg(path.as_ref());
    }

    command.status().map_err(Into::into)
}

/// Removes paths from the provided iterator via `git rm`, forcefully and quietly.
///
/// # Errors
///
/// Returns [`struct@Error`] when the command fails to execute.
pub fn remove<P: AsRef<Path>, I: IntoIterator<Item = P>>(iterator: I) -> Result<ExitStatus, Error> {
    let mut command = Command::new(GIT);

    command.arg(REMOVE).arg(FORCE).arg(QUIET);

    for path in iterator {
        command.arg(path.as_ref());
    }

    command.status().map_err(Into::into)
}

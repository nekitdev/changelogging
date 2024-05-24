//! Creating changelog fragments.
//!
//! The [`create`] function implements the `create` subcommand.

use std::{fs::File, io::Write, path::Path};

use edit::edit_file;
use thiserror::Error;

use crate::fragments::{validate, ParseError};

/// Represents errors that can occur during [`create`] runs.
#[derive(Debug, Error)]
#[error(transparent)]
pub enum Error {
    /// Create error.
    Create(#[from] std::io::Error),
    /// Parse error.
    Parse(#[from] ParseError),
}

const PLACEHOLDER: &str = "Add the fragment content here.";

/// Creates changelog fragments.
pub fn create<D: AsRef<Path>, S: AsRef<str>, C: AsRef<str>>(
    directory: D,
    name: S,
    content: Option<C>,
    edit: bool,
) -> Result<(), Error> {
    validate(name.as_ref())?;

    let path = directory.as_ref().join(name.as_ref());

    let mut file = File::options()
        .create_new(true)
        .write(true)
        .open(path.as_path())?;

    let string = content.as_ref().map_or(PLACEHOLDER, |slice| slice.as_ref());

    writeln!(file, "{string}")?;

    if edit {
        edit_file(path.as_path())?;
    }

    Ok(())
}

//! Creating changelog fragments.
//!
//! The [`create`] function implements the `create` subcommand.

use std::{
    fs::File,
    io::Write,
    iter::once,
    path::{Path, PathBuf},
};

use edit::edit_file;
use miette::Diagnostic;
use thiserror::Error;

use crate::{
    fragment::{validate, ParseError},
    git,
};

/// Represents errors that can occur when opening files.
#[derive(Debug, Error, Diagnostic)]
#[error("opening failed")]
#[diagnostic(
    code(changelogging::create::open),
    help("check that the file does not already exist and the fragments directory is accessible")
)]
pub struct OpenError(#[from] pub std::io::Error);

/// Represents errors that can occur when writing to files.
#[derive(Debug, Error, Diagnostic)]
#[error("writing failed")]
#[diagnostic(
    code(changelogging::create::write),
    help("make sure the fragment file is accessible")
)]
pub struct WriteError(#[from] pub std::io::Error);

/// Represents errors that can occur when starting default editors.
#[derive(Debug, Error, Diagnostic)]
#[error("editing failed")]
#[diagnostic(
    code(changelogging::create::edit),
    help("check your default editor configuration")
)]
pub struct EditError(#[from] pub std::io::Error);

/// Represents sources of errors that can occur during fragment creation.
#[derive(Debug, Error, Diagnostic)]
#[error(transparent)]
#[diagnostic(transparent)]
pub enum ErrorSource {
    /// Parse errors.
    Parse(#[from] ParseError),
    /// Open errors.
    Open(#[from] OpenError),
    /// Write errors.
    Write(#[from] WriteError),
    /// Edit errors.
    Edit(#[from] EditError),
    /// `git` errors.
    Git(#[from] crate::git::Error),
}

/// Represents errors that can occur during fragment creation.
#[derive(Debug, Error, Diagnostic)]
#[error("failed to create fragment `{path}`")]
#[diagnostic(
    code(changelogging::create::create),
    help("see the report for more information")
)]
pub struct Error {
    /// The source of this error.
    #[source]
    #[diagnostic_source]
    pub source: ErrorSource,
    /// The path provided.
    pub path: PathBuf,
}

impl Error {
    /// Constructs [`Self`].
    pub fn new(source: ErrorSource, path: PathBuf) -> Self {
        Self { source, path }
    }

    /// Constructs [`Self`] from [`ParseError`].
    pub fn parse(error: ParseError, path: PathBuf) -> Self {
        Self::new(error.into(), path)
    }

    /// Constructs [`Self`] from [`OpenError`].
    pub fn open(error: OpenError, path: PathBuf) -> Self {
        Self::new(error.into(), path)
    }

    /// Constructs [`Self`] from [`WriteError`].
    pub fn write(error: WriteError, path: PathBuf) -> Self {
        Self::new(error.into(), path)
    }

    /// Constructs [`Self`] from [`EditError`].
    pub fn edit(error: EditError, path: PathBuf) -> Self {
        Self::new(error.into(), path)
    }

    /// Constructs [`Self`] from [`Error`].
    ///
    /// [`Error`]: crate::git::Error
    pub fn git(error: crate::git::Error, path: PathBuf) -> Self {
        Self::new(error.into(), path)
    }

    /// Constructs [`OpenError`] and constructs [`Self`] from it.
    pub fn new_open(error: std::io::Error, path: PathBuf) -> Self {
        Self::open(OpenError(error), path)
    }

    /// Constructs [`WriteError`] and constructs [`Self`] from it.
    pub fn new_write(error: std::io::Error, path: PathBuf) -> Self {
        Self::write(WriteError(error), path)
    }

    /// Constructs [`EditError`] and constructs [`Self`] from it.
    pub fn new_edit(error: std::io::Error, path: PathBuf) -> Self {
        Self::edit(EditError(error), path)
    }
}

/// The placeholder that gets written to fragment files if contents are not provided.
pub const PLACEHOLDER: &str = "Add the fragment content here.";

/// Creates changelog fragments.
///
/// # Errors
///
/// Returns [`struct@Error`] if parsing the fragment name, creating the fragment file
/// and writing to it fails. Also returned if starting the default editor fails.
pub fn create<D: AsRef<Path>, S: AsRef<str>, C: AsRef<str>>(
    directory: D,
    name: S,
    content: Option<C>,
    edit: bool,
    add: bool,
) -> Result<(), Error> {
    let name = name.as_ref();

    let path = directory.as_ref().join(name);

    validate(name).map_err(|error| Error::parse(error, path.clone()))?;

    let mut file = File::options()
        .create_new(true)
        .write(true)
        .open(&path)
        .map_err(|error| Error::new_open(error, path.clone()))?;

    let string = content.as_ref().map_or(PLACEHOLDER, |slice| slice.as_ref());

    writeln!(file, "{string}").map_err(|error| Error::new_write(error, path.clone()))?;

    if edit {
        edit_file(&path).map_err(|error| Error::new_edit(error, path.clone()))?;
    }

    if add {
        git::add(once(&path)).map_err(|error| Error::git(error, path.clone()))?;
    }

    Ok(())
}

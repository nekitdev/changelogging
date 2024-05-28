//! Discovering workspaces.

use std::{
    env::current_dir,
    path::{Path, PathBuf},
};

use miette::Diagnostic;
use thiserror::Error;

use crate::workspace::{PyProject, Workspace};

/// Represents errors that can occur when fetching the current directory fails.
#[derive(Debug, Error, Diagnostic)]
#[error("failed to fetch current directory")]
#[diagnostic(
    code(changelogging::discover::current_directory),
    help("check whether the current directory is accessible")
)]
pub struct CurrentDirectoryError(#[from] pub std::io::Error);

/// Represents errors that can occur when checking the existence of paths fails.
#[derive(Debug, Error, Diagnostic)]
#[error("failed to check existence of `{path}`")]
#[diagnostic(
    code(changelogging::discover::existence),
    help("check whether the current directory is accessible")
)]
pub struct ExistenceError {
    /// The underlying I/O error.
    pub source: std::io::Error,
    /// The path provided.
    pub path: PathBuf,
}

impl ExistenceError {
    /// Constructs [`Self`].
    pub fn new<P: AsRef<Path>>(source: std::io::Error, path: P) -> Self {
        let path = path.as_ref().to_owned();

        Self { source, path }
    }
}

/// Represents errors that can occur when workspaces are absent from the current directory.
#[derive(Debug, Error, Diagnostic)]
#[error("workspace not found in `{directory}`")]
#[diagnostic(
    code(changelogging::discover::not_found),
    help("workspaces must contain `{CHANGELOGGING}` or `{PYPROJECT}`")
)]
pub struct NotFoundError {
    /// The current directory.
    pub directory: PathBuf,
}

impl NotFoundError {
    /// Constructs [`Self`].
    pub fn new<D: AsRef<Path>>(directory: D) -> Self {
        let directory = directory.as_ref().to_owned();

        Self { directory }
    }
}

/// Represents sources of errors that can occur when discovering workspaces.
#[derive(Debug, Error, Diagnostic)]
#[error(transparent)]
#[diagnostic(transparent)]
pub enum ErrorSource {
    /// Current directory fetching errors.
    CurrentDirectory(#[from] CurrentDirectoryError),
    /// Existence checking errors.
    Existence(#[from] ExistenceError),
    /// Workspace loading errors.
    Workspace(#[from] crate::workspace::Error),
    /// Workspace not found errors.
    NotFound(#[from] NotFoundError),
}

/// Represents errors that can occur when discovering workspaces.
#[derive(Debug, Error, Diagnostic)]
#[error("failed to discover workspace")]
#[diagnostic(
    code(changelogging::discover::discover),
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

    /// Constructs [`Self`] from [`CurrentDirectoryError`].
    pub fn current_directory(source: CurrentDirectoryError) -> Self {
        Self::new(source.into())
    }

    /// Constructs [`Self`] from [`ExistenceError`].
    pub fn existence(source: ExistenceError) -> Self {
        Self::new(source.into())
    }

    /// Constructs [`Self`] from [`Error`].
    ///
    /// [`Error`]: crate::workspace::Error
    pub fn workspace(source: crate::workspace::Error) -> Self {
        Self::new(source.into())
    }

    /// Constructs [`Self`] from [`NotFoundError`].
    pub fn not_found(source: NotFoundError) -> Self {
        Self::new(source.into())
    }

    /// Constructs [`CurrentDirectoryError`] and constructs [`Self`] from it.
    pub fn new_current_directory(source: std::io::Error) -> Self {
        Self::current_directory(CurrentDirectoryError(source))
    }

    /// Constructs [`ExistenceError`] and constructs [`Self`] from it.
    pub fn new_existence<P: AsRef<Path>>(source: std::io::Error, path: P) -> Self {
        Self::existence(ExistenceError::new(source, path))
    }

    /// Constructs [`NotFoundError`] and constructs [`Self`] from it.
    pub fn new_not_found<D: AsRef<Path>>(directory: D) -> Self {
        Self::not_found(NotFoundError::new(directory))
    }
}

/// The `changelogging.toml` literal.
pub const CHANGELOGGING: &str = "changelogging.toml";

/// The `pyproject.toml` literal.
pub const PYPROJECT: &str = "pyproject.toml";

/// Discovers workspaces in the current directory.
///
/// This function looks for [`CHANGELOGGING`] as well as for [`PYPROJECT`]
/// (if it defines `tool.changelogging` section) in the current directory.
///
/// If both files are present, the former takes precedence.
///
/// # Errors
///
/// Returns [`struct@Error`] if fetching the current directory, checking the existence
/// or loading the workspace fails. Also returned when no workspace can be found.
pub fn discover() -> Result<Workspace<'static>, Error> {
    let mut path = current_dir().map_err(|error| Error::new_current_directory(error))?;

    // try `changelogging.toml`

    path.push(CHANGELOGGING);

    if path
        .try_exists()
        .map_err(|error| Error::new_existence(error, path.as_path()))?
    {
        let workspace = Workspace::load(path.as_path()).map_err(|error| Error::workspace(error))?;

        return Ok(workspace);
    }

    // try `pyproject.toml` if it contains `tool.changelogging`

    path.pop();

    path.push(PYPROJECT);

    if path
        .try_exists()
        .map_err(|error| Error::new_existence(error, path.as_path()))?
    {
        let pyproject = PyProject::load(path.as_path()).map_err(|error| Error::workspace(error))?;

        if let Some(workspace) = pyproject.tool.and_then(|tools| tools.changelogging) {
            return Ok(workspace);
        }
    }

    // not found

    path.pop();

    Err(Error::new_not_found(path))
}

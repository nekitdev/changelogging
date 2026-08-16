//! Discovering workspaces.

use std::{
    env::current_dir,
    io::Error as IoError,
    path::{Path, PathBuf},
};

use from_path::Load;
use thiserror::Error;

use crate::{
    toml::load::Error as LoadError,
    workspace::{OwnedWorkspace, PyProject, Workspace},
};

/// Represents errors that can occur when fetching the current directory fails.
#[derive(Debug, Error)]
#[error("failed to fetch current directory: {error}")]
pub struct CurrentDirError {
    #[from]
    error: IoError,
}

impl CurrentDirError {
    pub const fn new(error: IoError) -> Self {
        Self { error }
    }

    pub fn get(self) -> IoError {
        self.error
    }
}

/// Represents errors that can occur when checking whether paths exist fails.
#[derive(Debug, Error)]
#[error("failed to check whether `{path}` exists: {error}")]
pub struct ExistsError {
    /// The underlying I/O error.
    pub error: IoError,
    /// The path provided.
    pub path: PathBuf,
}

impl ExistsError {
    /// Constructs [`Self`].
    pub const fn new(error: IoError, path: PathBuf) -> Self {
        Self { error, path }
    }

    pub fn get(self) -> (IoError, PathBuf) {
        (self.error, self.path)
    }
}

/// Represents errors that can occur when workspaces are absent from the current directory.
#[derive(Debug, Error)]
#[error("workspace not found in `{directory}` (expected `{CHANGELOGGING}` or `{PY_PROJECT}`)")]
pub struct NotFoundError {
    /// The current directory.
    pub directory: PathBuf,
}

impl NotFoundError {
    /// Constructs [`Self`].
    pub const fn new(directory: PathBuf) -> Self {
        Self { directory }
    }

    pub fn get(self) -> PathBuf {
        self.directory
    }
}

/// Represents sources of errors that can occur when discovering workspaces.
#[derive(Debug, Error)]
#[error("failed to discover workspace")]
pub enum Error {
    /// Current directory fetching errors.
    CurrentDir(#[from] CurrentDirError),
    /// Existence checking errors.
    Exists(#[from] ExistsError),
    /// Workspace loading errors.
    Workspace(#[from] LoadError),
    /// Workspace not found errors.
    NotFound(#[from] NotFoundError),
}

/// The `changelogging.toml` literal.
pub const CHANGELOGGING: &str = "changelogging.toml";

/// The `pyproject.toml` literal.
pub const PY_PROJECT: &str = "pyproject.toml";

pub fn start() -> Result<PathBuf, CurrentDirError> {
    let dir = current_dir()?;

    Ok(dir)
}

pub fn check_exists<P: AsRef<Path>>(path: P) -> Result<bool, ExistsError> {
    fn check_exists_inner(path: &Path) -> Result<bool, ExistsError> {
        path.try_exists()
            .map_err(|error| ExistsError::new(error, path.to_owned()))
    }

    check_exists_inner(path.as_ref())
}

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
pub fn discover() -> Result<OwnedWorkspace, Error> {
    let mut current = start()?;

    // try `changelogging.toml`

    current.push(CHANGELOGGING);

    if check_exists(current.as_path())? {
        let workspace: Workspace<'_> = current.load()?;

        return Ok(workspace);
    }

    // try `pyproject.toml` if it contains `tool.changelogging`

    current.pop();

    current.push(PY_PROJECT);

    if check_exists(current.as_path())? {
        let pyproject: PyProject<'_> = current.load()?;

        if let Some(workspace) = pyproject.into_workspace() {
            return Ok(workspace);
        }
    }

    // not found

    current.pop();

    let not_found_error = NotFoundError::new(current);

    Err(not_found_error.into())
}

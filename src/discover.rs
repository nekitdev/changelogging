//! Discovering workspaces.

use std::{
    env::current_dir,
    path::{Path, PathBuf},
};

use miette::Diagnostic;
use thiserror::Error;

use crate::workspace::{PyProject, Workspace};

#[derive(Debug, Error, Diagnostic)]
#[error("failed to fetch current directory")]
#[diagnostic(
    code(changelogging::discover::current_directory),
    help("check whether the current directory is accessible")
)]
pub struct CurrentDirectoryError(#[from] pub std::io::Error);

#[derive(Debug, Error, Diagnostic)]
#[error("failed to check existence of `{path}`")]
#[diagnostic(
    code(changelogging::discover::existence),
    help("check whether the current directory is accessible")
)]
pub struct ExistenceError {
    pub source: std::io::Error,
    pub path: PathBuf,
}

impl ExistenceError {
    pub fn new<P: AsRef<Path>>(source: std::io::Error, path: P) -> Self {
        let path = path.as_ref().to_owned();

        Self { source, path }
    }
}

#[derive(Debug, Error, Diagnostic)]
#[error("workspace not found in `{directory}`")]
#[diagnostic(
    code(changelogging::discover::not_found),
    help("workspaces must contain `{CHANGELOGGING}` or `{PYPROJECT}`")
)]
pub struct NotFoundError {
    directory: PathBuf,
}

impl NotFoundError {
    pub fn new<D: AsRef<Path>>(directory: D) -> Self {
        let directory = directory.as_ref().to_owned();

        Self { directory }
    }
}

#[derive(Debug, Error, Diagnostic)]
#[error(transparent)]
#[diagnostic(transparent)]
pub enum ErrorSource {
    CurrentDirectory(#[from] CurrentDirectoryError),
    Existence(#[from] ExistenceError),
    Workspace(#[from] crate::workspace::Error),
    NotFound(#[from] NotFoundError),
}

#[derive(Debug, Error, Diagnostic)]
#[error("failed to discover workspace")]
#[diagnostic(
    code(changelogging::discover::discover),
    help("see the report for more information")
)]
pub struct Error {
    #[source]
    #[diagnostic_source]
    source: ErrorSource,
}

impl Error {
    pub fn new(source: ErrorSource) -> Self {
        Self { source }
    }

    pub fn current_directory(source: CurrentDirectoryError) -> Self {
        Self::new(source.into())
    }

    pub fn existence(source: ExistenceError) -> Self {
        Self::new(source.into())
    }

    pub fn workspace(source: crate::workspace::Error) -> Self {
        Self::new(source.into())
    }

    pub fn not_found(source: NotFoundError) -> Self {
        Self::new(source.into())
    }

    pub fn new_current_directory(source: std::io::Error) -> Self {
        Self::current_directory(CurrentDirectoryError(source))
    }

    pub fn new_existence<P: AsRef<Path>>(source: std::io::Error, path: P) -> Self {
        Self::existence(ExistenceError::new(source, path))
    }

    pub fn new_not_found<D: AsRef<Path>>(directory: D) -> Self {
        Self::not_found(NotFoundError::new(directory))
    }
}

const CHANGELOGGING: &str = "changelogging.toml";
const PYPROJECT: &str = "pyproject.toml";

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

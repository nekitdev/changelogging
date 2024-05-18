use std::{
    env::current_dir,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{
    context::Context,
    macros::{impl_from_path_with_parse, impl_from_str_with_toml},
    options::Options,
    paths::{load, load_if_exists},
};

/// Combines [`Context`] and [`Options`] into one structure.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Workspace<'w> {
    /// The context of the workspace.
    pub context: Context<'w>,
    /// The options of the workspace.
    #[serde(flatten)]
    pub options: Options<'w>,
}

trait IntoWorkspace<'w> {
    fn into_workspace(self) -> Option<Workspace<'w>>;
}

impl_from_str_with_toml!(Workspace<'_>);
impl_from_path_with_parse!(Workspace<'_>, crate::config::Error);

impl<'w> IntoWorkspace<'w> for Workspace<'w> {
    fn into_workspace(self) -> Option<Workspace<'w>> {
        Some(self)
    }
}

/// Represents `tool` sections in `pyproject.toml` files.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Tools<'t> {
    /// The `changelogging` section.
    pub changelogging: Option<Workspace<'t>>,
}

impl<'t> IntoWorkspace<'t> for Tools<'t> {
    fn into_workspace(self) -> Option<Workspace<'t>> {
        self.changelogging
    }
}

impl_from_str_with_toml!(Tools<'_>);
impl_from_path_with_parse!(Tools<'_>, crate::config::Error);

/// Represents structures of `pyproject.toml` files.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PyProject<'p> {
    pub tool: Option<Tools<'p>>,
}

impl<'p> IntoWorkspace<'p> for PyProject<'p> {
    fn into_workspace(self) -> Option<Workspace<'p>> {
        self.tool.and_then(|tools| tools.into_workspace())
    }
}

impl_from_str_with_toml!(PyProject<'_>);
impl_from_path_with_parse!(PyProject<'_>, crate::config::Error);

const CHANGELOGGING: &str = "changelogging.toml";
const PYPROJECT: &str = "pyproject.toml";

/// Represents discovery errors.
#[derive(Debug, Error)]
#[error("failed to discover workspace in {path}")]
pub struct DiscoverError {
    /// The path where discovering failed.
    path: PathBuf,
}

impl DiscoverError {
    /// Constructs [`DiscoverError`] from [`PathBuf`].
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }
}

/// Represents [`discover`] and [`workspace`] errors.
#[derive(Debug, Error)]
#[error(transparent)]
pub enum Error {
    /// I/O error.
    Io(#[from] std::io::Error),
    /// Config error.
    Config(#[from] crate::config::Error),
    /// Discovery error.
    Discover(#[from] DiscoverError),
}

pub fn workspace<P: AsRef<Path>>(path: P) -> Result<Workspace<'static>, Error> {
    let workspace = load(path)?;

    Ok(workspace)
}

/// Discovers the workspace in the current directory.
///
/// # Errors
///
/// [`enum@Error`] is returned on I/O errors (from [`current_dir`]),
/// config errors (from [`load_if_exists`]) and when workspace can not be discovered.
pub fn discover() -> Result<Workspace<'static>, Error> {
    let mut path = current_dir()?;

    // try `changelogging.toml`

    path.push(CHANGELOGGING);

    if let Some(workspace) = load_if_exists(path.as_path())? {
        return Ok(workspace);
    }

    // try `pyproject.toml` iff `[tool.changelogging]` is in there

    path.pop();

    path.push(PYPROJECT);

    let pyproject_option: Option<PyProject<'_>> = load_if_exists(path.as_path())?;

    if let Some(workspace) = pyproject_option.and_then(|pyproject| pyproject.into_workspace()) {
        return Ok(workspace);
    }

    // not found

    path.pop();

    let error = DiscoverError::new(path);

    Err(error.into())
}

//! Discovering and loading workspaces.
//!
//! This module provides two notable functions, [`workspace`] and [`discover`], as well as
//! the [`Workspace`] structure.
//!
//! See also [`context`] and [`options`].
//!
//! [`context`]: crate::context
//! [`options`]: crate::options

use std::{env::current_dir, path::Path};

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
    ///
    /// This field is flattened during (de)serialization.
    #[serde(flatten)]
    pub options: Options<'w>,
}

impl<'w> Workspace<'w> {
    /// Creates [`Workspace`] from [`Context`] and [`Options`] provided.
    pub fn new(context: Context<'w>, options: Options<'w>) -> Self {
        Self { context, options }
    }
}

impl_from_str_with_toml!(Workspace<'_>);
impl_from_path_with_parse!(Workspace<'_>, crate::config::Error);

trait IntoWorkspace<'w> {
    fn into_workspace(self) -> Option<Workspace<'w>>;
}

/// Represents `tool` sections in `pyproject.toml` files.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Tools<'t> {
    /// The `changelogging` section.
    pub changelogging: Option<Workspace<'t>>,
}

impl_from_str_with_toml!(Tools<'_>);
impl_from_path_with_parse!(Tools<'_>, crate::config::Error);

/// Represents structures of `pyproject.toml` files.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PyProject<'p> {
    /// The `tool` section.
    pub tool: Option<Tools<'p>>,
}

impl_from_str_with_toml!(PyProject<'_>);
impl_from_path_with_parse!(PyProject<'_>, crate::config::Error);

const CHANGELOGGING: &str = "changelogging.toml";
const PYPROJECT: &str = "pyproject.toml";

/// Represents discovery errors.
#[derive(Debug, Error)]
#[error("failed to discover workspace")]
pub struct DiscoverError;

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

/// Loads the workspace from the given `path`.
///
/// # Errors
///
/// Returns [`enum@Error`] on loading errors.
pub fn workspace<P: AsRef<Path>>(path: P) -> Result<Workspace<'static>, Error> {
    let workspace = load(path)?;

    Ok(workspace)
}

/// Discovers the workspace in the current directory.
///
/// # Errors
///
/// [`enum@Error`] is returned on I/O errors (from [`current_dir`]),
/// loading errors (from [`load_if_exists`]) and when workspace can not be discovered.
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

    let option: Option<PyProject<'_>> = load_if_exists(path.as_path())?;

    if let Some(workspace) = option
        .and_then(|pyproject| pyproject.tool)
        .and_then(|tools| tools.changelogging)
    {
        return Ok(workspace);
    }

    // not found

    path.pop();

    Err(DiscoverError.into())
}

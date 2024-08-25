//! Discovering and loading workspaces.
//!
//! This module provides two notable structures, [`Workspace`] and [`PyProject`].
//!
//! See also [`context`] and [`config`].
//!
//! [`context`]: crate::context
//! [`config`]: crate::config

use std::{
    fs::read_to_string,
    path::{Path, PathBuf},
};

use miette::Diagnostic;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{config::Config, context::Context, load::Load};

/// Represents errors that can occur when reading files.
#[derive(Debug, Error, Diagnostic)]
#[error("read failed")]
#[diagnostic(
    code(changelogging::workspace::read),
    help("check that the file exists and is accessible")
)]
pub struct ReadError(#[from] pub std::io::Error);

/// Represents errors that can occur when parsing TOML configuration into concrete types.
#[derive(Debug, Error, Diagnostic)]
#[error("parsing failed")]
#[diagnostic(
    code(changelogging::workspace::parse),
    help("check that the configuration is correct")
)]
pub struct ParseError(#[from] pub toml::de::Error);

/// Represents sources of errors that can occur when loading workspaces.
#[derive(Debug, Error, Diagnostic)]
#[error(transparent)]
#[diagnostic(transparent)]
pub enum ErrorSource {
    /// Read errors.
    Read(#[from] ReadError),
    /// Parse errors.
    Parse(#[from] ParseError),
}

/// Represents errors that can occur during workspace loading.
#[derive(Debug, Error, Diagnostic)]
#[error("loading workspace from `{path}` failed")]
#[diagnostic(
    code(changelogging::workspace::load),
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

    /// Constructs [`Self`] from [`ReadError`].
    pub fn read(source: ReadError, path: PathBuf) -> Self {
        Self::new(source.into(), path)
    }

    /// Constructs [`Self`] from [`ParseError`].
    pub fn parse(source: ParseError, path: PathBuf) -> Self {
        Self::new(source.into(), path)
    }

    /// Constructs [`ReadError`] and constructs [`Self`] from it.
    pub fn new_read(source: std::io::Error, path: PathBuf) -> Self {
        Self::read(ReadError(source), path)
    }

    /// Constructs [`ParseError`] and constructs [`Self`] from it.
    pub fn new_parse(source: toml::de::Error, path: PathBuf) -> Self {
        Self::parse(ParseError(source), path)
    }
}

/// Combines [`Context`] and [`Config`] into one structure.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Workspace<'w> {
    /// The context of the workspace.
    pub context: Context<'w>,
    /// The config of the workspace.
    ///
    /// This field is flattened during (de)serialization.
    #[serde(flatten)]
    pub config: Config<'w>,
}

impl<'w> Workspace<'w> {
    /// Constructs [`Self`].
    pub fn new(context: Context<'w>, config: Config<'w>) -> Self {
        Self { context, config }
    }
}

impl Load for Workspace<'_> {
    type Error = Error;

    fn load<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let path = path.as_ref();

        let string =
            read_to_string(path).map_err(|error| Error::new_read(error, path.to_owned()))?;

        let workspace =
            toml::from_str(&string).map_err(|error| Error::new_parse(error, path.to_owned()))?;

        Ok(workspace)
    }
}

/// Represents `tool` sections in `pyproject.toml` files.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Tools<'t> {
    /// The `changelogging` section.
    pub changelogging: Option<Workspace<'t>>,
}

/// Represents structures of `pyproject.toml` files.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PyProject<'p> {
    /// The `tool` section.
    pub tool: Option<Tools<'p>>,
}

impl Load for PyProject<'_> {
    type Error = Error;

    fn load<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let path = path.as_ref();

        let string =
            read_to_string(path).map_err(|error| Error::new_read(error, path.to_owned()))?;

        let workspace =
            toml::from_str(&string).map_err(|error| Error::new_parse(error, path.to_owned()))?;

        Ok(workspace)
    }
}

impl<'p> PyProject<'p> {
    /// Converts [`Self`] to [`Workspace`], provided that the `tool.changelogging` table is present.
    pub fn into_workspace(self) -> Option<Workspace<'p>> {
        self.tool.and_then(|tools| tools.changelogging)
    }
}

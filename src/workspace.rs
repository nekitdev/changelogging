//! Discovering and loading workspaces.
//!
//! This module provides two notable functions, [`workspace`] and [`discover`], as well as
//! the [`Workspace`] structure.
//!
//! See also [`context`] and [`options`].
//!
//! [`context`]: crate::context
//! [`options`]: crate::options

use std::{
    fs::read_to_string,
    path::{Path, PathBuf},
};

use miette::Diagnostic;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{context::Context, options::Options};

#[derive(Debug, Error, Diagnostic)]
#[error("read failed")]
#[diagnostic(
    code(changelogging::workspace::read),
    help("check that the file exists and is accessible")
)]
pub struct ReadError(#[from] pub std::io::Error);

#[derive(Debug, Error, Diagnostic)]
#[error("parsing failed")]
#[diagnostic(
    code(changelogging::workspace::parse),
    help("check that the configuration is correct")
)]
pub struct ParseError(#[from] pub toml::de::Error);

#[derive(Debug, Error, Diagnostic)]
#[error(transparent)]
#[diagnostic(transparent)]
pub enum ErrorSource {
    Read(#[from] ReadError),
    Parse(#[from] ParseError),
}

/// Represents errors that can occur during configuration loading.
#[derive(Debug, Error, Diagnostic)]
#[error("loading workspace from `{path}` failed")]
#[diagnostic(
    code(changelogging::workspace::load),
    help("see the report for more information")
)]
pub struct Error {
    #[source]
    #[diagnostic_source]
    pub source: ErrorSource,
    pub path: PathBuf,
}

impl Error {
    pub fn new<P: AsRef<Path>>(source: ErrorSource, path: P) -> Self {
        let path = path.as_ref().to_owned();

        Self { source, path }
    }

    pub fn read<P: AsRef<Path>>(source: ReadError, path: P) -> Self {
        Self::new(source.into(), path)
    }

    pub fn parse<P: AsRef<Path>>(source: ParseError, path: P) -> Self {
        Self::new(source.into(), path)
    }

    pub fn new_read<P: AsRef<Path>>(source: std::io::Error, path: P) -> Self {
        Self::read(ReadError(source), path)
    }

    pub fn new_parse<P: AsRef<Path>>(source: toml::de::Error, path: P) -> Self {
        Self::parse(ParseError(source), path)
    }
}

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
    /// Constructs [`Self`].
    pub fn new(context: Context<'w>, options: Options<'w>) -> Self {
        Self { context, options }
    }
}

impl Workspace<'_> {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let path = path.as_ref();

        let string = read_to_string(path).map_err(|error| Error::new_read(error, path))?;

        let workspace =
            toml::from_str(string.as_ref()).map_err(|error| Error::new_parse(error, path))?;

        Ok(workspace)
    }
}

pub fn load<P: AsRef<Path>>(path: P) -> Result<Workspace<'static>, Error> {
    Workspace::load(path)
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

impl PyProject<'_> {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let path = path.as_ref();

        let string = read_to_string(path).map_err(|error| Error::new_read(error, path))?;

        let workspace =
            toml::from_str(string.as_ref()).map_err(|error| Error::new_parse(error, path))?;

        Ok(workspace)
    }
}

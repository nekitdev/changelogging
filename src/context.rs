//! Context.
//!
//! The `context` section provides information about the project to `changelogging`.
//!
//! It is always required, and the fields are as follows:
//!
//! - `name` is the name of the project;
//! - `version` is the version of the project;
//! - `url` is the URL of the project.
//!
//! Here is an example of this section:
//!
//! ```toml
//! [context]
//! name = "changelogging"
//! version = "0.5.0"
//! url = "https://github.com/nekitdev/changelogging"
//! ```
//!
//! This section is represented by the [`Context`] structure.

use std::borrow::Cow;

use serde::{Deserialize, Serialize};

/// Represents project contexts.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Context<'c> {
    /// The name of the project.
    pub name: Cow<'c, str>,
    /// The version of the project.
    pub version: Cow<'c, str>,
    /// The URL of the project.
    pub url: Cow<'c, str>,
}

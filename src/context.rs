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
//! version = "0.8.0"
//! url = "https://github.com/nekitdev/changelogging"
//! ```
//!
//! This section is represented by the [`Context`] structure.

use bon::Builder;
use non_empty_str::NonEmptyCowStr;
use ownership::IntoOwned;
use serde::{Deserialize, Serialize};

/// Represents project contexts.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Builder, IntoOwned)]
pub struct Context<'c> {
    /// The name of the project.
    pub name: NonEmptyCowStr<'c>,
    /// The version of the project.
    pub version: NonEmptyCowStr<'c>,
    /// The URL of the project.
    pub url: NonEmptyCowStr<'c>,
}

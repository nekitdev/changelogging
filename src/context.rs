//! Project contexts.
//!
//! This module provides the [`Context`] structure that represents contexts of projects.

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

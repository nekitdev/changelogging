//! The `indents` section.
//!
//! This module provides the [`Indents`] structure and its default values.
//!
//! See also [`config`].
//!
//! [`config`]: crate::config

use bon::Builder;
use ownership::IntoOwned;
use serde::{Deserialize, Serialize};

use crate::internals::impl_default_via_builder_build;

/// The default `indents.heading` value.
pub const DEFAULT_HEADING: char = '#';

/// The default `indents.bullet` value.
pub const DEFAULT_BULLET: char = '-';

/// Specifies characters to use for headings and indentation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Builder, IntoOwned)]
#[serde(default)]
pub struct Indents {
    /// The character to use for headings.
    #[builder(default = DEFAULT_HEADING)]
    pub heading: char,

    /// The character to use for indentation.
    #[builder(default = DEFAULT_BULLET)]
    pub bullet: char,
}

impl Indents {
    pub const fn heading(&self) -> char {
        self.heading
    }

    pub const fn bullet(&self) -> char {
        self.bullet
    }
}

impl_default_via_builder_build!(Indents);

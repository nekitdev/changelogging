//! The `levels` section.
//!
//! This module provides the [`Levels`] structure and its default values.
//!
//! See also [`config`].
//!
//! [`config`]: crate::config

use bon::Builder;
use non_zero_size::{Size, const_size};
use ownership::IntoOwned;
use serde::{Deserialize, Serialize};

use crate::internals::impl_default_via_builder_build;

/// The default `levels.entry` value.
pub const DEFAULT_ENTRY: Size = const_size!(2);

/// The default `levels.section` value.
pub const DEFAULT_SECTION: Size = const_size!(3);

/// Defines which heading levels to use.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Builder, IntoOwned)]
#[serde(default)]
pub struct Levels {
    /// The heading level of the entry title.
    #[builder(default = DEFAULT_ENTRY)]
    pub entry: Size,

    /// The heading level of individual sections.
    #[builder(default = DEFAULT_SECTION)]
    pub section: Size,
}

impl Levels {
    pub const fn entry(&self) -> Size {
        self.entry
    }

    pub const fn section(&self) -> Size {
        self.section
    }
}

impl_default_via_builder_build!(Levels);

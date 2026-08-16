//! The `levels` section.
//!
//! This module provides the [`Paths`] structure and its default values.
//!
//! See also [`config`].
//!
//! [`config`]: crate::config

use std::path::Path;

use bon::Builder;
use non_empty_str::{NonEmptyCowStr, NonEmptyStr, const_non_empty_str};
use ownership::IntoOwned;
use serde::{Deserialize, Serialize};

use crate::internals::impl_default_via_builder_build;

/// The default `paths.directory` value.
pub const DEFAULT_DIRECTORY: &NonEmptyStr = const_non_empty_str!("changes");

/// The default `paths.output` value.
pub const DEFAULT_OUTPUT: &NonEmptyStr = const_non_empty_str!("CHANGELOG.md");

/// Specifies fragment directories and changelog files.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Builder, IntoOwned)]
#[serde(default)]
pub struct Paths<'p> {
    /// The directory to fetch fragments from.
    #[builder(into, default = DEFAULT_DIRECTORY)]
    pub directory: NonEmptyCowStr<'p>,

    /// The file to write entries to.
    #[builder(into, default = DEFAULT_OUTPUT)]
    pub output: NonEmptyCowStr<'p>,
}

impl<'p> Paths<'p> {
    pub fn directory(&self) -> &NonEmptyStr {
        self.directory.as_ref()
    }

    pub fn output(&self) -> &NonEmptyStr {
        self.output.as_ref()
    }

    pub fn directory_path(&self) -> &Path {
        self.directory().as_ref()
    }

    pub fn output_path(&self) -> &Path {
        self.output().as_ref()
    }
}

impl_default_via_builder_build!(Paths<'_>);

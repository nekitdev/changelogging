//! The `formats` section.
//!
//! This module provides the [`Formats`] structure and its default values.
//!
//! See also [`config`].
//!
//! [`config`]: crate::config

use bon::Builder;
use non_empty_str::{NonEmptyCowStr, NonEmptyStr, const_non_empty_str};
use ownership::IntoOwned;
use serde::{Deserialize, Serialize};

use crate::internals::impl_default_via_builder_build;

/// The default `formats.title` value.
pub const DEFAULT_TITLE: &NonEmptyStr = const_non_empty_str!("{{version}} ({{date}})");

/// The default `formats.fragment` value.
pub const DEFAULT_FRAGMENT: &NonEmptyStr = const_non_empty_str!("{{content}} (#{{id}})");

/// Defines formats to use for rendering titles and fragments.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Builder, IntoOwned)]
#[serde(default)]
pub struct Formats<'f> {
    /// The format to use for rendering titles.
    #[builder(into, default = DEFAULT_TITLE)]
    pub title: NonEmptyCowStr<'f>,

    /// The format to use for rendering fragments.
    #[builder(into, default = DEFAULT_FRAGMENT)]
    pub fragment: NonEmptyCowStr<'f>,
}

impl Formats<'_> {
    pub fn title(&self) -> &NonEmptyStr {
        self.title.as_ref()
    }

    pub fn fragment(&self) -> &NonEmptyStr {
        self.fragment.as_ref()
    }
}

impl_default_via_builder_build!(Formats<'_>);

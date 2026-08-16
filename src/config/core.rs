//! The configuration core.
//!
//! This module contains the [`Config`] structure, which combines various parts of
//! `changelogging` configuration.
//!
//! See also [`config`].
//!
//! [`config`]: crate::config

use bon::Builder;
use ownership::IntoOwned;
use serde::{Deserialize, Serialize};

use crate::{
    config::{
        formats::Formats, indents::Indents, levels::Levels, order::Order, paths::Paths,
        start::Start, types::Types, wrap::Wrap,
    },
    internals::impl_default_via_builder_build,
};

/// Represents configurations.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Builder, IntoOwned)]
#[serde(default)]
pub struct Config<'c> {
    /// The `paths` section.
    #[builder(default)]
    pub paths: Paths<'c>,

    /// The `start` field.
    #[builder(default)]
    pub start: Start<'c>,

    /// The `levels` section.
    #[builder(default)]
    pub levels: Levels,

    /// The `indents` section.
    #[builder(default)]
    pub indents: Indents,

    /// The `formats` section.
    #[builder(default)]
    pub formats: Formats<'c>,

    /// The `wrap` field.
    #[builder(default)]
    pub wrap: Wrap,

    /// The `order` field.
    #[builder(default)]
    pub order: Order<'c>,

    /// The `types` section.
    #[builder(default)]
    pub types: Types<'c>,
}

impl<'c> Config<'c> {
    pub const fn paths(&self) -> &Paths<'c> {
        &self.paths
    }

    pub const fn start(&self) -> &Start<'c> {
        &self.start
    }

    pub const fn levels(&self) -> &Levels {
        &self.levels
    }

    pub const fn indents(&self) -> &Indents {
        &self.indents
    }

    pub const fn formats(&self) -> &Formats<'c> {
        &self.formats
    }

    pub const fn wrap(&self) -> Wrap {
        self.wrap
    }

    pub const fn order(&self) -> &Order<'c> {
        &self.order
    }

    pub const fn types(&self) -> &Types<'c> {
        &self.types
    }
}

impl_default_via_builder_build!(Config<'_>);

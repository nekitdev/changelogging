//! The `order` field.
//!
//! This module provides the [`Order`] type and its default value.
//!
//! See also [`config`].
//!
//! [`config`]: crate::config

use ownership::IntoOwned;
use serde::{Deserialize, Serialize};

use crate::fragments::types::{Name, Names};

/// Defines which types to include, and in what order to do so.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, IntoOwned)]
#[serde(transparent)]
#[repr(transparent)]
pub struct Order<'o> {
    names: Names<'o>,
}

impl Default for Order<'_> {
    fn default() -> Self {
        Self::new(Name::ARRAY.into())
    }
}

impl<'o> Order<'o> {
    pub const fn new(names: Names<'o>) -> Self {
        Self { names }
    }

    pub fn names(&self) -> &Names<'o> {
        &self.names
    }

    pub fn get(self) -> Names<'o> {
        self.names
    }
}

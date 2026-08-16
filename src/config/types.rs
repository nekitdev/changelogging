//! The `types` field.
//!
//! This module provides the [`Types`] type and its default value.
//!
//! See also [`config`].
//!
//! [`config`]: crate::config

use ownership::IntoOwned;
use serde::{Deserialize, Serialize};

use crate::fragments::types::{MAPPING, Mapping, Name, Title};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, IntoOwned)]
#[serde(transparent)]
#[repr(transparent)]
pub struct Types<'t> {
    mapping: Mapping<'t>,
}

impl Default for Types<'_> {
    fn default() -> Self {
        Self::new(MAPPING.into())
    }
}

impl<'t> From<Mapping<'t>> for Types<'t> {
    fn from(mapping: Mapping<'t>) -> Self {
        Self { mapping }
    }
}

impl<'t> From<Types<'t>> for Mapping<'t> {
    fn from(types: Types<'t>) -> Self {
        types.get()
    }
}

impl<'t> Types<'t> {
    pub const fn new(mapping: Mapping<'t>) -> Self {
        Self { mapping }
    }

    pub const fn mapping(&self) -> &Mapping<'t> {
        &self.mapping
    }

    pub fn find(&self, name: &Name<'t>) -> Option<&Title<'t>> {
        self.mapping.get(name)
    }

    pub fn get(self) -> Mapping<'t> {
        self.mapping
    }

    pub fn apply_defaults(&mut self) {
        MAPPING.into_iter().for_each(|(name, title)| {
            self.mapping.entry(name).or_insert(title);
        });
    }

    pub fn merge_defaults(&self) -> Self {
        let mut cloned = self.clone();

        cloned.apply_defaults();

        cloned
    }
}

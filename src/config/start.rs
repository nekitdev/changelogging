//! See also [`config`].
//!
//! [`config`]: crate::config

use std::borrow::Cow;

use non_empty_str::{NonEmptyCowStr, NonEmptyStr, NonEmptyString, const_non_empty_str};
use ownership::IntoOwned;
use serde::{Deserialize, Serialize};

pub const DEFAULT: &NonEmptyStr = const_non_empty_str!("<!-- changelogging: start -->");

/// Marks the location to start writing entries after.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, IntoOwned)]
#[serde(transparent)]
#[repr(transparent)]
pub struct Start<'s> {
    marker: NonEmptyCowStr<'s>,
}

impl Default for Start<'_> {
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl<'s> Start<'s> {
    pub const fn new(marker: NonEmptyCowStr<'s>) -> Self {
        Self { marker }
    }

    pub const fn borrowed(string: &'s NonEmptyStr) -> Self {
        Self::new(Cow::Borrowed(string))
    }

    pub const fn owned(string: NonEmptyString) -> Self {
        Self::new(Cow::Owned(string))
    }

    pub fn get(self) -> NonEmptyCowStr<'s> {
        self.marker
    }
}

impl Start<'_> {
    pub fn marker(&self) -> &NonEmptyStr {
        self.marker.as_ref()
    }

    pub const fn default() -> Self {
        Self::DEFAULT
    }

    pub const DEFAULT: Self = Self::borrowed(DEFAULT);
}

//! See also [`config`].
//!
//! [`config`]: crate::config

use non_zero_size::{Size, const_size};
use ownership::IntoOwned;
use serde::{Deserialize, Serialize};

/// The default [`Wrap`] value.
pub const DEFAULT: Size = const_size!(100);

/// Represents line lengths used in wrapping.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, IntoOwned)]
#[serde(transparent)]
#[repr(transparent)]
pub struct Wrap {
    size: Size,
}

impl Default for Wrap {
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl Wrap {
    pub const fn new(size: Size) -> Self {
        Self { size }
    }

    pub const fn size(&self) -> Size {
        self.size
    }

    pub const fn get(self) -> Size {
        self.size
    }

    pub const fn full_get(self) -> usize {
        self.get().get()
    }

    pub const fn default() -> Self {
        Self::DEFAULT
    }

    pub const DEFAULT: Self = Self::new(DEFAULT);
}

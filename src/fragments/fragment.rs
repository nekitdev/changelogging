//! Changelog fragments.

use std::{collections::HashMap, path::Path};

use bon::Builder;
use from_path::{FromPath, Load};
use ownership::IntoOwned;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::fragments::{
    content::{Content, Error as ContentError},
    partial::{LoadError as PartialError, Partial},
    types::Name,
};

/// Represents sources of errors that can occur when loading fragments.
#[derive(Debug, Error)]
#[error(transparent)]
pub enum LoadError {
    /// Content error.
    Content(#[from] ContentError),
    /// Partial error.
    Partial(#[from] PartialError),
}

impl LoadError {
    pub fn is_content(&self) -> bool {
        matches!(self, Self::Content(_))
    }

    pub fn is_partial(&self) -> bool {
        matches!(self, Self::Partial(_))
    }
}

/// Represents fragments.
#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Builder, IntoOwned,
)]
pub struct Fragment<'f> {
    /// The partial fragment.
    ///
    /// This field is flattened during (de)serialization.
    #[serde(flatten)]
    pub partial: Partial<'f>,
    /// The fragment content.
    pub content: Content<'f>,
}

impl<'f> Fragment<'f> {
    pub const fn partial(&self) -> &Partial<'f> {
        &self.partial
    }

    pub const fn content(&self) -> &Content<'f> {
        &self.content
    }
}

impl FromPath for Fragment<'_> {
    type Error = LoadError;

    fn from_path<P: AsRef<Path>>(path: P) -> Result<Self, Self::Error> {
        let partial = path.load()?;
        let content = path.load()?;

        let fragment = Self::builder().partial(partial).content(content).build();

        Ok(fragment)
    }
}

/// Represents collections of fragments.
pub type Fragments<'f> = [Fragment<'f>];

/// Represents sections, which map type names to collections of fragments.
pub type Sections<'s> = HashMap<Name<'s>, Vec<Fragment<'s>>>;

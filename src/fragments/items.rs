use std::{borrow::Cow, collections::HashMap, fs::DirEntry, path::Path};

use bon::Builder;
use from_path::Load;
use ownership::IntoOwned;
use serde::{Deserialize, Serialize};

use crate::fragments::{
    fragment::{Fragment, LoadError},
    types::Name,
};

pub type ItemPath<'i> = Cow<'i, Path>;

/// Represents fragments coupled with their respective paths.
#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Builder, IntoOwned,
)]
pub struct Item<'i> {
    /// The fragment.
    pub fragment: Fragment<'i>,
    /// The respective path.
    #[builder(into)]
    pub path: ItemPath<'i>,
}

impl<'i> Item<'i> {
    pub const fn fragment(&self) -> &Fragment<'i> {
        &self.fragment
    }

    pub fn path(&self) -> &Path {
        self.path.as_ref()
    }

    pub fn extract(self) -> Fragment<'i> {
        self.fragment
    }

    pub fn into_path(self) -> ItemPath<'i> {
        self.path
    }
}

impl TryFrom<DirEntry> for Item<'_> {
    type Error = LoadError;

    fn try_from(entry: DirEntry) -> Result<Self, Self::Error> {
        Self::try_entry(entry)
    }
}

impl Item<'_> {
    pub fn try_entry(entry: DirEntry) -> Result<Self, LoadError> {
        let path = entry.path();

        let fragment = path.load()?;

        let item = Self::builder().fragment(fragment).path(path).build();

        Ok(item)
    }

    pub fn entry(entry: DirEntry) -> Option<Self> {
        Self::try_entry(entry).ok()
    }
}

pub type OwnedItem = Item<'static>;

pub type SectionsPaths<'s> = HashMap<Name<'s>, Vec<Item<'s>>>;

pub type Paths<'p> = Vec<ItemPath<'p>>;

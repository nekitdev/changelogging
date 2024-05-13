use std::{borrow::Cow, collections::HashMap, fs::read_to_string, num::ParseIntError, path::Path};

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::paths::{name_str, FromPath};

pub type FragmentId = u32;

#[derive(Debug, Error)]
#[error(transparent)]
pub enum LoadFragmentError {
    Id(#[from] ParseIntError),
    Io(#[from] std::io::Error),
    #[error("the name of the fragment is not valid utf-8")]
    InvalidUtf8,
    #[error("unexpected EOF when parsing fragment name")]
    UnexpectedEof,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Fragment<'f> {
    pub id: FragmentId,
    pub name: Cow<'f, str>,
    pub content: Cow<'f, str>,
}

const DOT: char = '.';

impl FromPath for Fragment<'_> {
    type Error = LoadFragmentError;

    fn from_path<P: AsRef<Path>>(path: P) -> Result<Self, Self::Error> {
        let path_name = name_str(path.as_ref()).ok_or(LoadFragmentError::InvalidUtf8)?;

        let mut split = path_name.split(DOT);

        let id: FragmentId = split
            .next()
            .ok_or(LoadFragmentError::UnexpectedEof)?
            .parse()?;
        let name = split
            .next()
            .ok_or(LoadFragmentError::UnexpectedEof)?
            .to_owned()
            .into();

        let content = read_to_string(path)?.into();

        Ok(Self { id, name, content })
    }
}

impl<'f> Fragment<'f> {
    pub fn new(id: FragmentId, name: Cow<'f, str>, content: Cow<'f, str>) -> Self {
        Self { id, name, content }
    }
}

impl Fragment<'_> {
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn content(&self) -> &str {
        self.content.as_ref()
    }
}

pub type Fragments<'f> = Vec<Fragment<'f>>;
pub type Sections<'s> = HashMap<Cow<'s, str>, Fragments<'s>>;

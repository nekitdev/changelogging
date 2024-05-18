use std::{
    borrow::Cow, collections::HashMap, fs::read_to_string, num::ParseIntError, path::Path,
    str::FromStr,
};

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::paths::{name_str, FromPath};

pub type FragmentId = u32;

#[derive(Debug, Error)]
#[error(transparent)]
pub enum ParseError {
    Id(#[from] ParseIntError),
    #[error("unexpected EOF when parsing fragment info")]
    UnexpectedEof,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct FragmentInfo<'i> {
    pub id: FragmentId,
    pub type_name: Cow<'i, str>,
}

impl<'i> FragmentInfo<'i> {
    pub fn new(id: FragmentId, type_name: Cow<'i, str>) -> Self {
        Self { id, type_name }
    }
}

impl FromStr for FragmentInfo<'_> {
    type Err = ParseError;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let mut split = string.split(DOT);

        let id = split.next().ok_or(Self::Err::UnexpectedEof)?.parse()?;

        let type_name = split.next().ok_or(Self::Err::UnexpectedEof)?.to_owned();

        Ok(Self::new(id, type_name.into()))
    }
}

pub fn validate<S: AsRef<str>>(string: S) -> Result<(), ParseError> {
    let _check: FragmentInfo = string.as_ref().parse()?;

    Ok(())
}

#[derive(Debug, Error)]
#[error(transparent)]
pub enum Error {
    Io(#[from] std::io::Error),
    Info(#[from] ParseError),
    #[error("the name of the fragment is not valid utf-8")]
    InvalidUtf8,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Fragment<'f> {
    #[serde(flatten)]
    pub info: FragmentInfo<'f>,
    pub content: Cow<'f, str>,
}

const DOT: char = '.';

impl FromPath for Fragment<'_> {
    type Error = Error;

    fn from_path<P: AsRef<Path>>(path: P) -> Result<Self, Self::Error> {
        let name = name_str(path.as_ref()).ok_or(Self::Error::InvalidUtf8)?;

        let info = name.parse()?;

        let content = read_to_string(path)?.trim().to_owned();

        Ok(Self::new(info, content.into()))
    }
}

impl<'f> Fragment<'f> {
    pub fn new(info: FragmentInfo<'f>, content: Cow<'f, str>) -> Self {
        Self { info, content }
    }
}

impl Fragment<'_> {
    pub fn info(&self) -> &FragmentInfo<'_> {
        &self.info
    }

    pub fn content(&self) -> &str {
        self.content.as_ref()
    }
}

pub type Fragments<'f> = Vec<Fragment<'f>>;
pub type Slice<'a, 'f> = &'a [Fragment<'f>];
pub type Sections<'s> = HashMap<Cow<'s, str>, Fragments<'s>>;

use std::{borrow::Cow, path::Path};

use serde::{Deserialize, Serialize};

use crate::{
    config::{Config, Error, Order, Types},
    macros::{impl_from_path_with_parse, impl_from_str_with_toml},
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Paths<'p> {
    pub directory: Option<Cow<'p, Path>>,
    pub output: Option<Cow<'p, Path>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Levels {
    pub entry: Option<usize>,
    pub section: Option<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Indents {
    pub heading: Option<char>,
    pub bullet: Option<char>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Formats<'f> {
    pub title: Option<Cow<'f, str>>,
    pub fragment: Option<Cow<'f, str>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Options<'o> {
    pub paths: Option<Paths<'o>>,
    pub start: Option<Cow<'o, str>>,
    pub levels: Option<Levels>,
    pub indents: Option<Indents>,
    pub formats: Option<Formats<'o>>,
    pub wrap: Option<usize>,
    pub order: Option<Order<'o>>,
    pub types: Option<Types<'o>>,
}

impl_from_str_with_toml!(Options<'_>);
impl_from_path_with_parse!(Options<'_>, Error);

impl<'o> Options<'o> {
    pub fn into_config(self) -> Config<'o> {
        self.into()
    }
}

//! Optional configuration.
//!
//! This module provides the [`Options`] structure, which is essentially the same as [`Config`],
//! expect everything is wrapped in [`Option`], so `T` becomes `Option<T>`.
//!
//! The [`Options::into_config`] method is used to convert [`Options`] into [`Config`],
//! unwrapping values or using defaults provided by [`Config::default`].

use std::{borrow::Cow, path::Path};

use serde::{Deserialize, Serialize};

#[allow(unused)]
use crate::config;

use crate::config::{Config, Level, Order, Start, Types, Wrap};

/// Optional [`config::Paths`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Paths<'p> {
    /// Optional [`config::Paths::directory`].
    pub directory: Option<Cow<'p, Path>>,
    /// Optional [`config::Paths::output`].
    pub output: Option<Cow<'p, Path>>,
}

/// Optional [`config::Levels`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Levels {
    /// Optional [`config::Levels::entry`].
    pub entry: Option<Level>,
    /// Optional [`config::Levels::section`].
    pub section: Option<Level>,
}

/// Optional [`config::Indents`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Indents {
    /// Optional [`config::Indents::heading`].
    pub heading: Option<char>,
    /// Optional [`config::Indents::bullet`].
    pub bullet: Option<char>,
}

/// Optional [`config::Formats`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Formats<'f> {
    /// Optional [`config::Formats::title`].
    pub title: Option<Cow<'f, str>>,
    /// Optional [`config::Formats::fragment`].
    pub fragment: Option<Cow<'f, str>>,
}

/// Optional [`Config`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Options<'o> {
    /// Optional [`Config::paths`].
    pub paths: Option<Paths<'o>>,
    /// Optional [`Config::start`].
    pub start: Option<Start<'o>>,
    /// Optional [`Config::levels`].
    pub levels: Option<Levels>,
    /// Optional [`Config::indents`].
    pub indents: Option<Indents>,
    /// Optional [`Config::formats`].
    pub formats: Option<Formats<'o>>,
    /// Optional [`Config::wrap`].
    pub wrap: Option<Wrap>,
    /// Optional [`Config::order`].
    pub order: Option<Order<'o>>,
    /// Optional [`Config::types`].
    pub types: Option<Types<'o>>,
}

impl<'o> Options<'o> {
    /// Converts [`Options`] into [`Config`].
    ///
    /// This function uses the [`From`] implementation of [`Config`].
    pub fn into_config(self) -> Config<'o> {
        self.into()
    }
}

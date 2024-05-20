use std::{borrow::Cow, collections::HashMap, path::Path};

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{
    macros::{impl_from_path_with_parse, impl_from_str_with_toml},
    options::Options,
};

/// Represents errors that can occur during configuration loading.
#[derive(Debug, Error)]
#[error(transparent)]
pub enum Error {
    /// I/O error.
    Io(#[from] std::io::Error),
    /// TOML error.
    Toml(#[from] toml::de::Error),
}

/// Represents `paths` sections.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Paths<'p> {
    /// The directory to fetch fragments from.
    pub directory: Cow<'p, Path>,
    /// The file to write entries to.
    pub output: Cow<'p, Path>,
}

/// Represents `levels` sections.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Levels {
    pub entry: usize,
    pub section: usize,
}

/// Represents `indents` sections.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Indents {
    pub heading: char,
    pub bullet: char,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Formats<'f> {
    pub title: Cow<'f, str>,
    pub fragment: Cow<'f, str>,
}

pub type Order<'d> = Vec<Cow<'d, str>>;

pub type Types<'t> = HashMap<Cow<'t, str>, Cow<'t, str>>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Config<'c> {
    pub paths: Paths<'c>,
    pub start: Cow<'c, str>,
    pub levels: Levels,
    pub indents: Indents,
    pub formats: Formats<'c>,
    pub wrap: usize,
    pub order: Order<'c>,
    pub types: Types<'c>,
}

impl_from_str_with_toml!(Config<'_>);
impl_from_path_with_parse!(Config<'_>, Error);

const DEFAULTS: &str = include_str!("defaults.toml");

impl Default for Config<'_> {
    fn default() -> Self {
        // SAFETY: defaults must be valid
        DEFAULTS.parse().unwrap()
    }
}

impl Config<'_> {
    pub fn paths(&self) -> &Paths<'_> {
        &self.paths
    }

    pub fn levels(&self) -> &Levels {
        &self.levels
    }

    pub fn indents(&self) -> &Indents {
        &self.indents
    }

    pub fn formats(&self) -> &Formats<'_> {
        &self.formats
    }

    pub fn order(&self) -> &Order<'_> {
        &self.order
    }

    pub fn types(&self) -> &Types<'_> {
        &self.types
    }
}

impl<'a> From<Options<'a>> for Config<'a> {
    fn from(options: Options<'a>) -> Self {
        let default = Self::default();

        let default_paths = default.paths;
        let default_levels = default.levels;
        let default_indents = default.indents;
        let default_formats = default.formats;

        let paths_options = options.paths.unwrap_or_default();
        let levels_options = options.levels.unwrap_or_default();
        let indents_options = options.indents.unwrap_or_default();
        let formats_options = options.formats.unwrap_or_default();

        let paths = Paths {
            directory: paths_options.directory.unwrap_or(default_paths.directory),
            output: paths_options.output.unwrap_or(default_paths.output),
        };

        let levels = Levels {
            entry: levels_options.entry.unwrap_or(default_levels.entry),
            section: levels_options.section.unwrap_or(default_levels.section),
        };

        let indents = Indents {
            heading: indents_options.heading.unwrap_or(default_indents.heading),
            bullet: indents_options.bullet.unwrap_or(default_indents.bullet),
        };

        let formats = Formats {
            title: formats_options.title.unwrap_or(default_formats.title),
            fragment: formats_options.fragment.unwrap_or(default_formats.fragment),
        };

        let start = options.start.unwrap_or(default.start);

        let wrap = options.wrap.unwrap_or(default.wrap);

        let order = options.order.unwrap_or(default.order);

        let mut types = default.types;

        types.extend(options.types.into_iter().flatten());

        Self {
            paths,
            start,
            levels,
            indents,
            formats,
            wrap,
            order,
            types,
        }
    }
}

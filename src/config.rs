//! Configuration.
//!
//! See also [`context`].
//!
//! Below are all the configuration options known to and used by `changelogging`.
//!
//! ## `paths`
//!
//! The `paths` section specifies the location of *fragments* and the *changelog*.
//!
//! This section is optional, so are its fields (see [defaults] for more information):
//!
//! - `directory` is the directory containing fragments;
//! - `output` is the file containing the changelog.
//!
//! Here is an example of this section:
//!
//! ```toml
//! [paths]
//! directory = "changes"
//! output = "CHANGELOG.md"
//! ```
//!
//! This section is represented by the [`Paths`] structure.
//!
//! ## `start`
//!
//! The `start` field marks the location in the *changelog* to start writing entries after.
//!
//! In case the `start` string is not present in the changelog, the entries will be written
//! at the beginning of the changelog.
//!
//! This field is optional, and its default value can be found in [defaults].
//!
//! Here is an example of this field:
//!
//! ```toml
//! start = "<!-- changelogging: start -->
//! ```
//!
//! This field is represented as the `start` field of [`Config`].
//!
//! ## `levels`
//!
//! The `levels` section is used to tell `changelogging` which heading levels to use.
//!
//! This section is optional, so are its fields (see [defaults] for more information):
//!
//! - `entry` specifies the heading level of the entry title;
//! - `section` specifies the heading level of individual sections.
//!
//! Here is an example of this section:
//!
//! ```toml
//! [levels]
//! entry = 2
//! section = 3
//! ```
//!
//! This section is represented by the [`Levels`] structure.
//!
//! ## `indents`
//!
//! The `indents` section specifies which characters to use for indenting.
//!
//! This section is optional, so are its fields (see [defaults] for more information):
//!
//! - `heading` defines the character to use for headings;
//! - `bullet` defines the character to use for indenting.
//!
//! Here is an example of this section:
//!
//! ```toml
//! [indents]
//! heading = "#"
//! bullet = "-"
//! ```
//!
//! This section is represented by the [`Indents`] structure.
//!
//! ## `formats`
//!
//! The `formats` section defines formats (templates) to use for rendering titles and fragments.
//!
//! This section is optional, so are its fields (see [defaults] for more information):
//!
//! - `title` specifies the format to use for rendering titles.
//! - `fragment` specifies the format to use for rendering fragments.
//!
//! All fields of [`Context`] (plus `date`) are available as formatting arguments within `title`.
//! Within `fragment`, one can use fields of [`Context`] and [`Fragment`].
//!
//! ```toml
//! [formats]
//! title = "[{{version}}]({{url}}/tree/v{{version}}) ({{date}})"
//! fragment = "{{content}} ([#{{id}}]({{url}}/pull/{{id}}))"
//! ```
//!
//! This section is represented by the [`Formats`] structure.
//!
//! ## `wrap`
//!
//! The `wrap` field specifies the line length to use when wrapping entries.
//! Please note that entries are **always** wrapped.
//!
//! This field is optional, and its default value can be found in [defaults].
//!
//! Here is an example of this field:
//!
//! ```toml
//! wrap = 100
//! ```
//!
//! This field is represented as the `wrap` field of [`Config`].
//!
//! ## `order`
//!
//! The `order` field defines which *types* to include, and in what order to do so.
//!
//! This field is optional, and its default value can be found in [defaults].
//!
//! Here is an example of this field:
//!
//! ```toml
//! order = ["security", "feature", "change", "fix", "deprecation", "removal", "internal"]
//! ```
//!
//! This field is represented as the `order` field of [`Config`].
//!
//! ## `types`
//!
//! The `types` section specifies the *mapping* of *types* to their *titles*.
//! This section behaves slightly differently than others. Instead of using `types` directly,
//! the mapping specified extends the default mapping. (see [defaults] for the default mapping).
//!
//! Here is an example of this section:
//!
//! ```toml
//! [types]
//! security = "Security"
//! feature = "Features"
//! change = "Changes"
//! fix = "Fixes"
//! deprecation = "Deprecations"
//! removal = "Removals"
//! internal = "Internal"
//! ```
//!
//! This section is represented as the `types` field of [`Config`].
//!
//! [defaults]: https://github.com/nekitdev/changelogging/blob/main/src/defaults.toml
//!
//! [`context`]: crate::context
//! [`Context`]: crate::context::Context
//! [`Fragment`]: crate::fragment::Fragment

use std::{borrow::Cow, collections::HashMap, num::NonZeroUsize, path::Path};

use serde::{Deserialize, Serialize};

use crate::options::Options;

/// Marks the location in the changelog to start writing entries after.
pub type Start<'s> = Cow<'s, str>;

/// Specifies fragment directories and changelog files.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Paths<'p> {
    /// The directory to fetch fragments from.
    pub directory: Cow<'p, Path>,
    /// The file to write entries to.
    pub output: Cow<'p, Path>,
}

/// Represents heading levels.
pub type Level = NonZeroUsize;

/// Defines which heading levels to use.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Levels {
    /// The heading level of the entry title.
    pub entry: Level,
    /// The heading level of individual sections.
    pub section: Level,
}

/// Specifies characters to use for headings and indentation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Indents {
    /// The character to use for headings.
    pub heading: char,
    /// The character to use for indentation.
    pub bullet: char,
}

/// Defines formats to use for rendering titles and fragments.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Formats<'f> {
    /// The format to use for rendering titles.
    pub title: Cow<'f, str>,
    /// The format to use for rendering fragments.
    pub fragment: Cow<'f, str>,
}

/// Specifies the line length to use when wrapping entries.
pub type Wrap = NonZeroUsize;

/// Defines which types to include, and in what order to do so.
pub type Order<'o> = Vec<Cow<'o, str>>;

/// Specifies the mapping of types to their titles.
pub type Types<'t> = HashMap<Cow<'t, str>, Cow<'t, str>>;

/// Represents configurations.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Config<'c> {
    /// The `paths` section.
    pub paths: Paths<'c>,
    /// The `start` field.
    pub start: Start<'c>,
    /// The `levels` section.
    pub levels: Levels,
    /// The `indents` section.
    pub indents: Indents,
    /// The `formats` section.
    pub formats: Formats<'c>,
    /// The `wrap` field.
    pub wrap: Wrap,
    /// The `order` field.
    pub order: Order<'c>,
    /// The `types` section.
    pub types: Types<'c>,
}

const DEFAULTS: &str = include_str!("defaults.toml");

impl Default for Config<'_> {
    fn default() -> Self {
        // SAFETY: defaults must be valid
        toml::from_str(DEFAULTS).unwrap()
    }
}

impl Config<'_> {
    /// Returns [`Paths`] reference.
    pub fn paths_ref(&self) -> &Paths<'_> {
        &self.paths
    }

    /// Returns [`Levels`] reference.
    pub fn levels_ref(&self) -> &Levels {
        &self.levels
    }

    /// Returns [`Indents`] reference.
    pub fn indents_ref(&self) -> &Indents {
        &self.indents
    }

    /// Returns [`Formats`] reference.
    pub fn formats_ref(&self) -> &Formats<'_> {
        &self.formats
    }

    /// Returns [`Order`] reference.
    pub fn order_ref(&self) -> &Order<'_> {
        &self.order
    }

    /// Returns [`Types`] reference.
    pub fn types_ref(&self) -> &Types<'_> {
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

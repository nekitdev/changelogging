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
//! This section is optional, so are its fields (see defaults for more information):
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
//! This field is optional, and its default value can be found in defaults.
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
//! This section is optional, so are its fields (see defaults for more information):
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
//! This section is optional, so are its fields (see defaults for more information):
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
//! This section is optional, so are its fields (see defaults for more information):
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
//! This field is optional, and its default value can be found in defaults.
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
//! This field is optional, and its default value can be found in defaults.
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
//! the mapping specified extends the default mapping. (see defaults for the default mapping).
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
//! [`context`]: crate::context
//! [`Context`]: crate::context::Context
//! [`Fragment`]: crate::fragment::Fragment

use std::{borrow::Cow, collections::HashMap, num::NonZeroUsize, path::Path};

use serde::{Deserialize, Serialize};

/// Marks the location in the changelog to start writing entries after.
pub type Start<'s> = Cow<'s, str>;

/// The default `start` value.
pub const DEFAULT_START: &str = "<!-- changelogging: start -->";

/// Specifies fragment directories and changelog files.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct Paths<'p> {
    /// The directory to fetch fragments from.
    pub directory: Cow<'p, Path>,
    /// The file to write entries to.
    pub output: Cow<'p, Path>,
}

/// The default `paths.directory` value.
pub const DEFAULT_DIRECTORY: &str = "changes";

/// The default `paths.output` value.
pub const DEFAULT_OUTPUT: &str = "CHANGELOG.md";

impl Default for Paths<'_> {
    fn default() -> Self {
        let directory = Path::new(DEFAULT_DIRECTORY).into();
        let output = Path::new(DEFAULT_OUTPUT).into();

        Self { directory, output }
    }
}

/// Represents heading levels.
pub type Level = NonZeroUsize;

/// Defines which heading levels to use.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct Levels {
    /// The heading level of the entry title.
    pub entry: Level,
    /// The heading level of individual sections.
    pub section: Level,
}

/// The default `levels.entry` value.
pub const DEFAULT_ENTRY: usize = 2;

/// The default `levels.section` value.
pub const DEFAULT_SECTION: usize = 3;

impl Default for Levels {
    fn default() -> Self {
        let entry = Level::new(DEFAULT_ENTRY).unwrap();
        let section = Level::new(DEFAULT_SECTION).unwrap();

        Self { entry, section }
    }
}

/// Specifies characters to use for headings and indentation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct Indents {
    /// The character to use for headings.
    pub heading: char,
    /// The character to use for indentation.
    pub bullet: char,
}

/// The default `indents.heading` value.
pub const DEFAULT_HEADING: char = '#';

/// The default `indents.bullet` value.
pub const DEFAULT_BULLET: char = '-';

impl Default for Indents {
    fn default() -> Self {
        let heading = DEFAULT_HEADING;
        let bullet = DEFAULT_BULLET;

        Self { heading, bullet }
    }
}

/// Defines formats to use for rendering titles and fragments.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct Formats<'f> {
    /// The format to use for rendering titles.
    pub title: Cow<'f, str>,
    /// The format to use for rendering fragments.
    pub fragment: Cow<'f, str>,
}

/// The default `formats.title` value.
pub const DEFAULT_TITLE: &str = "{{version}} ({{date}})";

/// The default `formats.fragment` value.
pub const DEFAULT_FRAGMENT: &str = "{{content}} (#{{id}})";

impl Default for Formats<'_> {
    fn default() -> Self {
        let title = DEFAULT_TITLE.into();
        let fragment = DEFAULT_FRAGMENT.into();

        Self { title, fragment }
    }
}

/// Specifies the line length to use when wrapping entries.
pub type Wrap = NonZeroUsize;

/// The default `wrap` value.
pub const DEFAULT_WRAP: usize = 100;

/// Defines which types to include, and in what order to do so.
pub type Order<'o> = Vec<Cow<'o, str>>;

/// Returns the default `order` value.
pub fn default_order() -> Vec<&'static str> {
    vec![
        "security",
        "feature",
        "change",
        "fix",
        "deprecation",
        "removal",
        "internal",
    ]
}

fn into_order(vec: Vec<&str>) -> Order<'_> {
    vec.into_iter().map(Cow::Borrowed).collect()
}

/// Specifies the mapping of types to their titles.
pub type Types<'t> = HashMap<Cow<'t, str>, Cow<'t, str>>;

/// Represents configurations.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
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

macro_rules! hash_map {
    ($($key: expr => $value: expr),* $(,)?) => {
        {
            let mut hash_map = std::collections::HashMap::new();

            $(
                hash_map.insert($key, $value);
            )*

            hash_map
        }
    }
}

/// Returns the default `types` value.
pub fn default_types() -> HashMap<&'static str, &'static str> {
    hash_map! {
        "security" => "Security",
        "feature" => "Features",
        "change" => "Changes",
        "fix" => "Fixes",
        "deprecation" => "Deprecations",
        "removal" => "Removals",
        "internal" => "Internal",
    }
}

fn into_types<'t>(hash_map: HashMap<&'t str, &'t str>) -> Types<'t> {
    hash_map
        .into_iter()
        .map(|(name, title)| (Cow::Borrowed(name), Cow::Borrowed(title)))
        .collect()
}

impl Default for Config<'_> {
    fn default() -> Self {
        let paths = Paths::default();

        let start = DEFAULT_START.into();

        let levels = Levels::default();

        let indents = Indents::default();

        let formats = Formats::default();

        let wrap = Wrap::new(DEFAULT_WRAP).unwrap();

        let order = into_order(default_order());

        let types = into_types(default_types());

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

impl Config<'_> {
    /// Returns [`Paths`] reference.
    pub fn paths(&self) -> &Paths<'_> {
        &self.paths
    }

    /// Returns [`Levels`] reference.
    pub fn levels(&self) -> &Levels {
        &self.levels
    }

    /// Returns [`Indents`] reference.
    pub fn indents(&self) -> &Indents {
        &self.indents
    }

    /// Returns [`Formats`] reference.
    pub fn formats(&self) -> &Formats<'_> {
        &self.formats
    }

    /// Returns [`Order`] reference.
    pub fn order(&self) -> &Order<'_> {
        &self.order
    }

    /// Returns [`Types`] reference.
    pub fn types(&self) -> &Types<'_> {
        &self.types
    }
}

impl Config<'_> {
    /// Returns `types` with defaults included.
    pub fn types_with_defaults(&self) -> Types<'_> {
        let mut types_with_defaults = into_types(default_types());

        types_with_defaults.extend(self.types.clone());

        types_with_defaults
    }
}

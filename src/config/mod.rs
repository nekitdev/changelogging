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
//! This field is represented as the [`start`] field of [`Config`].
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
//! This field is represented as the [`wrap`] field of [`Config`].
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
//! This field is represented as the [`order`] (type [`Order<'_>`](Order)) field of [`Config`].
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
//! This section is represented as the [`types`] field (type [`Types<'_>`](Types)) of [`Config`].
//!
//! [`context`]: crate::context
//! [`Context<'_>`]: crate::context::Context
//! [`Fragment<'_>`]: crate::fragments::fragment::Fragment

pub mod core;
pub mod formats;
pub mod indents;
pub mod levels;
pub mod order;
pub mod paths;
pub mod start;
pub mod types;
pub mod wrap;

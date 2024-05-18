//! Building changelogs from fragments.
//!
//! # Example
//!
//! Please see the [README] for an overview of this project and an example of using it.
//!
//! # Inspiration
//!
//! [Keep a Changelog](https://keepachangelog.com/) is the inspiration of this project.
//!
//! # Terminology
//!
//! ## Changelogs
//!
//! *Changelogs* are files which document all notable changes between project versions.
//! Each version of the project has its own *entry* in the changelog.
//!
//! ## Fragments
//!
//! *Fragments* are small pieces of information, which, combined together, form changelog *entries*.
//!
//! Fragments contain three things:
//!
//! - *id*, usually bound to something like *pull requests*;
//! - *type*, for instance *change* or *feature*;
//! - *content*, which gets written to the changelog.
//!
//! In `changelogging`, fragments are files which have names starting with `{id}.{name}`
//! and contain fragment contents.
//!
//! ## Entries
//!
//! *Entries* describe changes between project versions. They are composed of *sections*.
//!
//! ## Sections
//!
//! *Sections* group and describe specific changes, e.g. *features*, *fixes*, *deprecations*, etc.
//!
//! # Configuration
//!
//! `changelogging` uses [TOML](https://github.com/toml-lang/toml) for its configuration.
//!
//! By default the application will look for the `changelogging.toml` file in the current directory.
//! It also understands `pyproject.toml` if it contains the `[tool.changelogging]` section.
//! In case both files are present, the former takes precendence.
//!
//! Below are all the configuration options known to and used by `changelogging`.
//!
//! ## `context`
//!
//! The `context` section provides information about the project to `changelogging`.
//!
//! It is always required, and the fields are as follows:
//!
//! - `name` is the name of the project;
//! - `version` is the version of the project;
//! - `url` is the URL of the project.
//!
//! Here is an example of this section:
//!
//! ```toml
//! [context]
//! name = "changelogging"
//! version = "0.1.0"
//! url = "https://github.com/nekitdev/changelogging"
//! ```
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
//! ## `start`
//!
//! The `start` field marks the location in the *changelog* to start writing entries at.
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
//! ## `levels`
//!
//! The `levels` section is used to tell `changelogging` which heading levels to use.
//!
//! This section is optional, so are its fields (see [defaults] for more information):
//!
//! - `entry` specifies the heading level of the title;
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
//! # Usage
//!
//! This section assumes we have [this] configuration.
//!
//! ## Globals
//!
//! - `--help (-h)` displays help information.
//! - `--version (-V)` shows this application's version.
//! - `--directory (-D)` changes the directory before doing anything.
//! - `--config (-C)` specifies the configuration file to use.
//!
//! ## `create`
//!
//! The `create` command is used to create changelog fragments.
//!
//! The fragment content can be passed through the argument:
//!
//! ```console
//! $ changelogging create --content "Added cool features!" 13.feature.md
//! ```
//!
//! If the content is not provided, the placeholder gets written instead:
//!
//! ```console
//! $ changelogging create 22.change.md
//! $ cat changes/22.change.md
//! Add the fragment content here.
//! ```
//!
//! Let us add some content to this fragment:
//!
//! ```console
//! $ echo "Changed some things!" > changes/22.change.md
//! ```
//!
//! Alternatively, the default editor can be used to write the fragment content:
//!
//! ```console
//! $ changelogging create --edit 34.fix.md
//! ```
//!
//! And, inside the editor:
//!
//! ```md
//! Fixed annoying bugs!
//! ```
//!
//! Here are the options (except for [globals](#globals)) that `create` supports:
//!
//! - `--content (-c)` passes the content of the fragment through the argument.
//! - `--edit (-e)` opens the default editor to enter the fragment's contents.
//!
//! ## `build`
//!
//! The `build` command is used to build changelog entries from fragments.
//!
//! Firstly, one can preview the changelog entry:
//!
//! ```console
//! $ changelogging build --preview
//! ## [0.1.0](https://github.com/nekitdev/changelogging/tree/v0.1.0) (YYYY-MM-DD)
//!
//! ### Features
//!
//! - Added cool features! ([#13](https://github.com/nekitdev/changelogging/pull/13))
//!
//! ### Changes
//!
//! - Changed some things! ([#22](https://github.com/nekitdev/changelogging/pull/22))
//!
//! ### Fixes
//!
//! - Fixed annoying bugs! ([#34](https://github.com/nekitdev/changelogging/pull/34))
//! ```
//!
//! And, finally, write it to the changelog:
//!
//! ```console
//! $ changelogging build
//! ```
//!
//! Here are the options (except for [globals](#globals)) that `build` supports:
//!
//! - `--date (-d)` specifies the date to use instead of today.
//! - `--preview (-p)` outputs the built entry instead of writing it to the changelog.
//!
//! [README]: https://github.com/nekitdev/changelogging/blob/main/README.md
//! [defaults]: https://github.com/nekitdev/changelogging/blob/main/src/defaults.toml
//! [this]: https://github.com/nekitdev/changelogging/blob/main/changelogging.toml
//!
//! [`Context`]: crate::context::Context
//! [`Fragment`]: crate::fragments::Fragment

#![forbid(unsafe_code)]

pub mod app;
pub mod build;
pub mod config;
pub mod context;
pub mod create;
pub mod date;
pub mod fragments;
mod macros;
pub mod options;
pub mod paths;
pub mod workspace;

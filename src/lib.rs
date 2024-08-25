//! Building changelogs from fragments.
//!
//! # Example
//!
//! Please see the [readme] for an overview of this project and an example of using it.
//!
//! # Inspiration
//!
//! [Keep a Changelog](https://keepachangelog.com/) is the inspiration of this project.
//!
//! The [changelog] of `changelogging` is built using itself, which gives the "bootstrapping" vibe.
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
//! In case both files are present, the former takes precedence.
//!
//! See [`config`] for configuration, [`context`] for contexts and [`workspace`]
//! that combines configuration and context into one structure.
//!
//! # Usage
//!
//! This section assumes we have [this] configuration and the following [template].
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
//! - `--add (-a)` adds the fragment file via `git`.
//!
//! ## `preview`
//!
//! The `preview` command is used to preview changelog entries:
//!
//! ```console
//! $ changelogging preview
//! ## [0.5.0](https://github.com/nekitdev/changelogging/tree/v0.5.0) (YYYY-MM-DD)
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
//! Here are the options (except for [globals](#globals)) that `preview` supports:
//!
//! - `--date (-d)` specifies the date to use instead of today.
//!
//! ## `build`
//!
//! The `build` command is used to build changelog entries from fragments.
//!
//! After one ensures that the changelog entry is correct, building the changelog is as simple as:
//!
//! ```console
//! $ changelogging build
//! ```
//!
//! You can also see the [rendered] version.
//!
//! Here are the options (except for [globals](#globals)) that `build` supports:
//!
//! - `--date (-d)` specifies the date to use instead of today.
//! - `--stage (-s)` stages the updated changelog via `git`.
//! - `--remove (-r)` removes all fragment files with `git`.
//!
//! [changelog]: https://github.com/nekitdev/changelogging/blob/main/CHANGELOG.md
//! [readme]: https://github.com/nekitdev/changelogging/blob/main/README.md
//! [this]: https://github.com/nekitdev/changelogging/blob/main/changelogging.toml
//! [template]: https://github.com/nekitdev/changelogging/blob/main/examples/TEMPLATE.md
//! [rendered]: https://github.com/nekitdev/changelogging/blob/main/examples/CHANGELOG.md

#![forbid(unsafe_code)]
#![deny(missing_docs)]

pub mod app;
pub mod builder;
pub mod commands;
pub mod config;
pub mod context;
pub mod date;
pub mod discover;
pub mod fragment;
pub mod git;
pub mod init;
pub mod load;
pub mod workspace;

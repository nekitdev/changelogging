//! The application.

use std::path::{Path, PathBuf};

use clap::{Args, Parser, Subcommand};
use clap_verbosity_flag::{InfoLevel, Verbosity};
use from_path::Load;
use non_empty_str::NonEmptyString;
use thiserror::Error;

use crate::{
    commands::{
        build::{Error as BuildError, build},
        create::{Error as CreateError, create},
        preview::{Error as PreviewError, preview},
    },
    discover::{Error as DiscoverError, discover},
    init::{Error as InitError, init},
    prepare::{Error as PrepareError, prepare},
    toml::load::Error as LoadError,
    workspace::Workspace,
};

pub type DefaultVerbosity = Verbosity<InfoLevel>;

/// Represents global options of `changelogging`.
#[derive(Debug, Args)]
pub struct Globals {
    /// The path to the `git` executable to use.
    #[arg(
        short = 'G',
        long,
        global = true,
        name = "GIT",
        help = "Use this git executable",
        default_value = "git"
    )]
    pub git: PathBuf,

    /// The directory to change to before doing anything.
    #[arg(
        short = 'D',
        long,
        global = true,
        name = "DIRECTORY",
        help = "Change to this directory before doing anything"
    )]
    pub directory: Option<PathBuf>,

    /// The path to the config file to use.
    #[arg(
        short = 'C',
        long,
        global = true,
        name = "FILE",
        help = "Use the config from this file"
    )]
    pub config: Option<PathBuf>,

    /// The path to the file to use for tracing.
    #[arg(
        short = 'T',
        long,
        global = true,
        name = "FILE",
        help = "Use this file for tracing"
    )]
    pub tracing: Option<PathBuf>,
}

/// Represents the `changelogging` application.
#[derive(Debug, Parser)]
#[command(
    author,
    version,
    about,
    propagate_version = true,
    arg_required_else_help = true
)]
pub struct App {
    /// The global options to use.
    #[command(flatten)]
    pub globals: Globals,
    /// The verbosity level to use.
    #[command(flatten)]
    pub verbosity: DefaultVerbosity,
    /// The subcommand to run, if any.
    #[command(subcommand)]
    pub command: Command,
}

/// Represents errors that can occur during application runs.
#[derive(Debug, Error)]
#[error(transparent)]
pub enum Error {
    /// Preparation errors.
    Prepare(#[from] PrepareError),
    /// Initialization errors.
    Init(#[from] InitError),
    /// Workspace discovery errors.
    Discover(#[from] DiscoverError),
    /// Workspace loading errors.
    Workspace(#[from] LoadError),
    /// `build` errors.
    Build(#[from] BuildError),
    /// `preview` errors.
    Preview(#[from] PreviewError),
    /// `create` errors.
    Create(#[from] CreateError),
}

impl App {
    /// Runs the application.
    ///
    /// # Errors
    ///
    /// Returns [`struct@Error`] when any error is encountered.
    pub fn run(self) -> Result<(), Error> {
        let globals = self.globals;

        prepare()
            .verbosity(self.verbosity)
            .maybe_path(globals.tracing)
            .call()?;

        init().maybe_directory(globals.directory).call()?;

        let workspace = if let Some(path) = globals.config {
            path.load()?
        } else {
            discover()?
        };

        match self.command {
            Command::Build(build) => {
                build.run(globals.git, workspace)?;
            }
            Command::Preview(preview) => {
                preview.run(workspace)?;
            }
            Command::Create(create) => {
                create.run(globals.git, workspace)?;
            }
        };

        Ok(())
    }
}

/// Represents `changelogging` subcommands.
#[derive(Debug, Subcommand)]
pub enum Command {
    /// The `build` subcommand.
    #[command(about = "Build changelogs from fragments")]
    Build(BuildCommand),
    /// The `preview` subcommand.
    #[command(about = "Preview changelog entries")]
    Preview(PreviewCommand),
    /// The `create` subcommand.
    #[command(about = "Create changelog fragments")]
    Create(CreateCommand),
}

/// Represents the `build` subcommand.
#[derive(Debug, Args)]
pub struct BuildCommand {
    #[arg(name = "VERSION", help = "Use this version instead of the current one")]
    pub version: Option<NonEmptyString>,

    /// The date to use. If not provided, [`today`] is used.
    ///
    /// [`today`]: crate::date::today
    #[arg(
        short = 'd',
        long,
        name = "DATE",
        help = "Use the date provided instead of today"
    )]
    pub date: Option<NonEmptyString>,

    /// Whether to stage the changelog.
    #[arg(short = 's', long, action, help = "Stage the changelog")]
    pub stage: bool,

    /// Whether to remove fragments.
    #[arg(short = 'r', long, action, help = "Remove the fragments")]
    pub remove: bool,
}

impl BuildCommand {
    /// Runs the `build` subcommand.
    ///
    /// # Errors
    ///
    /// Returns [`BuildError`] when any error is encountered.
    pub fn run<G: AsRef<Path>>(
        self,
        git: G,
        mut workspace: Workspace<'_>,
    ) -> Result<(), BuildError> {
        if let Some(version) = self.version {
            workspace.context.version = version.into();
        };

        build()
            .git(git)
            .workspace(workspace)
            .maybe_date(self.date)
            .stage(self.stage)
            .remove(self.remove)
            .call()
    }
}

/// Represents the `preview` subcommand.
#[derive(Debug, Args)]
pub struct PreviewCommand {
    #[arg(name = "VERSION", help = "Use this version instead of the current one")]
    pub version: Option<NonEmptyString>,

    /// The date to use. If not provided, [`today`] is used.
    ///
    /// [`today`]: crate::date::today
    #[arg(
        short = 'd',
        long,
        name = "DATE",
        help = "Use the date provided instead of today"
    )]
    pub date: Option<NonEmptyString>,
}

impl PreviewCommand {
    /// Runs the `preview` subcommand.
    ///
    /// # Errors
    ///
    /// Returns [`PreviewError`] when any error is encountered.
    pub fn run(self, mut workspace: Workspace<'_>) -> Result<(), PreviewError> {
        if let Some(version) = self.version {
            workspace.context.version = version.into();
        };

        preview().workspace(workspace).maybe_date(self.date).call()
    }
}

/// Represents the `create` subcommand.
#[derive(Debug, Args)]
#[command(about = "Create changelog fragments")]
pub struct CreateCommand {
    /// The name of the fragment.
    #[arg(name = "NAME", help = "Write to the directory with this file name")]
    pub name: NonEmptyString,

    /// The fragment content, if it is passed as the argument.
    #[arg(
        short = 'c',
        long,
        name = "TEXT",
        help = "Pass the fragment content as this argument"
    )]
    pub content: Option<NonEmptyString>,

    /// Whether to open the default editor to edit the fragment content.
    #[arg(
        short = 'e',
        long,
        action,
        help = "Open the default editor to edit the content"
    )]
    pub edit: bool,

    /// Whether to add the fragment via `git`.
    #[arg(short = 'a', long, action, help = "Add the fragment via `git`")]
    pub add: bool,
}

impl CreateCommand {
    /// Runs the `create` subcommand.
    ///
    /// # Errors
    ///
    /// Returns [`CreateError`] when any error is encountered.
    pub fn run<G: AsRef<Path>>(self, git: G, workspace: Workspace<'_>) -> Result<(), CreateError> {
        create()
            .git(git)
            .workspace(workspace)
            .name(self.name)
            .maybe_content(self.content)
            .edit(self.edit)
            .add(self.add)
            .call()
    }
}

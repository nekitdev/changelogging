//! The application.

use std::path::{Path, PathBuf};

use clap::{Args, Parser, Subcommand};
use miette::Diagnostic;
use thiserror::Error;

use crate::{
    commands::{build::build, create::create, preview::preview},
    discover::discover,
    init::init,
    load::load,
    workspace::Workspace,
};

/// Represents global options of `changelogging`.
#[derive(Debug, Args)]
pub struct Globals {
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
    /// The subcommand to run, if any.
    #[command(subcommand)]
    pub command: Option<Command>,
}

/// Represents sources of errors that can occur during application runs.
#[derive(Debug, Error, Diagnostic)]
#[error(transparent)]
#[diagnostic(transparent)]
pub enum ErrorSource {
    /// Initialization errors.
    Init(#[from] crate::init::Error),
    /// Workspace discovery errors.
    Discover(#[from] crate::discover::Error),
    /// Workspace loading errors.
    Workspace(#[from] crate::workspace::Error),
    /// `build` errors.
    Build(#[from] crate::commands::build::Error),
    /// `preview` errors.
    Preview(#[from] crate::commands::preview::Error),
    /// `create` errors.
    Create(#[from] crate::commands::create::Error),
}

/// Represents errors that can occur during application runs.
#[derive(Debug, Error, Diagnostic)]
#[error("error encountered")]
#[diagnostic(
    code(changelogging::app::run),
    help("see the report for more information")
)]
pub struct Error {
    /// The source of this error.
    #[source]
    #[diagnostic_source]
    pub source: ErrorSource,
}

impl Error {
    /// Constructs [`Self`].
    pub fn new(source: ErrorSource) -> Self {
        Self { source }
    }

    /// Constructs [`Self`] from [`Error`].
    ///
    /// [`Error`]: crate::init::Error
    pub fn init(source: crate::init::Error) -> Self {
        Self::new(source.into())
    }

    /// Constructs [`Self`] from [`Error`].
    ///
    /// [`Error`]: crate::discover::Error
    pub fn discover(source: crate::discover::Error) -> Self {
        Self::new(source.into())
    }

    /// Constructs [`Self`] from [`Error`].
    ///
    /// [`Error`]: crate::workspace::Error
    pub fn workspace(source: crate::workspace::Error) -> Self {
        Self::new(source.into())
    }

    /// Constructs [`Self`] from [`Error`].
    ///
    /// [`Error`]: crate::commands::build::Error
    pub fn build(source: crate::commands::build::Error) -> Self {
        Self::new(source.into())
    }

    /// Constructs [`Self`] from [`Error`]
    ///
    /// [`Error`]: crate::commands::preview::Error
    pub fn preview(source: crate::commands::preview::Error) -> Self {
        Self::new(source.into())
    }

    /// Constructs [`Self`] from [`Error`].
    ///
    /// [`Error`]: crate::commands::create::Error
    pub fn create(source: crate::commands::create::Error) -> Self {
        Self::new(source.into())
    }
}

impl App {
    /// Runs the application.
    ///
    /// # Errors
    ///
    /// Returns [`struct@Error`] when any error is encountered.
    pub fn run(self) -> Result<(), Error> {
        let globals = self.globals;

        init(globals.directory).map_err(Error::init)?;

        let workspace = if let Some(path) = globals.config {
            load(path).map_err(Error::workspace)?
        } else {
            discover().map_err(Error::discover)?
        };

        if let Some(command) = self.command {
            match command {
                Command::Build(build) => {
                    build.run(workspace).map_err(Error::build)?;
                }
                Command::Preview(preview) => {
                    preview.run(workspace).map_err(Error::preview)?;
                }
                Command::Create(create) => {
                    let directory = workspace.config.paths.directory;

                    create.run(directory).map_err(Error::create)?;
                }
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
    /// The date to use. If not provided, [`today`] is used.
    ///
    /// [`today`]: crate::date::today
    #[arg(
        short = 'd',
        long,
        name = "DATE",
        help = "Use the date provided instead of today"
    )]
    pub date: Option<String>,

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
    /// Returns [`Error`] when any error is encountered.
    ///
    /// [`Error`]: crate::commands::build::Error
    pub fn run(self, workspace: Workspace<'_>) -> Result<(), crate::commands::build::Error> {
        build(workspace, self.date, self.stage, self.remove)
    }
}

/// Represents the `preview` subcommand.
#[derive(Debug, Args)]
pub struct PreviewCommand {
    /// The date to use. If not provided, [`today`] is used.
    ///
    /// [`today`]: crate::date::today
    #[arg(
        short = 'd',
        long,
        name = "DATE",
        help = "Use the date provided instead of today"
    )]
    pub date: Option<String>,
}

impl PreviewCommand {
    /// Runs the `preview` subcommand.
    ///
    /// # Errors
    ///
    /// Returns [`Error`] when any error is encountered.
    ///
    /// [`Error`]: crate::commands::preview::Error
    pub fn run(self, workspace: Workspace<'_>) -> Result<(), crate::commands::preview::Error> {
        preview(workspace, self.date)
    }
}

/// Represents the `create` subcommand.
#[derive(Debug, Args)]
#[command(about = "Create changelog fragments")]
pub struct CreateCommand {
    /// The name of the fragment.
    #[arg(name = "NAME", help = "Write to the directory with this file name")]
    pub name: String,

    /// The fragment content, if it is passed as the argument.
    #[arg(
        short = 'c',
        long,
        name = "TEXT",
        help = "Pass the fragment content as this argument"
    )]
    pub content: Option<String>,

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
    /// Returns [`Error`] when any error is encountered.
    ///
    /// [`Error`]: crate::commands::create::Error
    pub fn run<D: AsRef<Path>>(self, directory: D) -> Result<(), crate::commands::create::Error> {
        create(directory, self.name, self.content, self.edit, self.add)
    }
}

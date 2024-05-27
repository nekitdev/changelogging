//! The application.

use std::path::{Path, PathBuf};

use clap::{Args, Parser, Subcommand};
use miette::Diagnostic;
use thiserror::Error;

use crate::{
    build::run,
    create::create,
    discover::discover,
    init::init,
    workspace::{load, Workspace},
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
    Build(#[from] crate::build::Error),
    /// `create` errors.
    Create(#[from] crate::create::Error),
}

/// Represents errors that can occur during application runs.
#[derive(Debug, Error, Diagnostic)]
#[error("error encoutered")]
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
    /// [`Error`]: crate::build::Error
    pub fn build(source: crate::build::Error) -> Self {
        Self::new(source.into())
    }

    /// Constructs [`Self`] from [`Error`].
    ///
    /// [`Error`]: crate::create::Error
    pub fn create(source: crate::create::Error) -> Self {
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

        init(globals.directory).map_err(|error| Error::init(error))?;

        let workspace = match globals.config {
            Some(path) => load(path).map_err(|error| Error::workspace(error))?,
            None => discover().map_err(|error| Error::discover(error))?,
        };

        if let Some(command) = self.command {
            match command {
                Command::Build(build) => {
                    build.run(workspace).map_err(|error| Error::build(error))?;
                }
                Command::Create(create) => {
                    let directory = workspace.options.into_config().paths.directory;

                    create
                        .run(directory)
                        .map_err(|error| Error::create(error))?;
                }
            }
        };

        Ok(())
    }
}

/// Represents `changelogging` subcommands.
#[derive(Debug, Subcommand)]
pub enum Command {
    /// The `build` command.
    #[command(about = "Build changelog entries from fragments")]
    Build(BuildCommand),
    /// The `create` command.
    #[command(about = "Create changelog fragments")]
    Create(CreateCommand),
}

/// Represents `build` commands.
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

    /// Whether to preview or write the build result.
    #[arg(
        short = 'p',
        long,
        action,
        help = "Output instead of writing to the file"
    )]
    pub preview: bool,
}

impl BuildCommand {
    /// Runs the `build` command.
    ///
    /// # Errors
    ///
    /// Returns [`Error`] when any error is encountered.
    ///
    /// [`Error`]: crate::build::Error
    pub fn run(self, workspace: Workspace<'_>) -> Result<(), crate::build::Error> {
        run(workspace, self.date, self.preview)
    }
}

/// Represents `create` commands.
#[derive(Debug, Args)]
#[command(about = "Create changelog fragments")]
pub struct CreateCommand {
    /// The name of the fragment.
    #[arg(name = "NAME", help = "Write to the directory with this file name")]
    pub name: String,

    /// The fragment content, if it is passed as the argument.
    #[arg(
        short,
        long,
        name = "TEXT",
        help = "Pass the fragment content as this argument"
    )]
    pub content: Option<String>,

    /// Whether to open the default editor to edit the fragment content.
    #[arg(
        short,
        long,
        action,
        help = "Open the default editor to edit the content"
    )]
    pub edit: bool,
}

impl CreateCommand {
    /// Runs the `create` command.
    ///
    /// # Errors
    ///
    /// Returns [`Error`] when any error is encountered.
    ///
    /// [`Error`]: crate::create::Error
    pub fn run<D: AsRef<Path>>(self, directory: D) -> Result<(), crate::create::Error> {
        create(directory, self.name, self.content, self.edit)
    }
}

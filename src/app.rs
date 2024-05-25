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

/// Represents errors that can occur during application runs.
#[derive(Debug, Error, Diagnostic)]
#[error(transparent)]
#[diagnostic(transparent)]
pub enum ErrorSource {
    Init(#[from] crate::init::Error),
    Discover(#[from] crate::discover::Error),
    Workspace(#[from] crate::workspace::Error),
    Build(#[from] crate::build::Error),
    Create(#[from] crate::create::Error),
}

#[derive(Debug, Error, Diagnostic)]
#[error("error encoutered")]
#[diagnostic(
    code(changelogging::app::run),
    help("see the report for more information")
)]
pub struct Error {
    #[source]
    #[diagnostic_source]
    source: ErrorSource,
}

impl Error {
    pub fn new(source: ErrorSource) -> Self {
        Self { source }
    }

    pub fn init(source: crate::init::Error) -> Self {
        Self::new(source.into())
    }

    pub fn discover(source: crate::discover::Error) -> Self {
        Self::new(source.into())
    }

    pub fn workspace(source: crate::workspace::Error) -> Self {
        Self::new(source.into())
    }

    pub fn build(source: crate::build::Error) -> Self {
        Self::new(source.into())
    }

    pub fn create(source: crate::create::Error) -> Self {
        Self::new(source.into())
    }
}

impl App {
    /// Runs the application.
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
    pub fn run<D: AsRef<Path>>(self, directory: D) -> Result<(), crate::create::Error> {
        create(directory, self.name, self.content, self.edit)
    }
}

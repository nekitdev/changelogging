//! Defines the `changelogging` application.

use std::{env::set_current_dir, path::PathBuf};

use clap::{Args, Parser, Subcommand};
use thiserror::Error;
use time::Date;

use crate::{
    build::builder_from_workspace,
    create::create_from_workspace,
    date::{parse_slice, today},
    workspace::{discover, workspace, Workspace},
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
    arg_required_else_help = true,
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
#[derive(Debug, Error)]
#[error(transparent)]
pub enum Error {
    /// I/O errors.
    Io(#[from] std::io::Error),
    /// Workspace discovery and loading errors.
    Workspace(#[from] crate::workspace::Error),
    /// `build` errors.
    Build(#[from] crate::build::Error),
    /// `create` errors.
    Create(#[from] crate::create::Error),
}

impl App {
    /// Runs the application.
    pub fn run(self) -> Result<(), Error> {
        let globals = self.globals;

        if let Some(directory) = globals.directory {
            set_current_dir(directory)?;
        };

        let workspace = globals.config.map_or_else(discover, workspace)?;

        if let Some(command) = self.command {
            match command {
                Command::Build(build) => build.run(workspace)?,
                Command::Create(create) => create.run(workspace)?,
            }
        };

        Ok(())
    }
}

/// Represents `changelogging` subcommands.
#[derive(Debug, Subcommand)]
pub enum Command {
    /// The `build` subcommand.
    Build(BuildCommand),
    /// The `create` subcommand.
    Create(CreateCommand),
}

/// Represents `build` commands.
#[derive(Debug, Args)]
#[command(about = "Build changelog entries from fragments")]
pub struct BuildCommand {
    /// The date to use. If not provided, [`today`] is used.
    #[arg(
        short = 'd',
        long,
        name = "DATE",
        value_parser = parse_slice,
        help = "Use the date provided instead of today",
    )]
    pub date: Option<Date>,

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
        let builder = builder_from_workspace(workspace, self.date.unwrap_or_else(today))?;

        if self.preview {
            builder.preview()?;
        } else {
            builder.write()?;
        }

        Ok(())
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
    pub fn run(self, workspace: Workspace<'_>) -> Result<(), crate::create::Error> {
        create_from_workspace(workspace, self.name, self.content, self.edit)
    }
}

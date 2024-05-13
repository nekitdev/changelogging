use std::fs::write;
use std::path::{Path, PathBuf};

use clap::Parser;
use thiserror::Error;
use time::Date;

use crate::build::builder_from_workspace;
use crate::date::{parse, today};
use crate::paths::load;
use crate::workspace::discover;

#[derive(Debug, Clone, PartialEq, Eq, Parser)]
#[command(version, about)]
#[command(propagate_version = true)]
pub enum Command {
    Build(BuildCommand),
    Create(CreateCommand),
}

#[derive(Debug, Clone, PartialEq, Eq, Parser)]
#[command(visible_alias = "b", about = "Build changelog entries from fragments")]
pub struct BuildCommand {
    /// Use the config from the file provided
    #[arg(short, long, name = "FILE")]
    pub config: Option<PathBuf>,
    /// Use the date provided instead of today
    #[arg(short, long, name = "DATE", value_parser = parse)]
    pub date: Option<Date>,
    /// Output instead of writing to the file
    #[arg(short, long, action)]
    pub preview: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Parser)]
#[command(visible_alias = "c", about = "Create changelog fragments")]
pub struct CreateCommand {
    /// Write to the directory with this file name
    #[arg(name = "NAME")]
    pub name: PathBuf,
    /// Use the config from the file provided
    #[arg(short, long, name = "FILE")]
    pub config: Option<PathBuf>,
    /// Open the default editor to enter the content
    #[arg(short, long, action)]
    pub edit: bool,
}

impl CreateCommand {
    pub fn path(&self) -> &Path {
        self.name.as_ref()
    }
}

#[derive(Debug, Error)]
#[error(transparent)]
pub enum BuildError {
    Config(#[from] crate::config::Error),
    Build(#[from] crate::build::Error),
    Date(#[from] crate::date::Error),
}

pub trait Runnable {
    type Error;

    fn run(&self) -> Result<(), Self::Error>;
}

impl Runnable for BuildCommand {
    type Error = BuildError;

    fn run(&self) -> Result<(), Self::Error> {
        let workspace = self.config.as_ref().map_or_else(discover, load)?;

        let date = self.date.unwrap_or_else(today);

        let builder = builder_from_workspace(workspace, date)?;

        if self.preview {
            let entry = builder.build()?;

            println!("{entry}");
        } else {
            builder.write()?;
        }

        Ok(())
    }
}

#[derive(Debug, Error)]
#[error(transparent)]
pub enum CreateError {
    Config(#[from] crate::config::Error),
    Edit(#[from] std::io::Error),
}

const PLACEHOLDER: &str = "Add the fragment content here.";

impl Runnable for CreateCommand {
    type Error = CreateError;

    fn run(&self) -> Result<(), Self::Error> {
        let workspace = self.config.as_ref().map_or_else(discover, load)?;

        let config = workspace.options.into_config();

        let path = config.paths.directory.join(self.path());

        if self.edit {
            edit::edit_file(path)?;
        } else {
            write(path, PLACEHOLDER)?;
        }

        Ok(())
    }
}

#[derive(Debug, Error)]
#[error(transparent)]
pub enum Error {
    Build(#[from] BuildError),
    Create(#[from] CreateError),
}

impl Runnable for Command {
    type Error = Error;

    fn run(&self) -> Result<(), Self::Error> {
        match self {
            Command::Build(build) => build.run()?,
            Command::Create(create) => create.run()?,
        };

        Ok(())
    }
}

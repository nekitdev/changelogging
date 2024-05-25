//! Creating changelog fragments.
//!
//! The [`create`] function implements the `create` subcommand.

use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};

use edit::edit_file;
use miette::Diagnostic;
use thiserror::Error;

use crate::fragment::{validate, ParseError};

#[derive(Debug, Error, Diagnostic)]
#[error("opening failed")]
#[diagnostic(
    code(changelogging::create::open),
    help("check that the file does not already exist and the fragments directory is accessible")
)]
pub struct OpenError(#[from] pub std::io::Error);

#[derive(Debug, Error, Diagnostic)]
#[error("writing failed")]
#[diagnostic(
    code(changelogging::create::write),
    help("make sure the fragment file is accessible")
)]
pub struct WriteError(#[from] pub std::io::Error);

#[derive(Debug, Error, Diagnostic)]
#[error("editing failed")]
#[diagnostic(
    code(changelogging::create::edit),
    help("check your default editor configuration")
)]
pub struct EditError(#[from] pub std::io::Error);

#[derive(Debug, Error, Diagnostic)]
#[error(transparent)]
#[diagnostic(transparent)]
pub enum ErrorSource {
    Parse(#[from] ParseError),
    Open(#[from] OpenError),
    Write(#[from] WriteError),
    Edit(#[from] EditError),
}

#[derive(Debug, Error, Diagnostic)]
#[error("failed to create fragment `{path}`")]
#[diagnostic(
    code(changelogging::create::create),
    help("see the report for more information")
)]
pub struct Error {
    #[source]
    #[diagnostic_source]
    pub source: ErrorSource,
    pub path: PathBuf,
}

impl Error {
    pub fn new<P: AsRef<Path>>(source: ErrorSource, path: P) -> Self {
        let path = path.as_ref().to_owned();

        Self { source, path }
    }

    pub fn parse<P: AsRef<Path>>(source: ParseError, path: P) -> Self {
        Self::new(source.into(), path)
    }

    pub fn open<P: AsRef<Path>>(source: OpenError, path: P) -> Self {
        Self::new(source.into(), path)
    }

    pub fn write<P: AsRef<Path>>(source: WriteError, path: P) -> Self {
        Self::new(source.into(), path)
    }

    pub fn edit<P: AsRef<Path>>(source: EditError, path: P) -> Self {
        Self::new(source.into(), path)
    }

    pub fn new_open<P: AsRef<Path>>(source: std::io::Error, path: P) -> Self {
        Self::open(OpenError(source), path)
    }

    pub fn new_write<P: AsRef<Path>>(source: std::io::Error, path: P) -> Self {
        Self::write(WriteError(source), path)
    }

    pub fn new_edit<P: AsRef<Path>>(source: std::io::Error, path: P) -> Self {
        Self::edit(EditError(source), path)
    }
}

const PLACEHOLDER: &str = "Add the fragment content here.";

/// Creates changelog fragments.
pub fn create<D: AsRef<Path>, S: AsRef<str>, C: AsRef<str>>(
    directory: D,
    name: S,
    content: Option<C>,
    edit: bool,
) -> Result<(), Error> {
    let name = name.as_ref();

    let joined = directory.as_ref().join(name);

    let path = joined.as_path();

    validate(name).map_err(|error| Error::parse(error, path))?;

    let mut file = File::options()
        .create_new(true)
        .write(true)
        .open(path)
        .map_err(|error| Error::new_open(error, path))?;

    let string = content.as_ref().map_or(PLACEHOLDER, |slice| slice.as_ref());

    writeln!(file, "{string}").map_err(|error| Error::new_write(error, path))?;

    if edit {
        edit_file(path).map_err(|error| Error::new_edit(error, path))?;
    }

    Ok(())
}

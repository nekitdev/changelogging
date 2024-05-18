use std::{fs::File, io::Write};

use edit::edit_file;
use thiserror::Error;

use crate::{
    config::Config,
    context::Context,
    fragments::{validate, ParseError},
    workspace::Workspace,
};

#[derive(Debug, Error)]
#[error(transparent)]
pub enum Error {
    Io(#[from] std::io::Error),
    Parse(#[from] ParseError),
}

const PLACEHOLDER: &str = "Add the fragment content here.";

pub fn create<S: AsRef<str>, C: AsRef<str>>(
    _context: Context<'_>,
    config: Config<'_>,
    name: S,
    content: Option<C>,
    edit: bool,
) -> Result<(), Error> {
    validate(name.as_ref())?;

    let path = config.paths.directory.join(name.as_ref());

    let mut file = File::options().create_new(true).write(true).open(path.as_path())?;

    let string = content.as_ref().map(|slice| slice.as_ref()).unwrap_or(PLACEHOLDER);

    writeln!(file, "{string}")?;

    if edit {
        edit_file(path)?;
    }

    Ok(())
}

pub fn create_from_workspace<S: AsRef<str>, C: AsRef<str>>(
    workspace: Workspace<'_>,
    name: S,
    content: Option<C>,
    edit: bool,
) -> Result<(), Error> {
    create(workspace.context, workspace.options.into_config(), name, content, edit)
}

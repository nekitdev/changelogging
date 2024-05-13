use std::env::current_dir;

use serde::{Deserialize, Serialize};

use crate::{
    config::Error,
    context::Context,
    macros::{impl_from_path_with_parse, impl_from_str_with_toml},
    options::Options,
    paths::load,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Workspace<'w> {
    pub context: Context<'w>,
    #[serde(flatten)]
    pub options: Options<'w>,
}

impl_from_str_with_toml!(Workspace<'_>);
impl_from_path_with_parse!(Workspace<'_>, Error);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Tools<'t> {
    pub changelogging: Option<Workspace<'t>>,
}

impl_from_str_with_toml!(Tools<'_>);
impl_from_path_with_parse!(Tools<'_>, Error);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PyProject<'p> {
    pub tool: Option<Tools<'p>>,
}

impl_from_str_with_toml!(PyProject<'_>);
impl_from_path_with_parse!(PyProject<'_>, Error);

const CHANGELOGGING: &str = "changelogging.toml";
const PYPROJECT: &str = "pyproject.toml";

pub fn pyproject_workspace(pyproject: PyProject<'_>) -> Option<Workspace<'_>> {
    pyproject.tool.and_then(|tools| tools.changelogging)
}

pub fn discover() -> Result<Workspace<'static>, Error> {
    let mut path = current_dir()?;

    // use `pyproject.toml` iff `[tool.changelogging]` is in there

    path.push(PYPROJECT);

    if path.try_exists()? {
        let pyproject: PyProject = load(path.as_path())?;

        if let Some(workspace) = pyproject_workspace(pyproject) {
            return Ok(workspace);
        }
    }

    // fall back to `changelogging.toml`

    path.pop();

    path.push(CHANGELOGGING);

    load(path)
}

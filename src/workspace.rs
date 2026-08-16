//! Discovering and loading workspaces.
//!
//! This module provides two notable structures, [`Workspace`] and [`PyProject`].
//!
//! See also [`context`] and [`config`].
//!
//! [`context`]: crate::context
//! [`config`]: crate::config

use ownership::IntoOwned;
use serde::{Deserialize, Serialize};

use crate::{config::core::Config, context::Context, toml::internals::impl_from_path_with_toml};

/// Combines [`Context`] and [`Config`] into one structure.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, IntoOwned)]
pub struct Workspace<'w> {
    /// The context of the workspace.
    pub context: Context<'w>,

    /// The config of the workspace.
    ///
    /// This field is flattened during (de)serialization.
    #[serde(flatten)]
    pub config: Config<'w>,
}

impl<'w> Workspace<'w> {
    pub const fn new(context: Context<'w>, config: Config<'w>) -> Self {
        Self { context, config }
    }

    pub const fn context(&self) -> &Context<'w> {
        &self.context
    }

    pub const fn context_mut(&mut self) -> &mut Context<'w> {
        &mut self.context
    }

    pub const fn config(&self) -> &Config<'w> {
        &self.config
    }

    pub const fn config_mut(&mut self) -> &mut Config<'w> {
        &mut self.config
    }

    pub fn get(self) -> (Context<'w>, Config<'w>) {
        (self.context, self.config)
    }
}

impl_from_path_with_toml!(Workspace<'_>);

pub type OwnedWorkspace = Workspace<'static>;

/// Represents `tool` sections in `pyproject.toml` files.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, IntoOwned)]
pub struct Tools<'t> {
    /// The `changelogging` section.
    pub changelogging: Option<Workspace<'t>>,
}

impl<'t> Tools<'t> {
    pub const fn new(changelogging: Option<Workspace<'t>>) -> Self {
        Self { changelogging }
    }

    pub const fn some(workspace: Workspace<'t>) -> Self {
        Self::new(Some(workspace))
    }

    pub const fn none() -> Self {
        Self::new(None)
    }

    pub const fn changelogging(&self) -> Option<&Workspace<'t>> {
        self.changelogging.as_ref()
    }

    pub const fn changelogging_mut(&mut self) -> Option<&mut Workspace<'t>> {
        self.changelogging.as_mut()
    }

    pub fn get(self) -> Option<Workspace<'t>> {
        self.changelogging
    }
}

/// Represents structures of `pyproject.toml` files.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, IntoOwned)]
pub struct PyProject<'p> {
    /// The `tool` section.
    pub tool: Option<Tools<'p>>,
}

impl_from_path_with_toml!(PyProject<'_>);

impl<'p> PyProject<'p> {
    pub const fn new(tool: Option<Tools<'p>>) -> Self {
        Self { tool }
    }

    pub const fn some(tools: Tools<'p>) -> Self {
        Self::new(Some(tools))
    }

    pub const fn none() -> Self {
        Self::new(None)
    }

    pub const fn tool(&self) -> Option<&Tools<'p>> {
        self.tool.as_ref()
    }

    pub const fn tool_mut(&mut self) -> Option<&mut Tools<'p>> {
        self.tool.as_mut()
    }

    pub fn get(self) -> Option<Tools<'p>> {
        self.tool
    }

    pub const fn workspace(&self) -> Option<&Workspace<'p>> {
        if let Some(tools) = self.tool() {
            tools.changelogging()
        } else {
            None
        }
    }

    pub const fn workspace_mut(&mut self) -> Option<&mut Workspace<'p>> {
        if let Some(tools) = self.tool_mut() {
            tools.changelogging_mut()
        } else {
            None
        }
    }

    /// Converts [`Self`] into [`Workspace`], provided the `tool.changelogging` section is present.
    pub fn into_workspace(self) -> Option<Workspace<'p>> {
        self.get().and_then(|tools| tools.get())
    }
}

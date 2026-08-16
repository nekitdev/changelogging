use std::borrow::Cow;

use bon::Builder;
use handlebars::{Handlebars as Registry, RenderError, TemplateError, no_escape};
use non_empty_str::{NonEmptyStr, const_non_empty_str};
use serde::Serialize;
use thiserror::Error;

use crate::{
    by::By, config::formats::Formats, context::Context, date::Date, fragments::fragment::Fragment,
};

#[derive(Debug, Error)]
#[error("failed to create renderer: {error}")]
pub struct RegisterError {
    #[from]
    error: TemplateError,
}

impl RegisterError {
    pub const fn new(error: TemplateError) -> Self {
        Self { error }
    }

    pub fn get(self) -> TemplateError {
        self.error
    }
}

#[derive(Debug, Error)]
#[error("failed to render `{TITLE}`: {error}")]
pub struct RenderTitleError {
    #[from]
    error: RenderError,
}

impl RenderTitleError {
    pub const fn new(error: RenderError) -> Self {
        Self { error }
    }

    pub fn get(self) -> RenderError {
        self.error
    }
}

#[derive(Debug, Error)]
#[error("failed to render `{FRAGMENT}`: {error}")]
pub struct RenderFragmentError {
    #[from]
    error: RenderError,
}

type Str<'s> = Cow<'s, str>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Builder)]
struct RenderTitle<'t> {
    #[serde(flatten)]
    context: &'t Context<'t>,
    #[builder(into)]
    date: Str<'t>,
}

impl<'t> RenderTitle<'t> {
    fn new(context: &'t Context<'t>, date: Date) -> Self {
        let string = date.to_string();

        Self::builder().context(context).date(string).build()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Builder)]
struct RenderFragment<'f> {
    #[serde(flatten)]
    context: &'f Context<'f>,
    #[serde(flatten)]
    fragment: &'f Fragment<'f>,
}

impl<'f> RenderFragment<'f> {
    fn new(context: &'f Context<'f>, fragment: &'f Fragment<'f>) -> Self {
        Self::builder().context(context).fragment(fragment).build()
    }
}

pub struct Renderer<'r> {
    registry: Registry<'r>,
}

pub const TITLE: &NonEmptyStr = const_non_empty_str!("title");
pub const FRAGMENT: &NonEmptyStr = const_non_empty_str!("fragment");

impl<'r> Renderer<'r> {
    pub const fn new(registry: Registry<'r>) -> Self {
        Self { registry }
    }
}

impl Renderer<'_> {
    pub fn empty() -> Self {
        let registry = Registry::new();

        Self::new(registry)
    }

    pub fn with(formats: &Formats<'_>) -> Result<Self, RegisterError> {
        let mut renderer = Self::empty();

        renderer.enable_strict();

        renderer.disable_escape();

        renderer.register_title(formats.title.as_ref())?;
        renderer.register_fragment(formats.fragment.as_ref())?;

        Ok(renderer)
    }

    pub fn render_title(
        &self,
        context: &Context<'_>,
        date: Date,
    ) -> Result<String, RenderTitleError> {
        let render_title = RenderTitle::new(context, date);

        let rendered = self.registry.render(TITLE, render_title.by_ref())?;

        Ok(rendered)
    }

    pub fn render_fragment(
        &self,
        context: &Context<'_>,
        fragment: &Fragment<'_>,
    ) -> Result<String, RenderFragmentError> {
        let render_fragment = RenderFragment::new(context, fragment);

        let rendered = self.registry.render(FRAGMENT, render_fragment.by_ref())?;

        Ok(rendered)
    }

    pub fn enable_strict(&mut self) {
        self.registry.set_strict_mode(true);
    }

    pub fn disable_escape(&mut self) {
        self.registry.register_escape_fn(no_escape);
    }

    pub fn register_title<T: AsRef<str>>(&mut self, title: T) -> Result<(), RegisterError> {
        self.registry.register_template_string(TITLE, title)?;

        Ok(())
    }

    pub fn register_fragment<F: AsRef<str>>(&mut self, fragment: F) -> Result<(), RegisterError> {
        self.registry.register_template_string(FRAGMENT, fragment)?;

        Ok(())
    }
}

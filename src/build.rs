use std::{
    borrow::Cow,
    fs::{read_to_string, write},
    iter::{once, repeat},
};

use handlebars::{Handlebars, RenderError, TemplateError};
use itertools::Itertools;
use serde::Serialize;
use textwrap::{fill, Options as WrapOptions};
use thiserror::Error;
use time::Date;

use crate::{
    config::{Config, Types},
    context::Context,
    fragments::{Fragment, Fragments, Sections},
    paths::{load, FromDir},
    workspace::Workspace,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RenderTitleData<'t> {
    #[serde(flatten)]
    pub context: &'t Context<'t>,
    pub date: Cow<'t, str>,
}

impl<'t> RenderTitleData<'t> {
    pub fn new(context: &'t Context<'_>, date: Date) -> Self {
        Self {
            context,
            date: date.to_string().into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RenderFragmentData<'f> {
    #[serde(flatten)]
    pub context: &'f Context<'f>,
    #[serde(flatten)]
    pub fragment: &'f Fragment<'f>,
}

impl<'f> RenderFragmentData<'f> {
    pub fn new(context: &'f Context<'_>, fragment: &'f Fragment<'_>) -> Self {
        Self { context, fragment }
    }
}

#[derive(Debug, Clone)]
pub struct Builder<'b> {
    pub context: Context<'b>,
    pub config: Config<'b>,
    pub date: Date,
    pub renderer: Handlebars<'b>,
}

#[derive(Debug, Error)]
#[error(transparent)]
pub enum Error {
    Template(#[from] TemplateError),
    Render(#[from] RenderError),
    Format(#[from] std::fmt::Error),
    Io(#[from] std::io::Error),
}

const TITLE: &str = "title";
const FRAGMENT: &str = "fragment";

impl<'b> Builder<'b> {
    pub fn from_workspace(workspace: Workspace<'b>, date: Date) -> Result<Self, TemplateError> {
        Self::new(workspace.context, workspace.options.into_config(), date)
    }

    pub fn new(
        context: Context<'b>,
        config: Config<'b>,
        date: Date,
    ) -> Result<Self, TemplateError> {
        let mut renderer = Handlebars::new();

        let formats = config.formats();

        renderer.set_strict_mode(true);

        renderer.register_template_string(TITLE, formats.title.as_ref())?;
        renderer.register_template_string(FRAGMENT, formats.fragment.as_ref())?;

        Ok(Self {
            context,
            config,
            date,
            renderer,
        })
    }
}

const SPACE: char = ' ';
const DOUBLE_NEW_LINE: &str = "\n\n";
const NO_SIGNIFICANT_CHANGES: &str = "No significant changes.";

fn heading(character: char, level: usize) -> String {
    repeat(character).take(level).chain(once(SPACE)).collect()
}

fn indent(character: char) -> String {
    once(character).chain(once(SPACE)).collect()
}

impl Builder<'_> {
    pub fn context(&self) -> &Context<'_> {
        &self.context
    }

    pub fn config(&self) -> &Config<'_> {
        &self.config
    }

    pub fn types(&self) -> &Types<'_> {
        &self.config.types
    }

    // BUILDING

    pub fn write(&self) -> Result<(), Error> {
        let contents = self.prepare()?;

        write(self.config.paths.output.as_ref(), contents)?;

        Ok(())
    }

    pub fn prepare(&self) -> Result<String, Error> {
        let contents = read_to_string(self.config.paths.output.as_ref())?;

        let mut string = String::new();

        let start = self.config.start.as_ref();

        let entry = self.build()?;

        if let Some((before, after)) = contents.split_once(start) {
            string.push_str(before);
            string.push_str(start);
            string.push_str(DOUBLE_NEW_LINE);
            string.push_str(entry.as_ref());
            string.push_str(after);
        } else {
            string.push_str(entry.as_ref());
            string.push_str(DOUBLE_NEW_LINE);
            string.push_str(contents.as_ref());
        }

        Ok(string)
    }

    pub fn build(&self) -> Result<String, Error> {
        let mut string = self.build_title()?;

        string.push_str(DOUBLE_NEW_LINE);

        let sections = self.collect_sections()?;

        let built = self.build_sections(&sections)?;

        let contents = if built.is_empty() {
            NO_SIGNIFICANT_CHANGES
        } else {
            built.as_ref()
        };

        string.push_str(contents);

        Ok(string)
    }

    pub fn build_title(&self) -> Result<String, RenderError> {
        let mut string = self.entry_heading();

        let title = self.render_title()?;

        string.push_str(title.as_ref());

        Ok(string)
    }

    pub fn build_section_title<S: AsRef<str>>(&self, title: S) -> String {
        let mut string = self.section_heading();

        string.push_str(title.as_ref());

        string
    }

    pub fn build_fragment(&self, fragment: &Fragment<'_>) -> Result<String, RenderError> {
        let string = self.render_fragment(fragment)?;

        Ok(self.wrap(string))
    }

    pub fn build_fragments(&self, fragments: &Fragments<'_>) -> Result<String, Error> {
        let string = fragments
            .iter()
            .map(|fragment| self.build_fragment(fragment))
            .process_results(|iterator| iterator.into_iter().join(DOUBLE_NEW_LINE))?;

        Ok(string)
    }

    pub fn build_section<S: AsRef<str>>(
        &self,
        title: S,
        fragments: &Fragments<'_>,
    ) -> Result<String, Error> {
        let mut string = self.build_section_title(title);

        let built = self.build_fragments(fragments)?;

        string.push_str(DOUBLE_NEW_LINE);
        string.push_str(built.as_ref());

        Ok(string)
    }

    pub fn build_sections(&self, sections: &Sections<'_>) -> Result<String, Error> {
        let types = self.types();

        let string = self
            .config
            .order
            .iter()
            .filter_map(|name| types.get(name).zip(sections.get(name)))
            .map(|(title, fragments)| self.build_section(title, fragments))
            .process_results(|iterator| iterator.into_iter().join(DOUBLE_NEW_LINE))?;

        Ok(string)
    }

    // WRAPPING

    pub fn wrap<S: AsRef<str>>(&self, string: S) -> String {
        let initial_indent = indent(self.config.indents.bullet);
        let subsequent_indent = indent(SPACE);

        let options = WrapOptions::new(self.config.wrap)
            .initial_indent(initial_indent.as_ref())
            .subsequent_indent(subsequent_indent.as_ref());

        fill(string.as_ref(), options)
    }

    // RENDERING

    pub fn render_title(&self) -> Result<String, RenderError> {
        let data = RenderTitleData::new(self.context(), self.date);

        self.renderer.render(TITLE, &data)
    }

    pub fn render_fragment(&self, fragment: &Fragment<'_>) -> Result<String, RenderError> {
        let data = RenderFragmentData::new(self.context(), fragment);

        self.renderer.render(FRAGMENT, &data)
    }

    // COLLECTING

    pub fn collect_sections(&self) -> Result<Sections<'_>, Error> {
        let mut sections = Sections::new();

        self.iter_fragments()?
            .filter_map(|result| result.ok())
            .for_each(|fragment| {
                sections
                    .entry(fragment.name.clone())
                    .or_default()
                    .push(fragment)
            });

        sections.values_mut().for_each(|section| section.sort());

        Ok(sections)
    }

    pub fn iter_fragments(&self) -> Result<FromDir<Fragment<'_>>, Error> {
        let iterator = load(self.config.paths.directory.as_ref())?;

        Ok(iterator)
    }

    // HEADING

    pub fn level_heading(&self, level: usize) -> String {
        heading(self.config.indents.heading, level)
    }

    pub fn entry_heading(&self) -> String {
        self.level_heading(self.config.levels.entry)
    }

    pub fn section_heading(&self) -> String {
        self.level_heading(self.config.levels.section)
    }
}

pub fn builder<'a>(
    context: Context<'a>,
    config: Config<'a>,
    date: Date,
) -> Result<Builder<'a>, Error> {
    let builder = Builder::new(context, config, date)?;

    Ok(builder)
}

pub fn builder_from_workspace(workspace: Workspace<'_>, date: Date) -> Result<Builder<'_>, Error> {
    let builder = Builder::from_workspace(workspace, date)?;

    Ok(builder)
}

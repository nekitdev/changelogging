use std::iter::{once, repeat_n};

use bon::Builder;
use from_path::Load;
use itertools::Itertools;
use non_empty_iter::{NonEmptyIterator, TryIntoNonEmptyIterator};
use non_empty_str::{NonEmptyStr, NonEmptyString, const_non_empty_str};
use non_zero_size::Size;
use textwrap::{Options, WordSeparator, WordSplitter, fill};
use thiserror::Error;

use crate::{
    build::renderer::{RegisterError, RenderFragmentError, RenderTitleError, Renderer},
    by::By,
    config::core::Config,
    context::Context,
    date::Date,
    file::{ReadError as ReadFileError, WriteError as WriteFileError, read, write},
    fragments::{
        collect::{Collector, Error as CollectError, ReadError},
        fragment::{Fragment, Fragments, Sections},
        items::{Paths, SectionsPaths},
        types::Title,
    },
};

const SPACE: char = ' ';
const NEW_LINE: char = '\n';

const DOUBLE_NEW_LINE: &NonEmptyStr = const_non_empty_str!("\n\n");
const NO_SIGNIFICANT_CHANGES: &NonEmptyStr = const_non_empty_str!("No significant changes.");

const NON_EMPTY_HEADING: &NonEmptyStr = const_non_empty_str!("expected non-empty heading");
const NON_EMPTY_INDENT: &NonEmptyStr = const_non_empty_str!("expected non-empty indent");

fn heading(character: char, level: Size) -> NonEmptyString {
    repeat_n(character, level.get())
        .chain(once(SPACE))
        .try_into_non_empty_iter()
        .expect(NON_EMPTY_HEADING)
        .collect_non_empty()
}

fn indent(character: char) -> NonEmptyString {
    once(character)
        .chain(once(SPACE))
        .try_into_non_empty_iter()
        .expect(NON_EMPTY_INDENT)
        .collect_non_empty()
}

#[derive(Builder)]
pub struct PartialBuilder<'b> {
    pub context: Context<'b>,
    pub config: Config<'b>,
    pub date: Date,
}

impl<'b> PartialBuilder<'b> {
    pub const fn context(&self) -> &Context<'b> {
        &self.context
    }

    pub const fn config(&self) -> &Config<'b> {
        &self.config
    }

    pub const fn date(&self) -> Date {
        self.date
    }

    // WRAPPING

    pub fn wrap<S: AsRef<str>>(&self, string: S) -> String {
        let initial_indent = indent(self.config().indents().bullet());
        let subsequent_indent = indent(SPACE);

        let options = Options::new(self.config().wrap().full_get()) // use wrap width
            .break_words(false) // do not break words
            .word_separator(WordSeparator::AsciiSpace) // use space as word separator
            .word_splitter(WordSplitter::NoHyphenation) // do not hyphenate
            .initial_indent(initial_indent.by_ref())
            .subsequent_indent(subsequent_indent.by_ref());

        fill(string.as_ref(), options)
    }

    // HEADINGS

    fn level_heading(&self, level: Size) -> NonEmptyString {
        heading(self.config().indents().heading(), level)
    }

    pub fn entry_heading(&self) -> NonEmptyString {
        self.level_heading(self.config().levels().entry())
    }

    pub fn section_heading(&self) -> NonEmptyString {
        self.level_heading(self.config().levels().section())
    }
}

#[derive(Builder)]
pub struct Builder<'b> {
    pub partial: PartialBuilder<'b>,
    pub renderer: Renderer<'b>,
}

#[derive(Debug, Error)]
#[error(transparent)]
pub enum CollectorError {
    Read(#[from] ReadError),
    Collect(#[from] CollectError),
}

#[derive(Debug, Error)]
#[error(transparent)]
pub enum BuildError {
    Collector(#[from] CollectorError),
    Title(#[from] RenderTitleError),
    Fragment(#[from] RenderFragmentError),
}

#[derive(Debug, Error)]
#[error(transparent)]
pub enum WriteError {
    Build(#[from] BuildError),
    Read(#[from] ReadFileError),
    Write(#[from] WriteFileError),
}

pub const DOUBLE_NEW_LINE_LENGTH: usize = 2;
pub const NEW_LINE_LENGTH: usize = 1;

impl<'b> Builder<'b> {
    // ACCESSING

    pub const fn partial(&self) -> &PartialBuilder<'b> {
        &self.partial
    }

    pub const fn renderer(&self) -> &Renderer<'b> {
        &self.renderer
    }

    // REGISTERING

    pub fn registering(partial: PartialBuilder<'b>) -> Result<Self, RegisterError> {
        let renderer = Renderer::with(partial.config().formats())?;

        let builder = Self::builder().partial(partial).renderer(renderer).build();

        Ok(builder)
    }

    // RENDERING

    pub fn render_title(&self) -> Result<String, RenderTitleError> {
        self.renderer()
            .render_title(self.partial().context(), self.partial().date())
    }

    pub fn render_fragment(&self, fragment: &Fragment<'_>) -> Result<String, RenderFragmentError> {
        if fragment.partial().id().is_int() {
            self.renderer()
                .render_fragment(self.partial().context(), fragment)
        } else {
            let string = fragment.content().string().to_owned().into_string();

            Ok(string)
        }
    }

    // WRITING

    pub fn write(&self) -> Result<(), WriteError> {
        let entry = self.build()?;

        let path = self.partial().config().paths().output_path();

        let contents = read(path)?;

        let start = self.partial().config().start().marker();

        let marker = start.as_str();

        let string = if let Some((before, after)) = contents.split_once(marker) {
            let mut capacity = before.len()
                + marker.len()
                + DOUBLE_NEW_LINE_LENGTH
                + entry.len().get()
                + NEW_LINE_LENGTH;

            let trimmed = after.trim_start();

            let push = !trimmed.is_empty();

            if push {
                capacity += NEW_LINE_LENGTH + trimmed.len();
            }

            let mut output = String::with_capacity(capacity);

            output.push_str(before);

            output.push_str(marker);

            output.push_str(DOUBLE_NEW_LINE);

            output.push(NEW_LINE);

            if push {
                output.push(NEW_LINE);

                output.push_str(trimmed);
            }

            output
        } else {
            let mut capacity = entry.len().get() + NEW_LINE_LENGTH;

            let trimmed = contents.trim_start();

            let push = !trimmed.is_empty();

            if push {
                capacity += NEW_LINE_LENGTH + trimmed.len();
            }

            let mut output = String::with_capacity(capacity);

            output.push_str(entry.as_str());

            output.push(NEW_LINE);

            if push {
                output.push(NEW_LINE);

                output.push_str(trimmed);
            }

            output
        };

        write(path, string)?;

        Ok(())
    }

    // PREVIEWING

    pub fn preview(&self) -> Result<(), BuildError> {
        let string = self.build()?;

        println!("{string}");

        Ok(())
    }

    // BUILDING

    pub fn build(&self) -> Result<NonEmptyString, BuildError> {
        let mut string = self.build_title()?;

        string.push_str(DOUBLE_NEW_LINE);

        let sections = self.collect_sections()?;

        let built = self.build_sections(&sections)?;

        let contents = if built.is_empty() {
            NO_SIGNIFICANT_CHANGES
        } else {
            built.as_str()
        };

        string.push_str(contents);

        Ok(string)
    }

    pub fn build_title(&self) -> Result<NonEmptyString, RenderTitleError> {
        let mut string = self.partial().entry_heading();

        let title = self.render_title()?;

        string.push_str(title.as_str());

        Ok(string)
    }

    pub fn build_fragment(&self, fragment: &Fragment<'_>) -> Result<String, RenderFragmentError> {
        let string = self.render_fragment(fragment)?;

        let wrapped = self.partial().wrap(string);

        Ok(wrapped)
    }

    pub fn build_fragments(
        &self,
        fragments: &Fragments<'_>,
    ) -> Result<String, RenderFragmentError> {
        let string = fragments
            .iter()
            .map(|fragment| self.build_fragment(fragment))
            .process_results(|mut iterator| iterator.join(DOUBLE_NEW_LINE))?;

        Ok(string)
    }

    pub fn build_section_title(&self, title: &Title<'_>) -> NonEmptyString {
        let mut string = self.partial().section_heading();

        string.push_str(title.string());

        string
    }

    pub fn build_section(
        &self,
        title: &Title<'_>,
        fragments: &Fragments<'_>,
    ) -> Result<NonEmptyString, RenderFragmentError> {
        let mut string = self.build_section_title(title);

        let built = self.build_fragments(fragments)?;

        string.push_str(DOUBLE_NEW_LINE);
        string.push_str(built.as_str());

        Ok(string)
    }

    pub fn build_sections(&self, sections: &Sections<'_>) -> Result<String, RenderFragmentError> {
        let types = self.partial().config().types().merge_defaults();

        let string = self
            .partial()
            .config()
            .order()
            .names()
            .iter()
            .filter_map(|name| types.find(name).zip(sections.get(name)))
            .map(|(title, fragments)| self.build_section(title, fragments))
            .process_results(|mut iterator| iterator.join(DOUBLE_NEW_LINE))?;

        Ok(string)
    }

    // COLLECTING

    pub fn collector(&self) -> Result<Collector, ReadError> {
        self.partial().config().paths().directory_path().load()
    }

    pub fn collect_paths(&self) -> Result<Paths<'_>, CollectorError> {
        let mut paths = Paths::new();

        self.collector()?.process_results(|iterator| {
            iterator.for_each(|item| {
                paths.push(item.into_path());
            })
        })?;

        Ok(paths)
    }

    pub fn collect_sections(&self) -> Result<Sections<'_>, CollectorError> {
        let sections = self
            .collect()?
            .into_iter()
            .map(|(name, items)| (name, items.into_iter().map(|item| item.extract()).collect()))
            .collect();

        Ok(sections)
    }

    pub fn collect(&self) -> Result<SectionsPaths<'_>, CollectorError> {
        let mut sections_paths = SectionsPaths::new();

        self.collector()?.process_results(|iterator| {
            iterator.for_each(|item| {
                sections_paths
                    .entry(item.fragment().partial().name().clone())
                    .or_default() // creates if not present
                    .push(item)
            })
        })?;

        sections_paths
            .values_mut()
            .for_each(|section| section.sort());

        Ok(sections_paths)
    }
}

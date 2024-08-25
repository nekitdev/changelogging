//! Building changelogs from fragments.

use std::{
    borrow::Cow,
    fs::{read_dir, File},
    io::{read_to_string, Write},
    iter::{once, repeat},
    path::PathBuf,
};

use handlebars::{no_escape, Handlebars, RenderError, TemplateError};
use itertools::Itertools;
use miette::Diagnostic;
use serde::Serialize;
use textwrap::{fill, Options as WrapOptions, WordSeparator, WordSplitter};
use thiserror::Error;
use time::Date;

use crate::{
    config::{Config, Level},
    context::Context,
    fragment::{is_valid_path, Fragment, Fragments, Sections},
    load::load,
    workspace::Workspace,
};

/// Represents errors that can occur during builder initialization.
#[derive(Debug, Error, Diagnostic)]
#[error("failed to initialize the renderer")]
#[diagnostic(
    code(changelogging::builder::init),
    help("make sure the formats configuration is valid")
)]
pub struct InitError(#[from] pub TemplateError);

/// Represents errors that can occur when building titles.
#[derive(Debug, Error, Diagnostic)]
#[error("failed to build the title")]
#[diagnostic(
    code(changelogging::builder::build_title),
    help("make sure the formats configuration is valid")
)]
pub struct BuildTitleError(#[from] pub RenderError);

/// Represents errors that can occur when building fragments.
#[derive(Debug, Error, Diagnostic)]
#[error("failed to build the fragment")]
#[diagnostic(
    code(changelogging::builder::build_fragment),
    help("make sure the formats configuration is valid")
)]
pub struct BuildFragmentError(#[from] pub RenderError);

/// Represents errors that can occur when reading from files.
#[derive(Debug, Error, Diagnostic)]
#[error("failed to read from `{path}`")]
#[diagnostic(
    code(changelogging::builder::read_file),
    help("check whether the file exists and is accessible")
)]
pub struct ReadFileError {
    /// The underlying I/O error.
    pub source: std::io::Error,
    /// The path provided.
    pub path: PathBuf,
}

impl ReadFileError {
    /// Constructs [`Self`].
    pub fn new(source: std::io::Error, path: PathBuf) -> Self {
        Self { source, path }
    }
}

/// Represents errors that can occur when writing to files.
#[derive(Debug, Error, Diagnostic)]
#[error("failed to write to `{path}`")]
#[diagnostic(
    code(changelogging::builder::write_file),
    help("check whether the file exists and is accessible")
)]
pub struct WriteFileError {
    /// The underlying I/O error.
    pub source: std::io::Error,
    /// The path provided.
    pub path: PathBuf,
}

impl WriteFileError {
    /// Constructs [`Self`].
    pub fn new(source: std::io::Error, path: PathBuf) -> Self {
        Self { source, path }
    }
}

/// Represents errors that can occur when opening files.
#[derive(Debug, Error, Diagnostic)]
#[error("failed to open `{path}`")]
#[diagnostic(
    code(changelogging::builder::open_file),
    help("check whether the file exists and is accessible")
)]
pub struct OpenFileError {
    /// The underlying I/O error.
    pub source: std::io::Error,
    /// The path provided.
    pub path: PathBuf,
}

impl OpenFileError {
    /// Constructs [`Self`].
    pub fn new(source: std::io::Error, path: PathBuf) -> Self {
        Self { source, path }
    }
}

/// Represents errors that can occur when reading directories.
#[derive(Debug, Error, Diagnostic)]
#[error("failed to read directory")]
#[diagnostic(
    code(changelogging::builder::read_directory),
    help("make sure the directory is accessible")
)]
pub struct ReadDirectoryError(#[from] std::io::Error);

/// Represents errors that can occur during iterating over directories.
#[derive(Debug, Error, Diagnostic)]
#[error("failed to iterate directory")]
#[diagnostic(
    code(changelogging::builder::iter_directory),
    help("make sure the directory is accessible")
)]
pub struct IterDirectoryError(#[from] std::io::Error);

/// Represents sources of errors that can occur during fragment collection.
#[derive(Debug, Error, Diagnostic)]
#[error(transparent)]
#[diagnostic(transparent)]
pub enum CollectErrorSource {
    /// Read directory errors.
    ReadDirectory(#[from] ReadDirectoryError),
    /// Iterate directory errors.
    IterDirectory(#[from] IterDirectoryError),
}

/// Represents errors that can occur during fragment collection.
#[derive(Debug, Error, Diagnostic)]
#[error("failed to collect from `{path}`")]
#[diagnostic(
    code(changelogging::builder::collect),
    help("make sure the directory is accessible")
)]
pub struct CollectError {
    /// The source of this error.
    #[source]
    #[diagnostic_source]
    pub source: CollectErrorSource,
    /// The path provided.
    pub path: PathBuf,
}

impl CollectError {
    /// Constructs [`Self`].
    pub fn new(source: CollectErrorSource, path: PathBuf) -> Self {
        Self { source, path }
    }

    /// Constructs [`Self`] from [`ReadDirectoryError`].
    pub fn read_directory(source: ReadDirectoryError, path: PathBuf) -> Self {
        Self::new(source.into(), path)
    }

    /// Constructs [`Self`] from [`IterDirectoryError`].
    pub fn iter_directory(source: IterDirectoryError, path: PathBuf) -> Self {
        Self::new(source.into(), path)
    }

    /// Constructs [`ReadDirectoryError`] and constructs [`Self`] from it.
    pub fn new_read_directory(source: std::io::Error, path: PathBuf) -> Self {
        Self::read_directory(ReadDirectoryError(source), path)
    }

    /// Constructs [`IterDirectoryError`] and constructs [`Self`] from it.
    pub fn new_iter_directory(source: std::io::Error, path: PathBuf) -> Self {
        Self::iter_directory(IterDirectoryError(source), path)
    }
}

/// Represents sources of errors that can occur when building.
#[derive(Debug, Error, Diagnostic)]
#[error(transparent)]
#[diagnostic(transparent)]
pub enum BuildErrorSource {
    /// Build title errors.
    BuildTitle(#[from] BuildTitleError),
    /// Build fragment errors.
    BuildFragment(#[from] BuildFragmentError),
    /// Collect errors.
    Collect(#[from] CollectError),
}

/// Represents errors that can occur when building.
#[derive(Debug, Error, Diagnostic)]
#[error("failed to build")]
#[diagnostic(
    code(changelogging::builder::build),
    help("see the report for more information")
)]
pub struct BuildError {
    /// The source of this error.
    #[source]
    #[diagnostic_source]
    pub source: BuildErrorSource,
}

impl BuildError {
    /// Constructs [`Self`].
    pub fn new(source: BuildErrorSource) -> Self {
        Self { source }
    }

    /// Constructs [`Self`] from [`BuildTitleError`].
    pub fn build_title(source: BuildTitleError) -> Self {
        Self::new(source.into())
    }

    /// Constructs [`Self`] from [`BuildFragmentError`].
    pub fn build_fragment(source: BuildFragmentError) -> Self {
        Self::new(source.into())
    }

    /// Constructs [`Self`] from [`CollectError`].
    pub fn collect(source: CollectError) -> Self {
        Self::new(source.into())
    }

    /// Constructs [`BuildTitleError`] and constructs [`Self`] from it.
    pub fn new_build_title(source: RenderError) -> Self {
        Self::build_title(BuildTitleError(source))
    }

    /// Constructs [`BuildFragmentError`] and constructs [`Self`] from it.
    pub fn new_build_fragment(source: RenderError) -> Self {
        Self::build_fragment(BuildFragmentError(source))
    }
}

/// Represents sources of errors that can occur when writing entries.
#[derive(Debug, Error, Diagnostic)]
#[error(transparent)]
#[diagnostic(transparent)]
pub enum WriteErrorSource {
    /// Open file errors.
    OpenFile(#[from] OpenFileError),
    /// Read file errors.
    ReadFile(#[from] ReadFileError),
    /// Build errors.
    Build(#[from] BuildError),
    /// Write file errors.
    WriteFile(#[from] WriteFileError),
}

/// Represents errors that can occur when writing entries.
#[derive(Debug, Error, Diagnostic)]
#[error("failed to write")]
#[diagnostic(
    code(changelogging::builder::write),
    help("see the report for more information")
)]
pub struct WriteError {
    /// The source of this error.
    #[source]
    #[diagnostic_source]
    pub source: WriteErrorSource,
}

impl WriteError {
    /// Constructs [`Self`].
    pub fn new(source: WriteErrorSource) -> Self {
        Self { source }
    }

    /// Constructs [`Self`] from [`OpenFileError`].
    pub fn open_file(source: OpenFileError) -> Self {
        Self::new(source.into())
    }

    /// Constructs [`Self`] from [`ReadFileError`].
    pub fn read_file(source: ReadFileError) -> Self {
        Self::new(source.into())
    }

    /// Constructs [`Self`] from [`BuildError`].
    pub fn build(source: BuildError) -> Self {
        Self::new(source.into())
    }

    /// Constructs [`Self`] from [`WriteFileError`].
    pub fn write_file(source: WriteFileError) -> Self {
        Self::new(source.into())
    }

    /// Constructs [`OpenFileError`] and constructs [`Self`] from it.
    pub fn new_open_file(source: std::io::Error, path: PathBuf) -> Self {
        Self::open_file(OpenFileError::new(source, path))
    }

    /// Constructs [`ReadFileError`] and constructs [`Self`] from it.
    pub fn new_read_file(source: std::io::Error, path: PathBuf) -> Self {
        Self::read_file(ReadFileError::new(source, path))
    }

    /// Constructs [`WriteFileError`] and constructs [`Self`] from it.
    pub fn new_write_file(source: std::io::Error, path: PathBuf) -> Self {
        Self::write_file(WriteFileError::new(source, path))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct RenderTitleData<'t> {
    #[serde(flatten)]
    context: &'t Context<'t>,
    date: Cow<'t, str>,
}

impl<'t> RenderTitleData<'t> {
    fn new(context: &'t Context<'_>, date: Date) -> Self {
        Self {
            context,
            date: Cow::Owned(date.to_string()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct RenderFragmentData<'f> {
    #[serde(flatten)]
    context: &'f Context<'f>,
    #[serde(flatten)]
    fragment: &'f Fragment<'f>,
}

impl<'f> RenderFragmentData<'f> {
    fn new(context: &'f Context<'_>, fragment: &'f Fragment<'_>) -> Self {
        Self { context, fragment }
    }
}

/// Represents changelog builders.
#[derive(Debug, Clone)]
pub struct Builder<'b> {
    /// The context of the project.
    pub context: Context<'b>,
    /// The configuration to use.
    pub config: Config<'b>,
    /// The date to use.
    pub date: Date,
    /// The renderer to use.
    pub renderer: Handlebars<'b>,
}

/// The `title` literal.
pub const TITLE: &str = "title";

/// The `fragment` literal.
pub const FRAGMENT: &str = "fragment";

impl<'b> Builder<'b> {
    /// Constructs [`Self`] from [`Workspace`].
    ///
    /// # Errors
    ///
    /// Returns [`InitError`] if initializing the renderer fails.
    pub fn from_workspace(workspace: Workspace<'b>, date: Date) -> Result<Self, InitError> {
        Self::new(workspace.context, workspace.config, date)
    }

    /// Constructs [`Self`].
    ///
    /// # Errors
    ///
    /// Returns [`InitError`] if initializing the renderer fails.
    pub fn new(context: Context<'b>, config: Config<'b>, date: Date) -> Result<Self, InitError> {
        let mut renderer = Handlebars::new();

        let formats = config.formats();

        renderer.set_strict_mode(true);

        renderer.register_escape_fn(no_escape);

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
const NEW_LINE: char = '\n';
const DOUBLE_NEW_LINE: &str = "\n\n";
const NO_SIGNIFICANT_CHANGES: &str = "No significant changes.";

fn heading(character: char, level: Level) -> String {
    repeat(character)
        .take(level.into())
        .chain(once(SPACE))
        .collect()
}

fn indent(character: char) -> String {
    once(character).chain(once(SPACE)).collect()
}

impl Builder<'_> {
    /// Returns [`Context`] reference.
    pub fn context(&self) -> &Context<'_> {
        &self.context
    }

    /// Returns [`Config`] reference.
    pub fn config(&self) -> &Config<'_> {
        &self.config
    }

    // BUILDING

    /// Builds entries and writes them to the changelog.
    ///
    /// # Errors
    ///
    /// Returns [`WriteError`] when building fails, as well as when I/O operations fail.
    pub fn write(&self) -> Result<(), WriteError> {
        let entry = self.build().map_err(WriteError::build)?;

        let path = self.config.paths.output.as_ref();

        let file = File::options()
            .read(true)
            .open(path)
            .map_err(|error| WriteError::new_open_file(error, path.to_owned()))?;

        let contents = read_to_string(file)
            .map_err(|error| WriteError::new_read_file(error, path.to_owned()))?;

        let mut file = File::options()
            .create(true)
            .write(true)
            .truncate(true)
            .open(path)
            .map_err(|error| WriteError::new_open_file(error, path.to_owned()))?;

        let start = self.config.start.as_ref();

        let mut string = String::new();

        if let Some((before, after)) = contents.split_once(start) {
            string.push_str(before);

            string.push_str(start);

            string.push_str(DOUBLE_NEW_LINE);

            string.push_str(&entry);

            string.push(NEW_LINE);

            let trimmed = after.trim_start();

            if !trimmed.is_empty() {
                string.push(NEW_LINE);

                string.push_str(trimmed);
            }
        } else {
            string.push_str(&entry);

            string.push(NEW_LINE);

            let trimmed = contents.trim_start();

            if !trimmed.is_empty() {
                string.push(NEW_LINE);

                string.push_str(trimmed);
            }
        };

        write!(file, "{string}")
            .map_err(|error| WriteError::new_write_file(error, path.to_owned()))?;

        Ok(())
    }

    /// Builds and previews (prints) entries.
    ///
    /// # Errors
    ///
    /// Returns [`BuildError`] when building fails.
    pub fn preview(&self) -> Result<(), BuildError> {
        let string = self.build()?;

        println!("{string}");

        Ok(())
    }

    /// Builds and returns entries.
    ///
    /// # Errors
    ///
    /// Returns [`BuildError`] when rendering titles and fragments or collecting fragments fails.
    pub fn build(&self) -> Result<String, BuildError> {
        let mut string = self
            .build_title()
            .map_err(|error| BuildError::new(error.into()))?;

        string.push_str(DOUBLE_NEW_LINE);

        let sections = self.collect().map_err(BuildError::collect)?;

        let built = self
            .build_sections(&sections)
            .map_err(|error| BuildError::new(error.into()))?;

        let contents = if built.is_empty() {
            NO_SIGNIFICANT_CHANGES
        } else {
            &built
        };

        string.push_str(contents);

        Ok(string)
    }

    /// Builds entry titles.
    ///
    /// # Errors
    ///
    /// Returns [`BuildTitleError`] when rendering fails.
    pub fn build_title(&self) -> Result<String, BuildTitleError> {
        let mut string = self.entry_heading();

        let title = self.render_title()?;

        string.push_str(&title);

        Ok(string)
    }

    /// Builds section titles.
    pub fn build_section_title_str(&self, title: &str) -> String {
        let mut string = self.section_heading();

        string.push_str(title);

        string
    }

    /// Similar to [`build_section_title_str`], except the input is [`AsRef<str>`].
    ///
    /// [`build_section_title_str`]: Self::build_section_title_str
    pub fn build_section_title<S: AsRef<str>>(&self, title: S) -> String {
        self.build_section_title_str(title.as_ref())
    }

    /// Builds fragments.
    ///
    /// # Errors
    ///
    /// Returns [`BuildFragmentError`] when rendering fails.
    pub fn build_fragment(&self, fragment: &Fragment<'_>) -> Result<String, BuildFragmentError> {
        let string = self.render_fragment(fragment)?;

        Ok(self.wrap(string))
    }

    /// Builds multiple fragments and joins them together.
    ///
    /// # Errors
    ///
    /// Returns [`BuildFragmentError`] when building any of the fragments fails.
    pub fn build_fragments(&self, fragments: &Fragments<'_>) -> Result<String, BuildFragmentError> {
        let string = fragments
            .iter()
            .map(|fragment| self.build_fragment(fragment))
            .process_results(|iterator| iterator.into_iter().join(DOUBLE_NEW_LINE))?;

        Ok(string)
    }

    /// Builds sections.
    ///
    /// This is essentially the same as calling [`build_section_title`] and [`build_fragments`],
    /// joining the results together.
    ///
    /// # Errors
    ///
    /// Returns [`BuildFragmentError`] when building any of the fragments fails.
    ///
    /// [`build_section_title`]: Self::build_section_title
    /// [`build_fragments`]: Self::build_fragments
    pub fn build_section_str(
        &self,
        title: &str,
        fragments: &Fragments<'_>,
    ) -> Result<String, BuildFragmentError> {
        let mut string = self.build_section_title(title);

        let built = self.build_fragments(fragments)?;

        string.push_str(DOUBLE_NEW_LINE);
        string.push_str(&built);

        Ok(string)
    }

    /// Similar to [`build_section_str`], except the input is [`AsRef<str>`].
    ///
    /// # Errors
    ///
    /// Returns [`BuildFragmentError`] when building any of the fragments fails.
    pub fn build_section<S: AsRef<str>>(
        &self,
        title: S,
        fragments: &Fragments<'_>,
    ) -> Result<String, BuildFragmentError> {
        self.build_section_str(title.as_ref(), fragments)
    }

    /// Builds multiple sections and joins them together.
    ///
    /// # Errors
    ///
    /// Returns [`BuildFragmentError`] when building any of the sections fails.
    pub fn build_sections(&self, sections: &Sections<'_>) -> Result<String, BuildFragmentError> {
        let types = self.config.types_with_defaults();

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

    /// Wraps the given string.
    pub fn wrap_str(&self, string: &str) -> String {
        let initial_indent = indent(self.config.indents.bullet);
        let subsequent_indent = indent(SPACE);

        let options = WrapOptions::new(self.config.wrap.get())
            .break_words(false)
            .word_separator(WordSeparator::AsciiSpace)
            .word_splitter(WordSplitter::NoHyphenation)
            .initial_indent(&initial_indent)
            .subsequent_indent(&subsequent_indent);

        fill(string, options)
    }

    /// Similar to [`wrap_str`], except the input is [`AsRef<str>`].
    ///
    /// [`wrap_str`]: Self::wrap_str
    pub fn wrap<S: AsRef<str>>(&self, string: S) -> String {
        self.wrap_str(string.as_ref())
    }

    // RENDERING

    /// Renders entry titles.
    ///
    /// # Errors
    ///
    /// Returns [`RenderError`] if rendering the title fails.
    pub fn render_title(&self) -> Result<String, RenderError> {
        let data = RenderTitleData::new(self.context(), self.date);

        self.renderer.render(TITLE, &data)
    }

    /// Renders fragments.
    ///
    /// # Errors
    ///
    /// Returns [`RenderError`] if rendering the given fragment fails.
    pub fn render_fragment(&self, fragment: &Fragment<'_>) -> Result<String, RenderError> {
        if fragment.partial.id.is_integer() {
            let data = RenderFragmentData::new(self.context(), fragment);

            self.renderer.render(FRAGMENT, &data)
        } else {
            Ok(fragment.content.as_ref().to_owned())
        }
    }

    // COLLECTING

    /// Collects fragments into sections.
    ///
    /// # Errors
    ///
    /// Returns [`CollectError`] when reading or iterating the fragments directory fails.
    pub fn collect(&self) -> Result<Sections<'_>, CollectError> {
        let directory = self.config.paths.directory.as_ref();

        let mut sections = Sections::new();

        read_dir(directory)
            .map_err(|error| CollectError::new_read_directory(error, directory.to_owned()))?
            .map(|result| {
                result
                    .map(|entry| entry.path())
                    .map_err(|error| CollectError::new_iter_directory(error, directory.to_owned()))
            })
            .process_results(|iterator| {
                iterator
                    .into_iter()
                    .filter_map(|path| load::<Fragment<'_>, _>(path).ok()) // ignore errors
                    .for_each(|fragment| {
                        sections
                            .entry(fragment.partial.type_name.clone())
                            .or_default()
                            .push(fragment);
                    });
            })?;

        sections.values_mut().for_each(|section| section.sort());

        Ok(sections)
    }

    /// Collects paths to fragments.
    ///
    /// # Errors
    ///
    /// Returns [`CollectError`] if reading or iterating the fragments directory fails.
    pub fn collect_paths(&self) -> Result<Vec<PathBuf>, CollectError> {
        let directory = self.config.paths.directory.as_ref();

        read_dir(directory)
            .map_err(|error| CollectError::new_read_directory(error, directory.to_owned()))?
            .map(|result| {
                result
                    .map(|entry| entry.path())
                    .map_err(|error| CollectError::new_iter_directory(error, directory.to_owned()))
            })
            .process_results(|iterator| {
                iterator
                    .into_iter()
                    .filter(|path| is_valid_path(path))
                    .collect()
            })
    }

    // HEADING

    /// Constructs headings for the given level.
    pub fn level_heading(&self, level: Level) -> String {
        heading(self.config.indents.heading, level)
    }

    /// Constructs entry headings.
    pub fn entry_heading(&self) -> String {
        self.level_heading(self.config.levels.entry)
    }

    /// Constructs section headings.
    pub fn section_heading(&self) -> String {
        self.level_heading(self.config.levels.section)
    }
}

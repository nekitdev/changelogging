use std::{
    fs::{read_dir, DirEntry, ReadDir},
    io::Error,
    marker::PhantomData,
    path::Path,
};

/// Load values from paths.
pub trait FromPath: Sized {
    /// The associated error type which can be returned when loading.
    ///
    /// Must implement [`From`] for [`Error`], which is useful
    /// when combined with I/O operations (for instance in [`load_if_exists`]).
    type Error: From<Error>;

    /// Loads the `path` provided to return the corresponding value.
    fn from_path<P: AsRef<Path>>(path: P) -> Result<Self, Self::Error>;
}

/// Loads the `path` provided into the corresponding value.
///
/// [`load`] can load any type that implements [`FromPath`].
///
/// See [`load_if_exists`] for the existence-checking version of this function.
///
/// # Errors
///
/// Will return [`Error`] if loading fails.
///
/// [`Error`]: FromPath::Error
pub fn load<F: FromPath, P: AsRef<Path>>(path: P) -> Result<F, F::Error> {
    F::from_path(path)
}

/// Loads the `path`, if it exists, into the corresponding value.
///
/// If [`try_exists`] (after applying the `?` operator) returns:
///
/// - [`true`], then [`load`] is attempted and the value is wrapped into [`Some`];
/// - [`false`], then [`None`] is returned right away.
///
/// # Errors
///
/// Will return [`Error`] if either loading or [`try_exists`] fails.
///
/// [`try_exists`]: Path::try_exists
/// [`Error`]: FromPath::Error
pub fn load_if_exists<F: FromPath, P: AsRef<Path>>(path: P) -> Result<Option<F>, F::Error> {
    let option = if path.as_ref().try_exists()? {
        let value = F::from_path(path)?;

        Some(value)
    } else {
        None
    };

    Ok(option)
}

/// Returns the final component of the `path`, mapping it to [`str`].
///
/// See [`file_name`] for more information.
///
/// [`file_name`]: Path::file_name
pub fn name_str(path: &Path) -> Option<&str> {
    path.file_name().and_then(|os_string| os_string.to_str())
}

/// Represents iterators that traverse directories, loading values via [`FromPath`].
pub struct FromDir<F: FromPath> {
    data: PhantomData<F>,
    iterator: ReadDir,
}

impl<F: FromPath> FromDir<F> {
    /// Creates [`FromDir`] instances from [`ReadDir`] iterators.
    pub fn new(iterator: ReadDir) -> Self {
        Self {
            data: PhantomData,
            iterator,
        }
    }
}

impl<F: FromPath> FromPath for FromDir<F> {
    type Error = std::io::Error;

    fn from_path<P: AsRef<Path>>(path: P) -> Result<Self, Self::Error> {
        let iterator = read_dir(path)?;

        Ok(Self::new(iterator))
    }
}

impl<F: FromPath> FromDir<F> {
    pub fn process(result: Result<DirEntry, Error>) -> Result<F, F::Error> {
        let path = result?.path();

        load(path)
    }
}

impl<F: FromPath> Iterator for FromDir<F> {
    type Item = Result<F, F::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self
            .iterator
            .next()?
            .map(|entry| entry.path())
            .map_err(|error| error.into())
            .and_then(|path| load(path));

        Some(result)
    }
}

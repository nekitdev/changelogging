use std::{
    fs::{ReadDir, read_dir},
    io::Error as IoError,
    path::Path,
};

use bon::Builder;
use from_path::FromPath;
use thiserror::Error;

use crate::fragments::{
    fragment::LoadError,
    items::{Item, OwnedItem},
};

#[derive(Debug, Error)]
#[error("failed to read directory: {error}")]
pub struct ReadError {
    #[from]
    error: IoError,
}

impl ReadError {
    pub const fn new(error: IoError) -> Self {
        Self { error }
    }

    pub fn get(self) -> IoError {
        self.error
    }
}

#[derive(Debug, Error)]
#[error("failed to iterate directory: {error}")]
pub struct IterError {
    #[from]
    error: IoError,
}

impl IterError {
    pub const fn new(error: IoError) -> Self {
        Self { error }
    }

    pub fn get(self) -> IoError {
        self.error
    }
}

#[derive(Debug, Error)]
#[error(transparent)]
pub enum Error {
    Iter(#[from] IterError),
    Load(#[from] LoadError),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Mode {
    Lenient,
    Strict,
}

impl Default for Mode {
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl Mode {
    pub const fn is_lenient(self) -> bool {
        matches!(self, Self::Lenient)
    }

    pub const fn is_strict(self) -> bool {
        matches!(self, Self::Strict)
    }

    pub const fn is_default(self) -> bool {
        matches!(self, Self::DEFAULT)
    }

    pub const DEFAULT: Self = Self::Lenient;
}

pub use Mode::{Lenient, Strict};

#[derive(Debug, Builder)]
pub struct Collector {
    reader: ReadDir,
    #[builder(default)]
    mode: Mode,
}

impl Collector {
    pub const fn new(reader: ReadDir, mode: Mode) -> Self {
        Self { reader, mode }
    }

    pub const fn lenient(reader: ReadDir) -> Self {
        Self::new(reader, Lenient)
    }

    pub const fn strict(reader: ReadDir) -> Self {
        Self::new(reader, Strict)
    }

    pub const fn only(reader: ReadDir) -> Self {
        Self::new(reader, Mode::DEFAULT)
    }

    pub const fn mode(&self) -> Mode {
        self.mode
    }

    pub fn next_output(&mut self) -> Option<Option<Result<OwnedItem, Error>>> {
        match self.reader.next() {
            None => Some(None),
            Some(Ok(entry)) => match Item::try_entry(entry) {
                Ok(item) => Some(Some(Ok(item))),
                Err(error) => {
                    if self.mode().is_lenient() {
                        None
                    } else {
                        Some(Some(Err(error.into())))
                    }
                }
            },
            Some(Err(error)) => Some(Some(Err(IterError::new(error).into()))),
        }
    }
}

impl FromPath for Collector {
    type Error = ReadError;

    fn from_path<P: AsRef<Path>>(path: P) -> Result<Self, Self::Error> {
        let reader = read_dir(path)?;

        let collector = Self::only(reader);

        Ok(collector)
    }
}

impl Iterator for Collector {
    type Item = Result<OwnedItem, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(option) = self.next_output() {
                return option;
            }
        }
    }
}

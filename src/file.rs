use std::{fs, io::Error as IoError, path::Path};

use edit::edit_file;
use non_empty_str::{EmptyString, NonEmptyStr, NonEmptyString};
use thiserror::Error;

use crate::trim_in_place::TrimInPlace;

#[derive(Debug, Error)]
#[error("failed to read file: {error}")]
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

pub fn read<P: AsRef<Path>>(path: P) -> Result<String, ReadError> {
    let content = fs::read_to_string(path)?;

    Ok(content)
}

#[derive(Debug, Error)]
#[error("expected non-empty contents after trimming: {error}")]
pub struct EmptyError {
    error: EmptyString,
}

impl EmptyError {
    pub const fn new(error: EmptyString) -> Self {
        Self { error }
    }

    pub fn get(self) -> EmptyString {
        self.error
    }
}

#[derive(Debug, Error)]
#[error(transparent)]
pub enum ReadNonEmptyError {
    Read(#[from] ReadError),
    Empty(#[from] EmptyString),
}

pub fn read_non_empty<P: AsRef<Path>>(path: P) -> Result<NonEmptyString, ReadNonEmptyError> {
    let mut content = read(path)?;

    content.trim_in_place();

    let non_empty = content.try_into()?;

    Ok(non_empty)
}

#[derive(Debug, Error)]
#[error("failed to edit file: {error}")]
pub struct EditError {
    #[from]
    error: IoError,
}

impl EditError {
    pub const fn new(error: IoError) -> Self {
        Self { error }
    }

    pub fn get(self) -> IoError {
        self.error
    }
}

pub fn edit<P: AsRef<Path>>(path: P) -> Result<(), EditError> {
    edit_file(path)?;

    Ok(())
}

#[derive(Debug, Error)]
#[error("failed to write to file")]
pub struct WriteError(#[from] std::io::Error);

pub fn write<P: AsRef<Path>, S: AsRef<str>>(path: P, string: S) -> Result<(), WriteError> {
    fs::write(path, string.as_ref())?;

    Ok(())
}

pub fn write_non_empty<P: AsRef<Path>, S: AsRef<NonEmptyStr>>(
    path: P,
    string: S,
) -> Result<(), WriteError> {
    write(path, string.as_ref())?;

    Ok(())
}

//! Application initialization.

use std::{env::set_current_dir, path::Path};

/// Represents initialization errors.
pub type Error = std::io::Error;

/// Initializes `changelogging`, optionally changing current directory.
pub fn init<D: AsRef<Path>>(directory: Option<D>) -> Result<(), Error> {
    if let Some(path) = directory {
        set_current_dir(path)?;
    };

    Ok(())
}

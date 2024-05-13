use anyhow::Result;
use changelogging::cli::{Command, Runnable};
use clap::Parser;

fn main() -> Result<()> {
    Command::parse().run()?;

    Ok(())
}

//! The `changelogging` binary.

use changelogging::app::App;
use clap::Parser;
use miette::Result;

fn main() -> Result<()> {
    App::parse().run()?;

    Ok(())
}

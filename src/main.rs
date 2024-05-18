use anyhow::Result;
use changelogging::app::App;
use clap::Parser;

fn main() -> Result<()> {
    App::parse().run()?;

    Ok(())
}

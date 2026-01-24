mod cli;
mod commands;
mod proxy;

use anyhow::Result;
use clap::Parser;
use cli::Cli;

fn main() -> Result<()> {
    let cli = Cli::parse();
    cli.execute()?;
    Ok(())
}

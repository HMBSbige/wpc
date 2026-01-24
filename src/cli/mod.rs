use crate::commands::{direct, global, pac, query};
use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    Query,
    Direct,
    Pac {
        url: String,
    },
    Global {
        url: String,
        #[arg(short, long)]
        bypass: Option<String>,
    },
}

impl Cli {
    pub fn execute(&self) -> Result<()> {
        match &self.command {
            Command::Query => query::execute(),
            Command::Direct => direct::execute(),
            Command::Pac { url } => pac::execute(url),
            Command::Global { url, bypass } => global::execute(url, bypass.as_deref()),
        }
    }
}

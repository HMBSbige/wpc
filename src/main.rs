mod proxy;

use anyhow::Result;
use clap::{Parser, Subcommand};
use proxy::Server;

#[derive(Parser)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Query,
    Direct,
    Pac {
        url: String,
    },
    Global {
        url: Server,
        #[arg(short, long)]
        bypass: Option<String>,
    },
}

fn main() -> Result<()> {
    match Cli::parse().command {
        Command::Query => return proxy::query(),
        Command::Direct => proxy::set_direct(),
        Command::Pac { url } => proxy::set_pac(url),
        Command::Global { url, bypass } => proxy::set_global(url, bypass.unwrap_or_default()),
    }?;

    println!("success");
    Ok(())
}

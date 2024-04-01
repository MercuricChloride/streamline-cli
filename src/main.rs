use anyhow::Error;
use clap::{Parser, Subcommand};

mod commands;
pub mod utils;

use commands::*;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Install,
    List,
    Add { path: String },
    Build { path: String },
}

fn main() -> Result<(), Error> {
    let Cli { command } = Cli::parse();

    match command {
        Commands::Install => install::handler()?,
        Commands::Add { path } => add::handler(&path)?,
        Commands::List => list::handler()?,
        Commands::Build { path } => build::handler(&path)?,
    };

    Ok(())
}

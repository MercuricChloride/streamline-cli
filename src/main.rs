use std::{fs, path::Path};

use anyhow::Error;
use clap::{Parser, Subcommand};

mod commands;
pub mod constants;
pub mod utils;

use commands::*;
use utils::get_streamline_dir;

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
    Clean,
    Add {
        path: String,
    },
    Build {
        path: String,
        start_block: Option<i64>,
    },
}

fn main() -> Result<(), Error> {
    let Cli { command } = Cli::parse();

    match command {
        Commands::Install => install::handler()?,
        Commands::Add { path } => add::handler(&path)?,
        Commands::List => list::handler()?,
        Commands::Build { path, start_block } => build::handler(&path, start_block)?,
        Commands::Clean => {
            let path = format!("{}/template-repo/Cargo.lock", get_streamline_dir());
            let lock_file = Path::new(&path);
            if lock_file.exists() {
                fs::remove_file(lock_file).unwrap();
                println!("Removed template repo lock file!");
            }
        }
    };

    Ok(())
}

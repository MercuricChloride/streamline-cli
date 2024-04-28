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
    List {
        prefix: Option<String>,
    },
    Clean,
    Add {
        path: String,
        name: String,
    },
    Schema {
        path: String,
    },
    Build {
        path: String,
        #[arg(short, long)]
        start_block: Option<i64>,
        #[arg(short, long)]
        network: Option<String>,
        #[arg(short, long, default_value = "false")]
        only_format: bool,
    },
}

fn main() -> Result<(), Error> {
    let Cli { command } = Cli::parse();

    match command {
        Commands::List { prefix } => list::handler(prefix)?,
        Commands::Add { path, name } => add::handler(&path, &name)?,
        Commands::Build {
            path,
            start_block,
            network,
            only_format,
        } => build::handler(&path, start_block, network, only_format)?,
        Commands::Schema { path } => schema::handler(&path)?,
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

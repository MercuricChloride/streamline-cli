use std::{fs, path::Path};

use crate::utils::*;
use anyhow::{anyhow, Error};

fn format_rhai_file(path: &str) -> Result<(), Error> {
    let scripts_dir = get_scripts_dir();
    let command = format!("bash {scripts_dir}/format.sh {path}");
    run_command(&command)?;
    println!("Formatted the rhai file");
    Ok(())
}

pub fn handler(path: &str) -> Result<(), Error> {
    // First format the rhai file
    format_rhai_file(path)?;
    Ok(())
}

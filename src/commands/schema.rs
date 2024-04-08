use anyhow::Error;

use crate::utils::{get_scripts_dir, run_command};

fn build_schema_file(path: &str) -> Result<(), Error> {
    let scripts_dir = get_scripts_dir();
    let command = format!("{scripts_dir}/schema.sh {path}");
    run_command(&command)?;
    println!("Built schema.graphql!");
    Ok(())
}

pub fn handler(path: &str) -> Result<(), Error> {
    build_schema_file(path)?;

    Ok(())
}

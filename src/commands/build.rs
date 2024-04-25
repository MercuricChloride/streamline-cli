use std::{fs, path::Path};

use crate::utils::*;
use anyhow::{anyhow, Error};
use rhai::{packages::streamline, Engine, Scope};

fn format_rhai_file(path: &str, start_block: Option<i64>) -> Result<(), Error> {
    let start_block = start_block.map(|e| e.to_string()).unwrap_or(String::new());
    let scripts_dir = get_scripts_dir();
    let command = format!("{scripts_dir}/format.sh {path} {start_block}");
    run_command(&command)?;
    println!("Formatted the rhai file");
    Ok(())
}

fn build_spkg() -> Result<(), Error> {
    println!("Starting compilation...");
    let scripts_dir = get_scripts_dir();
    let command = format!("{scripts_dir}/build.sh");
    run_command(&command)?;
    let current_dir = std::env::current_dir()?;
    let spkg_path = format!("{}/output.spkg", current_dir.to_string_lossy());
    let spkg_path = Path::new(&spkg_path);

    if !spkg_path.exists() {
        return Err(anyhow!("Error building the spkg"));
    }

    println!("Built the spkg");
    Ok(())
}

fn rhai_run() -> Result<(), Error> {
    let formatted_file = format!("{}/template-repo/streamline.rhai", get_streamline_dir());
    let formatted_file = Path::new(&formatted_file);
    let filename = formatted_file.file_name().unwrap();

    let engine = Engine::new_raw();
    let scope = Scope::new();
    let (mut engine, _scope) = streamline::init_package(engine, scope);
    engine.set_optimization_level(rhai::OptimizationLevel::Simple);

    if let Err(err) = engine
        .compile(fs::read_to_string(formatted_file).unwrap())
        .map_err(|err| err.into())
        .and_then(|mut ast| {
            ast.set_source(filename.to_string_lossy().to_string());
            engine.run_ast(&ast)
        })
    {
        let filename = filename.to_string_lossy();

        eprintln!("{:=<1$}", "", filename.len());
        eprintln!("{filename}");
        eprintln!("{:=<1$}", "", filename.len());
        eprintln!();

        return Err(anyhow!("Error running file: {:?}", err));
    }

    Ok(())
}

fn update_yaml_network(network: String) -> Result<(), Error> {
    println!("Updating network in yaml...");
    let scripts_dir = get_scripts_dir();
    let command = format!("{scripts_dir}/update_network.sh {network}");
    run_command(&command)?;
    println!("Network Updated");
    Ok(())
}

pub fn handler(
    path: &str,
    start_block: Option<i64>,
    network: Option<String>,
    only_format: bool,
) -> Result<(), Error> {
    // First format the rhai file
    format_rhai_file(path, start_block)?;

    // If only format is passed, cat out the file and return
    if only_format {
        let formatted_file = format!("{}/template-repo/streamline.rhai", get_streamline_dir());
        let formatted_file = Path::new(&formatted_file);
        let file = fs::read_to_string(formatted_file)?;
        println!("{}", file);
        return Ok(());
    }

    // Run the file to generate the rust code
    rhai_run()?;

    // Update the network field in the yaml if passed as an arg
    if let Some(network) = network {
        update_yaml_network(network)?;
    }

    // build the spkg
    build_spkg()?;
    Ok(())
}

use std::env;

use anyhow::Error;
use std::process::{Command, Stdio};

pub fn get_streamline_dir() -> String {
    let home_path =
        env::var("HOME").expect("Couldn't find the $HOME variable in your shell, is it set?");
    format!("{home_path}/streamline-cli")
}

pub fn get_abis_dir() -> String {
    format!("{}/abis", get_streamline_dir())
}

pub fn get_scripts_dir() -> String {
    format!("{}/scripts", get_streamline_dir())
}

pub fn run_command(command: &str) -> Result<String, Error> {
    let command_parts = command.split(" ").collect::<Vec<_>>();
    let command = command_parts[0];
    let args: &[&str] = if command_parts.len() > 1 {
        &command_parts[1..]
    } else {
        &[]
    };

    let result = Command::new(command)
        .args(args)
        .stdout(Stdio::piped())
        .output();

    match result {
        Ok(output) => {
            let std_err = String::from_utf8(output.stderr).unwrap_or(String::new());
            if !output.status.success() {
                eprintln!("Error: {:?}", std_err);
                return Err(Error::msg(std_err));
            }
            let output_str = String::from_utf8(output.stdout).unwrap_or(String::new());
            Ok(output_str)
        }
        Err(err) => {
            eprintln!("Error running command: {:?}", err);
            Err(Error::from(err))
        }
    }
}

use std::{fs, path::Path};

use crate::utils::*;
use anyhow::{anyhow, Error};

pub fn handler() -> Result<(), Error> {
    let abis_dir = get_abis_dir();
    let abis_dir = Path::new(&abis_dir)
        .read_dir()
        .expect("Couldn't read the abis dir, does it exist?");

    println!("ABIS AVAILABLE IN STREAMLINE:\n----\n");

    for entry in abis_dir {
        if let Ok(entry) = entry {
            let file_name = entry.file_name();
            let file_name = file_name.to_str().unwrap();

            println!("- {}", file_name.trim_end_matches(".json"));
        }
    }

    Ok(())
}

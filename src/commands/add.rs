use std::{fs, path::Path};

use crate::utils::*;
use anyhow::{anyhow, Error};

pub fn handler(abi_path: &str) -> Result<(), Error> {
    let abis_dir = get_abis_dir();
    let abi = Path::new(abi_path);
    let file_name = abi.file_name().and_then(|e| e.to_str()).unwrap();

    let streamline_abi_path = format!("{}/{}", abis_dir, file_name);
    let streamline_abi_path = Path::new(&streamline_abi_path);

    // check if the name already exists in the abi dir
    if streamline_abi_path.exists() {
        return Err(anyhow!(
            "Abi already exists in the streamline abis directory!"
        ));
    }

    fs::copy(abi, streamline_abi_path)?;

    println!("Added {} to the streamline abis!", file_name);

    Ok(())
}

use std::{fs, path::Path};

use crate::utils::*;
use anyhow::{anyhow, Error};

enum LinkType<'a> {
    Http(&'a str),
    File(&'a str),
}

impl LinkType<'_> {
    fn new(link: &str) -> Result<LinkType, Error> {
        if link.starts_with("http") {
            Ok(LinkType::Http(link))
        } else {
            Ok(LinkType::File(link))
        }
    }

    fn resolve(&self) -> Result<String, Error> {
        match self {
            LinkType::Http(link) => {
                let url = reqwest::Url::parse(link)?;
                let response = reqwest::blocking::get(url)?;
                let response = response.text()?;
                Ok(response)
            }
            LinkType::File(link) => {
                let file = fs::read_to_string(link)?;
                Ok(file)
            }
        }
    }
}

pub fn handler(abi_path: &str, file_name: &str) -> Result<(), Error> {
    let abis_dir = get_abis_dir();
    let abi = LinkType::new(abi_path)?.resolve()?;

    let streamline_abi_path = format!("{}/{}", abis_dir, file_name);
    let streamline_abi_path = Path::new(&streamline_abi_path);

    // check if the name already exists in the abi dir
    if streamline_abi_path.exists() {
        return Err(anyhow!(
            "Abi already exists in the streamline abis directory!"
        ));
    }

    fs::write(streamline_abi_path, abi)?;

    println!("Added {} to the streamline abis!", file_name);

    Ok(())
}

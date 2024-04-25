use crate::{constants, utils::*};
use anyhow::Error;
use std::{
    env, fs,
    path::Path,
    process::{Command, Stdio},
};

const TEMPLATE_REPO_LINK: &str =
    "https://github.com/MercuricChloride/streamline-template-repository.git";

macro_rules! checks {
    ($($name:ident -> $command:expr)+) => {
        $(
            let msg = format!("{} is not installed!", stringify!($name));
            if let Err(e) = run_command($command) {
                eprintln!("{}", msg);
                return Err(e.into());
            } else {
                println!("{} is installed!", stringify!($name));
            }
        )+
        Ok(())
    };
}

fn check_dependencies() -> Result<(), Error> {
    checks! {
        Gawk -> "gawk --version"
        Git -> "git --version"
        Substreams -> "substreams --version"
        //FireEth -> "fireeth --version"
    }
}

fn install_template_repo(install_location: &str) -> Result<(), Error> {
    let path = Path::new(install_location);
    if path.exists() && path.is_dir() {
        fs::remove_dir_all(install_location)?;
    }

    run_command(&format!(
        "git clone {} {}",
        TEMPLATE_REPO_LINK, install_location
    ))?;
    Ok(())
}

fn create_dir(path: &str) -> Result<(), Error> {
    let path = Path::new(path);
    if !path.exists() {
        fs::create_dir(path)?
    }
    Ok(())
}

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> Result<(), Error> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

fn boostrap_dirs() -> Result<(), Error> {
    let home_path =
        env::var("HOME").expect("Couldn't find the $HOME variable in your shell, is it set?");
    let install_location = format!("{home_path}/streamline-cli");

    let template_repo_path = format!("{install_location}/template-repo");
    let abi_path = format!("{install_location}/abis");
    let scripts_path = format!("{install_location}/scripts");

    create_dir(&install_location).expect("Couldn't create the streamline-cli dir!");
    println!("Created streamline-cli directory");

    create_dir(&abi_path).expect("Couldn't create the abis dir!");
    println!("Created the abis dir");

    create_dir(&scripts_path).expect("Couldn't create the scripts dir!");
    copy_dir_all(format!("{}/scripts/", constants::ROOT_PATH), &scripts_path)?;
    println!("Created scripts directory");

    // copy the template repo
    install_template_repo(&template_repo_path).expect("Couldn't install the template repo");
    println!("Cloned the template repo!");

    Ok(())
}

pub fn handler() -> Result<(), Error> {
    println!("--DEPENDENCIES--");
    // check for awk
    check_dependencies()?;
    println!("n--DONE--\n");

    println!("--SETUP DIRECTORIES--");
    // boostrap the directories
    boostrap_dirs()?;
    println!("--DONE--\n");

    Ok(())
}

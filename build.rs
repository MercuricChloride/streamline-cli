use std::{env, fs};

fn main() {
    let root_path = env::var("CARGO_MANIFEST_DIR").unwrap();
    let root_path_constant = format!("pub const ROOT_PATH: &str = \"{}\";", &root_path);
    fs::write(format!("{root_path}/src/constants.rs"), root_path_constant).unwrap();
}

use std::{fmt::Display, fs, path::Path};

use crate::utils::*;
use anyhow::{anyhow, Error};
use serde_json::Value;

struct Input {
    name: String,
    kind: String,
}

impl Input {
    fn from_value(value: &Value) -> Option<Self> {
        let name = value.get("name")?.as_str()?.to_string();
        let kind = value.get("type")?.as_str()?.to_string();
        Some(Input { name, kind })
    }

    pub fn pretty_print(&self) {
        println!("    {}: {}", self.name, self.kind);
    }
}

struct Event {
    name: String,
    fields: Vec<Input>,
}

impl Event {
    fn from_value(value: &Value) -> Option<Self> {
        let kind = value.get("type")?;
        if kind == "event" {
            let name = value.get("name")?.as_str()?.to_string();
            let fields = value
                .get("inputs")?
                .as_array()?
                .iter()
                .filter_map(Input::from_value)
                .collect::<Vec<_>>();

            return Some(Event { name, fields });
        }

        None
    }

    pub fn print_with_module_prefix(&self, module_prefix: &str) {
        println!("  fn {module_prefix}::{}(BLOCK, ADDRESS[])", self.name);
        for field in self.fields.iter() {
            field.pretty_print()
        }
        println!("\n");
    }
}

fn events(abi: Value) -> Option<Vec<Event>> {
    Some(
        abi.as_array()?
            .iter()
            .filter_map(|e| {
                if e.get("type")?.as_str()? == "event" {
                    Event::from_value(e)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>(),
    )
}

pub fn handler(prefix: Option<String>) -> Result<(), Error> {
    let prefix = prefix.unwrap_or_default();
    let abis_dir = get_abis_dir();
    let abis_dir = Path::new(&abis_dir)
        .read_dir()
        .expect("Couldn't read the abis dir, does it exist?");

    println!("ABIS AVAILABLE IN STREAMLINE:\n----\n");

    for entry in abis_dir {
        if let Ok(entry) = entry {
            let contents = fs::read_to_string(entry.path())?;
            let file_name = entry.file_name();
            let file_name = file_name.to_str().unwrap();

            if !file_name.starts_with(&prefix) {
                continue;
            }

            let events = events(serde_json::from_str(&contents)?).unwrap();
            let module_name = file_name.trim_end_matches(".json");
            println!("\nModule: {}", module_name);
            for event in events {
                event.print_with_module_prefix(module_name);
            }
        }
    }

    Ok(())
}

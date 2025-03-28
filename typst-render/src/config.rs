use std::io::prelude::*;
use std::str::FromStr;
use std::{fs::File, path::PathBuf};

use serde::{Deserialize, Deserializer};

use crate::args::{Input, ProcessArgs, WorldArgs};

pub fn deserialize_path<'de, D>(deserializer: D) -> Result<Input, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;

    match s.as_str() {
        "stdin" => Ok(Input::Stdin),
        path => Ok(Input::Path(PathBuf::from_str(path).unwrap())),
    }
}

#[derive(Deserialize)]
#[serde(default)]
pub struct SimulationConfig {
    #[serde(deserialize_with = "deserialize_path")]
    pub input: Input,

    pub process_config: ProcessArgs,

    pub world_config: WorldArgs,
}

impl Default for SimulationConfig {
    fn default() -> Self {
        Self {
            input: Input::Stdin,
            process_config: ProcessArgs::default(),
            world_config: WorldArgs::default(),
        }
    }
}

pub fn load_config(path: &str) -> SimulationConfig {
    let config_str = open_config_file(path).expect("Couldn't open file at the given path");

    toml::from_str(&config_str).unwrap()
}

fn open_config_file(path: &str) -> std::io::Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

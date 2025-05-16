use serde::Deserialize;
use std::fs;
use toml;

#[derive(Deserialize)]
pub struct Config {
    pub ip: String,
    pub ports: Vec<u16>,
}

impl Config {
    pub fn from_file(file: &str) -> Self {
        let content = fs::read_to_string(file).expect("Failed to read config file");
        toml::from_str(&content).expect("Failed to parse config file")
    }
}

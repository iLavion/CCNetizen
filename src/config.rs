// Filename: config.rs
// Configuration file for the bot

use serde::Deserialize;
use std::fs;
use std::error::Error;
use toml;

#[derive(Deserialize)]
pub struct Config {
    pub discord_token: Option<String>,
    pub application_id: u64,
}

impl Config {
    pub fn from_file(path: &str) -> Result<Self, Box<dyn Error>> {
        let config_str = fs::read_to_string(path)?;
        let mut config: Config = toml::from_str(&config_str)?;
        config.discord_token = Some(config.discord_token.expect("Missing discord_token in config file"));
        Ok(config)
    }
}
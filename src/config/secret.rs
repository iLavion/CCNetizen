// Filename: config.rs
// Configuration file for the bot

use serde::Deserialize;
use std::fs;
use std::error::Error;
use toml;

#[derive(Deserialize)]
pub struct Config {
    // Discord 
    pub discord_token: String,
    // AWS Production
    pub aws_endpoint: String,
    pub aws_region: String,
    pub aws_access: String,
    pub aws_secret: String,
    // AWS Development
    pub aws_endpoint_dev: String,
    pub aws_region_dev: String,
    pub aws_access_dev: String,
    pub aws_secret_dev: String,
}

impl Config {
    pub fn from_file(path: &str) -> Result<Self, Box<dyn Error>> {
        let config_str = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&config_str)?;
        if config.discord_token.is_empty() {
            return Err("Missing discord_token in config file".into());
        }
        Ok(config)
    }
}
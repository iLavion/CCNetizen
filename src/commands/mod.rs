// Filename: commands/mod.rs
// Entry point for commands modules

pub mod ping;

use crate::{Data, Error};

pub fn get_commands() -> Vec<poise::Command<Data, Error>> {
    vec![
        // Add commands here...
        ping::ping(),
    ]
}
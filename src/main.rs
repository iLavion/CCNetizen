// Filename: main.rs
// Main entry point for the bot

#![warn(clippy::str_to_string)]

mod config;
mod commands;
mod services;
mod models;
mod repositories;

use poise::serenity_prelude as serenity;
use config::secret::Config;
use config::db_client::create_dynamodb_client;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::Duration,
};

// Types used by all command functions
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

// Custom user data passed to all command functions
pub struct Data {
    votes: Mutex<HashMap<String, u32>>,
    db_client: aws_sdk_dynamodb::Client,
}

async fn on_error(error: poise::FrameworkError<'_, Data, Error>) {
    match error {
        poise::FrameworkError::Setup { error, .. } => {
            panic!("Failed to start bot: {:?}", error)
        }
        poise::FrameworkError::Command { error, ctx, .. } => {
            println!("Error in command `{}`: {:?}", ctx.command().name, error);
        }
        poise::FrameworkError::CommandCheckFailed { error, ctx, .. } => {
            println!("Command check failed for command `{}`: {:?}", ctx.command().name, error);
        }
        error => {
            if let Err(e) = poise::builtins::on_error(error).await {
                println!("Error while handling error: {}", e)
            }
        }
    }
}

#[tokio::main]
async fn main() {
    // Set the environment variable for development mode
    std::env::set_var("APP_ENV", "development");

    tracing_subscriber::fmt::init();
    
    // Load the configuration file
    let config = Config::from_file("config.toml").expect("Failed to load config");

    // Determine if we're in development mode
    let is_development = std::env::var("APP_ENV").unwrap_or_default() == "development";

    let options = poise::FrameworkOptions {
        commands: commands::get_commands(),
        prefix_options: poise::PrefixFrameworkOptions {
            prefix: Some("~".into()),
            edit_tracker: Some(Arc::new(poise::EditTracker::for_timespan(
                Duration::from_secs(3600),
            ))),
            additional_prefixes: vec![
                poise::Prefix::Literal("hey bot,"),
                poise::Prefix::Literal("hey bot"),
            ],
            ..Default::default()
        },
        on_error: |error| Box::pin(on_error(error)),
        pre_command: |ctx| {
            Box::pin(async move {
                println!("Executing command {}...", ctx.command().qualified_name);
            })
        },
        post_command: |ctx| {
            Box::pin(async move {
                println!("Executed command {}!", ctx.command().qualified_name);
            })
        },
        command_check: Some(|ctx| {
            Box::pin(async move {
                if ctx.author().id == 123456789 {
                    return Ok(false);
                }
                Ok(true)
            })
        }),
        skip_checks_for_owners: false,
        ..Default::default()
    };

    // Create the framework with a conditional command registration approach
    let framework = poise::Framework::builder()
        .setup(move |ctx, ready, framework| {
            Box::pin(async move {
                println!("Logged in as {}", ready.user.name);
                if is_development {
                    // For development, register commands in a specific guild.
                    // Ensure your config provides a test_guild_id (as u64).
                    let guild_id = serenity::GuildId::new(config.test_guild_id);
                    match poise::builtins::register_in_guild(ctx, &framework.options().commands, guild_id).await {
                        Ok(_) => println!("Commands registered successfully in guild {:?}", guild_id),
                        Err(e) => println!("Failed to register commands in guild: {:?}", e),
                    }
                } else {
                    // For production, register commands globally (may take up to an hour to update)
                    match poise::builtins::register_globally(ctx, &framework.options().commands).await {
                        Ok(_) => println!("Commands registered successfully globally"),
                        Err(e) => println!("Failed to register commands: {:?}", e),
                    }
                }
                
                // Create the DynamoDB client and start the data fetcher.
                let db_client = create_dynamodb_client().await;
                let db_client_clone = db_client.clone();
                tokio::spawn(async move {
                    if let Err(e) = services::data::fetch_data(&db_client_clone).await {
                        println!("Error in data fetcher: {}", e);
                    }
                });
                Ok(Data {
                    votes: Mutex::new(HashMap::new()),
                    db_client,
                })
            })
        })
        .options(options)
        .build();

    // Discord token
    let token = if config.discord_token.is_empty() {
        panic!("Missing discord_token in config file");
    } else {
        config.discord_token.clone()
    };
    
    // Discord intents
    let intents = serenity::GatewayIntents::non_privileged() 
        | serenity::GatewayIntents::MESSAGE_CONTENT;

    // Create the client
    let mut client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await
        .expect("Error creating client");
    
    // Start the client
    client.start().await.expect("Error running client");
}

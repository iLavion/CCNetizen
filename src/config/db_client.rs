// File: config/db_client.rs
// AWS DynamoDB client creation

use aws_sdk_dynamodb::Client;
use aws_sdk_dynamodb::config::Credentials;
use crate::config::secret::Config;
use std::env;
use aws_types::region::Region;

pub async fn create_dynamodb_client() -> Client {
    // Load the configuration file
    let config = Config::from_file("config.toml")
        .expect("Failed to load config");

    // Check if the application is in development mode
    let is_dev = env::var("APP_ENV")
        .unwrap_or_else(|_| "production".to_string()) == "development";

    // Use different configurations based on the environment
    let (aws_access, aws_secret, aws_region, aws_endpoint) = if is_dev {
        (
            config.aws_access_dev,
            config.aws_secret_dev,
            config.aws_region_dev,
            config.aws_endpoint_dev,
        )
    } else {
        (
            config.aws_access,
            config.aws_secret,
            config.aws_region,
            config.aws_endpoint,
        )
    };

    // Create the AWS credentials
    let credentials = Credentials::new(
        aws_access, 
        aws_secret, 
        None, 
        None, 
        "ccnetizen"
    );

    // Create the AWS configuration
    let config = aws_config::defaults(aws_config::BehaviorVersion::latest())
        .credentials_provider(credentials)
        .region(Region::new(aws_region))
        .endpoint_url(aws_endpoint)
        .load()
        .await;

    // Create the DynamoDB client
    let dynamodb_config = aws_sdk_dynamodb::config::Builder::from(&config).build();
    Client::from_conf(dynamodb_config)
}
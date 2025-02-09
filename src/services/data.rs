// Filename: services/data_fetcher.rs
// Service to fetch data from a URL and extract information about a specific town

use reqwest;
use chrono;
use regex::Regex;
use serde_json::Value;
use tokio::time::{sleep, Duration};
use aws_sdk_dynamodb::Client;
use std::collections::HashMap;
use std::option::Option;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::models::towns::Town;
use crate::repositories::towns::TownRepository;

/// Fetches data from a specified URL in a loop, extracts information about a specific town,
/// and prints the details. The loop runs indefinitely with a delay between each fetch.
///
/// # Errors
///
/// This function will return an error if there is an issue with the HTTP request or JSON parsing.
pub async fn fetch_data(db_client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://map.ccnetmc.com/nationsmap/tiles/_markers_/marker_world.json";

    loop {
        println!("Fetching data from URL: {}", url);
        if let Err(e) = fetch_and_process_data(url, db_client).await {
            println!("Error fetching or processing data: {}", e);
        }
        sleep(Duration::from_secs(60)).await;
    }
}

async fn fetch_and_process_data(url: &str, db_client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::get(url).await?;
    if !response.status().is_success() {
        println!("Failed to fetch JSON, status: {}", response.status());
        return Ok(());
    }

    let json: Value = response.json().await?;
    if let Some(towny) = json.pointer("/sets/towny.markerset") {
        if let Some(areas) = towny.get("areas") {
            if let Err(e) = process_areas(areas, db_client).await {
                if e.to_string().contains("ValidationException") {
                    println!("ValidationException occurred: {}", e);
                    println!("Error details: {:?}", e);
                } else {
                    return Err(e);
                }
            }
        }
    }

    Ok(())
}

async fn process_areas(areas: &Value, db_client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    println!("Processing areas...");
    let mut astarte_found = false;
    let mut towns: HashMap<String, (Option<&Value>, Option<&Value>)> = HashMap::new();

    if let Some(areas_obj) = areas.as_object() {
        for (name, area) in areas_obj {
            let town_name = name.split("__").next().unwrap_or("");
            let entry = towns.entry(town_name.to_string()).or_insert((None, None));
            if name.ends_with("__home") {
                entry.1 = Some(area);
            } else {
                entry.0 = Some(area);
            }
        }

        for (town_name, (main_area, home_area)) in towns {
            if let Some(main_area) = main_area {
                if let Some(desc) = main_area.get("desc").and_then(|d| d.as_str()) {
                    let merged_desc = if let Some(home_area) = home_area {
                        if let Some(home_desc) = home_area.get("desc").and_then(|d| d.as_str()) {
                            format!("{}\n{}", desc, home_desc)
                        } else {
                            desc.to_string()
                        }
                    } else {
                        desc.to_string()
                    };

                    if town_name == "Astarte" {
                        astarte_found = true;
                        process_town_data(&town_name, &merged_desc, db_client, true).await?;
                    } else {
                        process_town_data(&town_name, &merged_desc, db_client, false).await?;
                    }
                }
            }
        }
    }

    if !astarte_found {
        println!("Reference town Astarte not found.");
    }

    Ok(())
}

async fn process_town_data(name: &str, desc: &str, db_client: &Client, print_info: bool) -> Result<(), Box<dyn std::error::Error>> {
    let bank = extract_value(desc, "Bank");
    let upkeep = extract_value(desc, "Upkeep");
    let culture = extract_value(desc, "Culture");
    let residents = extract_residents(desc);
    let mayor = extract_value(desc, "Mayor");
    let peaceful = extract_peaceful(desc);
    let board = extract_value(desc, "Board");
    let founded = extract_value(desc, "Founded");
    let resources = extract_resources(desc);
    let trusted_players = extract_trusted_players(desc);
    let nation = extract_nation(desc);

    let bank_value: f64 = parse_currency(&bank);
    let upkeep_value: f64 = parse_currency(&upkeep);
    let will_go_negative = bank_value - upkeep_value < 0.0;

    if print_info {
        println!(
            "\n\nTown: {}\nNation: {}\nMayor: {}\nPeaceful: {}\nCulture: {}\nBoard: {}\nBank: ${:.2}\nUpkeep: ${:.2}\nFounded: {}\nResources: {:?}\nResidents: {:?}\nTrusted Players: {:?}\nWill go negative: {}\n",
            name, nation, mayor, peaceful, culture, board, bank_value, upkeep_value, founded, resources, residents, trusted_players, will_go_negative
        );
    }

    let town = Town {
        town_name: name.to_string(),
        nation: Some(nation),
        mayor,
        peaceful,
        culture,
        board,
        bank: bank_value,
        upkeep: upkeep_value,
        founded: parse_date(&founded),
        resources,
        residents,
        trusted_players,
        area: 0.0, // Assuming area is not available in the current context
        coords: (0.0, 0.0), // Assuming coords are not available in the current context
        last_updated: SystemTime::now().duration_since(UNIX_EPOCH).map_err(|e| Box::<dyn std::error::Error>::from(e))?.as_secs() as i64,
    };

    save_town_data(db_client, town).await?;

    Ok(())
}

pub async fn ensure_table_exists(db_client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    let table_name = "towns";
    
    // Check if table exists
    let tables = db_client.list_tables().send().await?;
    let opt_names: Option<&[String]> = Some(tables.table_names());
    let table_names: &[String] = opt_names.unwrap_or(&[]);

    if table_names.contains(&table_name.to_string()) {
        println!("Table {} exists.", table_name);
    } else {
        println!("Table {} does not exist.", table_name);
    }

    Ok(())
}

pub async fn save_town_data(db_client: &Client, town: Town) -> Result<(), Box<dyn std::error::Error>> {
    let repository = TownRepository::new(db_client);
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)?
        .as_secs() as i64;

    // Update the town data with the current timestamp
    let mut updated_town = town;
    updated_town.last_updated = now;
    repository.save_town(&updated_town).await?;

    Ok(())
}

/// Extracts a value associated with a given key from a description string.
///
/// # Arguments
///
/// * `desc` - A string slice that holds the description.
/// * `key` - A string slice that holds the key to search for.
///
/// # Returns
///
/// A string containing the extracted value. If the key is not found, returns "0".
fn extract_value(desc: &str, key: &str) -> String {
    // This regex matches:
    //   - a <span style="font-weight:bold"> tag with any characters (non-greedy) before the key
    //   - the key (e.g., "Bank") followed by optional whitespace
    //   - the closing </span> and a colon
    //   - optional whitespace followed by the value (captured non-greedily)
    //   - ending when a <br tag is encountered.
    let pattern = format!(r#"<span style="font-weight:bold">.*?{}\s*</span>:\s*(.*?)<br"#, regex::escape(key));
    let re = Regex::new(&pattern).unwrap();
    if let Some(caps) = re.captures(desc) {
        caps.get(1).map_or("0".to_string(), |m| m.as_str().trim().to_string())
    } else {
        "0".to_string()
    }
}

/// Extracts residents from the description string.
/// 
/// # Arguments
/// 
/// * `desc` - A string slice that holds the description.
/// 
/// # Returns
/// 
/// A vector of strings containing the residents' names.
fn extract_residents(desc: &str) -> Vec<String> {
    // This regex captures the text after the bold "Residents (n)" label until the next <br
    let re = Regex::new(r#"<span style="font-weight:bold">.*?Residents\s*\(\d+\)\s*</span>:\s*(.*?)<br"#).unwrap();
    if let Some(caps) = re.captures(desc) {
        let residents_str = caps.get(1).map(|m| m.as_str().trim()).unwrap_or("");
        residents_str.split(',')
            .filter(|s| !s.is_empty())
            .map(|s| s.trim().to_string())
            .collect()
    } else {
        vec![]
    }
}

/// Parses a currency string and converts it to a floating-point number.
///
/// # Arguments
///
/// * `value` - A string slice that holds the currency value.
///
/// # Returns
///
/// A floating-point number representing the parsed currency value. If parsing fails, returns 0.0.
fn parse_currency(value: &str) -> f64 {
    value.trim_start_matches('$').replace(",", "").parse().unwrap_or(0.0)
}

/// Extracts resources from the description string.
/// 
/// # Arguments
/// 
/// * `desc` - A string slice that holds the description.
/// 
/// # Returns
/// 
/// A vector of strings containing the resources.
fn extract_resources(desc: &str) -> Vec<String> {
    let re = Regex::new(r#"<span style=\"font-weight:bold\">.*?Resources\s*</span>:\s*(.*?)<br"#).unwrap();
    if let Some(caps) = re.captures(desc) {
        let resources_str = caps.get(1).map(|m| m.as_str().trim()).unwrap_or("");
        return resources_str
            .split(',')
            .map(|s| s.trim().to_string())
            .collect();
    }
    vec![]
}

/// Extracts trusted players from the description string.
/// 
/// # Arguments
/// 
/// * `desc` - A string slice that holds the description.
/// 
/// # Returns
/// 
/// A vector of strings containing the trusted players' names.
fn extract_trusted_players(desc: &str) -> Vec<String> {
    let key_with_colon = "Trusted Players";
    // Updated regex pattern to more robustly capture the trusted players
    let re = Regex::new(r#"<span style=\"font-weight:bold\">.*?Trusted Players<\/span>: (.*?)(<br|<\/div>)"#).unwrap();
    
    if let Some(caps) = re.captures(desc) {
        let players_str = caps.get(1).map(|m| m.as_str().trim()).unwrap_or("");
        // Split by commas, trim whitespace, and collect as a vector of strings
        return players_str.split(',')
            .map(|s| s.trim().to_string())
            .collect();
    }
    vec![] // Return an empty vector if no trusted players are found
}

/// Parses a date string and converts it to a Unix timestamp.
/// 
/// # Arguments
/// 
/// * `date` - A string slice that holds the date.
/// 
/// # Returns
/// 
/// A Unix timestamp representing the parsed date. If parsing fails, returns 0.
fn parse_date(date: &str) -> i64 {
    // Assuming the date format is "Dec 1 2024"
    let format = "%b %d %Y";
    if let Ok(parsed_date) = chrono::NaiveDate::parse_from_str(date, format) {
        parsed_date.and_hms_opt(0, 0, 0).map_or(0, |dt| dt.and_utc().timestamp())
    } else {
        0
    }
}

/// Extracts the nation name from the description string.
/// 
/// # Arguments
/// 
/// * `desc` - A string slice that holds the description.
/// 
/// # Returns
/// 
/// A string containing the nation name.
fn extract_nation(desc: &str) -> String {
    let re = Regex::new(r#"<span style="font-size:150%">Member of (.*?)</span>"#).unwrap();
    if let Some(caps) = re.captures(desc) {
        caps.get(1).map_or("".to_string(), |m| m.as_str().trim().to_string())
    } else {
        "".to_string()
    }
}

/// Extracts the peaceful status from the description string.
/// 
/// # Arguments
/// 
/// * `desc` - A string slice that holds the description.
/// 
/// # Returns
/// 
/// A boolean indicating whether the town is peaceful.
fn extract_peaceful(desc: &str) -> bool {
    let re = Regex::new(r#"<span style="font-weight:bold">.*?Peaceful\?\s*</span>\s*(true|false)"#).unwrap();
    if let Some(caps) = re.captures(desc) {
        caps.get(1).map_or(false, |m| m.as_str().trim().to_lowercase() == "true")
    } else {
        false
    }
}
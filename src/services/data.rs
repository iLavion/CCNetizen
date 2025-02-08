// Filename: services/data_fetcher.rs
// Service to fetch data from a URL and extract information about a specific town

use reqwest::Error;
use serde_json::Value;
use tokio::time::{sleep, Duration};

/// Fetches data from a specified URL in a loop, extracts information about a specific town,
/// and prints the details. The loop runs indefinitely with a delay between each fetch.
///
/// # Errors
///
/// This function will return an error if there is an issue with the HTTP request or JSON parsing.
pub async fn fetch_data() -> Result<(), Error> {
    let url = "https://map.ccnetmc.com/nationsmap/tiles/_markers_/marker_world.json";

    loop {
        println!("Fetching data from URL: {}", url);
        match reqwest::get(url).await {
            Ok(response) => {
                if response.status().is_success() {
                    println!("Successfully fetched data.");
                    let json: Value = response.json().await?;
                    if let Some(sets) = json.get("sets") {
                        if let Some(towny) = sets.get("towny.markerset") {
                            if let Some(areas) = towny.get("areas") {
                                println!("Searching for town 'Astarte'...");
                                let mut found = false;
                                for (name, area) in areas.as_object().unwrap() {
                                    if name.starts_with("Astarte__") {
                                        found = true;
                                        if let Some(desc) = area.get("desc") {
                                            let desc_str = desc.as_str().unwrap();
                                            let bank = extract_value(desc_str, "Bank");
                                            let upkeep = extract_value(desc_str, "Upkeep");
                                            let culture = extract_value(desc_str, "Culture");
                                            let bank_value: f64 = parse_currency(&bank);
                                            let upkeep_value: f64 = parse_currency(&upkeep);
                                            let will_go_negative = bank_value - upkeep_value < 0.0;
                                            println!("Town: {}\nCulture: {}\nBank: ${}\nUpkeep: ${}\nWill go negative: {}\n", name, culture, bank, upkeep, will_go_negative);
                                        }
                                    }
                                }
                                if !found {
                                    println!("Town 'Astarte' not found.");
                                }
                            }
                        }
                    }
                } else {
                    println!("Failed to fetch JSON, status: {}", response.status());
                }
            }
            Err(e) => {
                println!("Error fetching JSON: {}", e);
            }
        }
        // Wait for a specified interval before checking again
        sleep(Duration::from_secs(60)).await;
    }
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
    let key_with_colon = format!("{}:</span>", key);
    if let Some(start) = desc.find(&key_with_colon) {
        let start = start + key_with_colon.len();
        if let Some(end) = desc[start..].find("<br />") {
            return desc[start..start + end].trim().to_string();
        }
    }
    "0".to_string()
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
mod cli_args;
mod fetch;

use clap::Parser;
use cli_args::{check_ascii, CliArgs};
use fetch::current_weather::fetch_current_weather;
use std::process;

fn main() {
    let query = CliArgs::parse();

    let city = check_ascii(query).unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        process::exit(1);
    });

    show_current_weather(&city);
}

fn show_current_weather(city: &str) {
    let current_weather_result = fetch_current_weather(city);

    match current_weather_result {
        Ok(response) => {
            let mut location_parts = Vec::new();

            if !response.location.name.is_empty() {
                location_parts.push(response.location.name.clone());
            }

            if !response.location.region.is_empty() {
                location_parts.push(response.location.region.clone());
            }

            if !response.location.country.is_empty() {
                location_parts.push(response.location.country.clone());
            }

            println!("🌍 Location: {}", location_parts.join(", "));
            println!("🌡️ Temperature: {}°C", response.current.temp_c);
            println!("☁️ Condition: {}", response.current.condition.text);
        }
        Err(err) => {
            eprintln!("❌ Request Failed: {}", err);
            process::exit(1);
        }
    }
}

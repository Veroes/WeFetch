mod cli_args;
mod config;
mod fetch;
mod types;

use clap::Parser;
use cli_args::{check_validity, CliArgs};
use dotenv::dotenv;
use fetch::weather_api::fetch_current_weather;
use std::process;

fn main() {
    dotenv().ok();

    let query = CliArgs::parse();

    let query = check_validity(query).unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        process::exit(1);
    });

    let service = match query.service {
        cli_args::Service::Current => "current",
        cli_args::Service::Forecast => "forecast",
        cli_args::Service::History => "history",
    };

    match service {
        "current" => show_current_weather(&query.city),
        // TODO: implemenet forecast and history
        _ => {
            eprintln!("❌ no satisified arguments");
            process::exit(1);
        }
    }
}

fn show_current_weather(city: &str) {
    let current_weather_result = fetch_current_weather(city);

    match current_weather_result {
        Ok(response) => {
            let mut location_parts = Vec::new();

            if !response.location.name.is_empty() {
                location_parts.push(response.location.name);
            }

            if !response.location.region.is_empty() {
                location_parts.push(response.location.region);
            }

            if !response.location.country.is_empty() {
                location_parts.push(response.location.country);
            }

            println!("🌍 Location: {}", location_parts.join(", "));
            println!("🌡️ Temperature: {}°C", response.current.weather.temp_c);
            println!("☁️ Condition: {}", response.current.weather.condition.text);
            println!("🕒 Local Time: {}", response.location.localtime);
        }
        Err(err) => {
            eprintln!("❌ Request Failed: {}", err);
            process::exit(1);
        }
    }
}

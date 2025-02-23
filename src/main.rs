mod config;
mod display;
mod service;
mod types;
mod ui;

use clap::Parser;
use config::cli_args::{self, check_validity, CliArgs};
use display::{show_current_weather, show_forecast_weather};
use dotenv::dotenv;
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
        "forecast" => {
            let days = query.days.unwrap_or(1);
            show_forecast_weather(&query.city, days)
        }
        // TODO: implemenet history
        "history" => {
            eprintln!("❌ History service not implemented yet");
            process::exit(1);
        }
        _ => {
            eprintln!("❌ no satisified arguments");
            process::exit(1);
        }
    }
}

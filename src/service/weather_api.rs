use crate::config::env::get_api_config;
use crate::types::weather::{ForecastResponse, WeatherResponse};
use reqwest;

pub fn fetch_current_weather(city: &str) -> Result<WeatherResponse, reqwest::Error> {
    let (base_url, api_key) = get_api_config().unwrap();

    let city = city.replace("_", "%20");

    let request = format!("{}/current.json?key={}&q={}", base_url, api_key, city);

    let response = reqwest::blocking::get(&request)?.json::<WeatherResponse>()?;

    Ok(response)
}

pub fn fetch_forecast_weather(city: &str, days: u8) -> Result<ForecastResponse, reqwest::Error> {
    let (base_url, api_key) = get_api_config().unwrap();

    let city = city.replace("_", "%20");

    let request = format!(
        "{}/forecast.json?key={}&q={}&days={}",
        base_url, api_key, city, days
    );

    let response = reqwest::blocking::get(&request)?.json::<ForecastResponse>()?;

    Ok(response)
}

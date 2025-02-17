use crate::config::api::get_api_config;
use crate::types::weather::WeatherResponse;
use reqwest;

pub fn fetch_current_weather(city: &str) -> Result<WeatherResponse, reqwest::Error> {
    let (base_url, api_key) = get_api_config().unwrap();

    let request = format!("{}/current.json?key={}&q={}", base_url, api_key, city);

    let response = reqwest::blocking::get(&request)?.json::<WeatherResponse>()?;

    Ok(response)
}

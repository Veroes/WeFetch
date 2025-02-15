use crate::fetch::weather::WeatherResponse;
use dotenv::dotenv;
use reqwest;
use std::env;

pub fn fetch_current_weather(city: &str) -> Result<WeatherResponse, reqwest::Error> {
    dotenv().ok();

    let base_url = env::var("BASE_URL").unwrap();
    let api_key = env::var("API_KEY").unwrap();

    let request = format!("{}/current.json?key={}&q={}", base_url, api_key, city);

    let response = reqwest::blocking::get(&request)?.json::<WeatherResponse>()?;

    Ok(response)
}

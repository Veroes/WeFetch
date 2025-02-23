use crate::service::weather_api::{fetch_current_weather, fetch_forecast_weather};
use std::process;

pub fn show_current_weather(city: &str) {
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
            println!("🕒 Time: {}", response.location.localtime);
            println!("Weather Statistics:");
            println!("🌡️ Feels Like: {}°C", response.current.weather.feelslike_c);
            println!(
                "💨 Wind Speed: {} mph, {} kph, ",
                response.current.weather.wind_kph, response.current.weather.wind_mph
            );
            println!(
                "🌬️ Wind Direction: {} degress to {}",
                response.current.weather.wind_degree, response.current.weather.wind_dir
            );
            println!("💧 Humidity: {}%", response.current.weather.humidity);
            println!("🌞 UV Index: {}", response.current.weather.uv);
            println!("👓 Visibility: {} km", response.current.weather.vis_km);
        }
        Err(err) => {
            eprintln!("❌ Request Failed: {}", err);
            process::exit(1);
        }
    }
}

pub fn show_forecast_weather(city: &str, days: u8) {
    let forecast_weather_result = fetch_forecast_weather(city, days);

    match forecast_weather_result {
        Ok(response) => {
            let mut location_parts = Vec::new();

            if !response.weather_response.location.name.is_empty() {
                location_parts.push(response.weather_response.location.name);
            }

            if !response.weather_response.location.region.is_empty() {
                location_parts.push(response.weather_response.location.region);
            }

            if !response.weather_response.location.country.is_empty() {
                location_parts.push(response.weather_response.location.country);
            }

            println!("🌍 Location: {}", location_parts.join(", "));
            println!(
                "📅 Forecast for the next {} days:",
                response.forecast.forecastday.len()
            );

            for day in response.forecast.forecastday {
                println!("📅 Date: {}", day.date);
                println!("☁️ Condition: {}", day.day.condition.text);
                println!("🌡️ Sunrise: {}", day.astro.sunrise);
                println!("🌡️ Sunset: {}", day.astro.sunset);
                println!("🌙 Moonrise: {}", day.astro.moonrise);
                println!("🌙 Moonset: {}", day.astro.moonset);
                println!("🌙 Moon Phase: {}", day.astro.moon_phase);
                println!("🌙 Moon Illumination: {}", day.astro.moon_illumination);

                println!("🕒 Hourly Forecast:");
                for hour in day.hour {
                    println!("🕒 Time: {}", hour.time);
                    println!("🌡️ Temperature: {}°C", hour.weather.temp_c);
                    println!("☁️ Condition: {}", hour.weather.condition.text);
                    println!("🌡️ Feels Like: {}°C", hour.weather.feelslike_c);
                    println!(
                        "💨 Wind Speed: {} mph, {} kph, ",
                        hour.weather.wind_kph, hour.weather.wind_mph
                    );
                    println!(
                        "🌬️ Wind Direction: {} degress to {}",
                        hour.weather.wind_degree, hour.weather.wind_dir
                    );
                    println!("💧 Humidity: {}%", hour.weather.humidity);
                    println!("🌞 UV Index: {}", hour.weather.uv);
                    println!("👓 Visibility: {} km", hour.weather.vis_km);
                }
            }
        }
        Err(err) => {
            eprintln!("❌ Request Failed: {}", err);
            process::exit(1);
        }
    }
}

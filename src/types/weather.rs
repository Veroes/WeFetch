use serde::Deserialize;

#[derive(Deserialize)]
pub struct Location {
    pub name: String,
    pub region: String,
    pub country: String,
    pub localtime: String,
}

#[derive(Deserialize)]
pub struct Condition {
    pub text: String,
}

#[derive(Deserialize)]
pub struct WeatherBase {
    pub temp_c: f64,
    pub condition: Condition,
    pub feelslike_c: f64,
    pub wind_mph: f64,
    pub wind_kph: f64,
    pub wind_degree: f64,
    pub wind_dir: String,
    pub humidity: f64,
    pub uv: f64,
    pub vis_km: f64,
}

#[derive(Deserialize)]
pub struct Day {
    pub condition: Condition,
}

#[derive(Deserialize)]
pub struct Current {
    #[serde(flatten)]
    pub weather: WeatherBase,
}

#[derive(Deserialize)]
pub struct Astro {
    pub sunrise: String,
    pub sunset: String,
    pub moonrise: String,
    pub moonset: String,
    pub moon_phase: String,
    pub moon_illumination: String,
}

#[derive(Deserialize)]
pub struct Hour {
    pub time: String,
    #[serde(flatten)]
    pub weather: WeatherBase,
}

#[derive(Deserialize)]
pub struct ForecastDay {
    pub date: String,
    pub day: Day,
    pub astro: Astro,
    pub hour: Vec<Hour>,
}

#[derive(Deserialize)]
pub struct Forecast {
    pub forecastday: Vec<ForecastDay>,
}

#[derive(Deserialize)]
pub struct WeatherResponse {
    pub location: Location,
    pub current: Current,
}

#[derive(Deserialize)]
pub struct ForecastResponse {
    pub location: Location,
    pub current: Current,
    pub forecast: Forecast,
}

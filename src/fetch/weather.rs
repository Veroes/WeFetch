use serde::Deserialize;

#[derive(Deserialize)]
pub struct WeatherResponse {
    pub location: Location,
    pub current: Current,
}

#[derive(Deserialize)]
pub struct Location {
    pub name: String,
    pub region: String,
    pub country: String,
}

#[derive(Deserialize)]
pub struct Current {
    pub temp_c: f64,
    pub condition: Condition,
}

#[derive(Deserialize)]
pub struct Condition {
    pub text: String,
}

use std::env;

pub fn get_api_config() -> Result<(String, String), String> {
    let base_url = env::var("BASE_URL").map_err(|_| "BASE_URL must be set in .env")?;
    let api_key = env::var("API_KEY").map_err(|_| "API_KEY must be set in .env")?;

    Ok((base_url, api_key))
}

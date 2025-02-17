use clap::{Parser, ValueEnum};

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Service {
    Current,
    Forecast,
    History,
}

#[derive(Parser)]
#[command(name = "WeFetch")]
#[command(version = "1.1")]
#[command(about = "Fetches weather data from weatherapi.com", long_about = None)]
#[command(propagate_version = true)]
pub struct CliArgs {
    /// City location (it could also be country or region)
    #[arg(short, long)]
    pub city: String,

    /// Service types
    #[arg(value_enum, default_value_t = Service::Current)]
    pub service: Service,

    /// Number of days of forecast data (min: 1 day, max: 14 days)
    #[arg(short, long)]
    pub days: Option<u8>,
}

pub fn check_validity(args: CliArgs) -> Result<CliArgs, String> {
    let city = &args.city;

    if !check_ascii_city(city) {
        return Err(String::from("City name must be ASCII"));
    }

    // Service type validity handled by clap

    Ok(args)
}

fn check_ascii_city(city: &str) -> bool {
    city.is_ascii()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_ascii_city() {
        let args = CliArgs {
            city: String::from("Jakarta"),
            service: Service::Current,
            days: Some(1),
        };
        assert!(check_validity(args).is_ok());
    }

    #[test]
    fn test_invalid_ascii_city() {
        let args = CliArgs {
            city: String::from("🗿"),
            service: Service::Current,
            days: Some(1),
        };

        assert!(check_validity(args).is_err());
    }

    // Service type validity is handled by clap,
    // so I don't know how to test it or it needs to be tested
}

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

    /// Number of days of forecast data (min: 1 day, max: 14 days / 3 days if you're using free plan)
    #[arg(short, long, default_value = "1")]
    pub days: Option<u8>,
}

pub fn check_validity(args: CliArgs) -> Result<CliArgs, String> {
    // City
    let city = &args.city;

    if !check_ascii_city(city) {
        return Err(String::from("City name must be ASCII"));
    }

    // Service type validity handled by clap

    // Days
    if let Some(days) = args.days {
        if args.service != Service::Forecast && days != 1 {
            return Err(String::from(
                "Forecast days only available for forecast service",
            ));
        }

        if days < 1 || days > 3 {
            // change 14 to 3 if you're using free plan
            return Err(String::from("Days must be between 1 and 3"));
        }
    }

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

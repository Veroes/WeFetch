use clap::Parser;

#[derive(Parser)]
#[command(name = "WeFetch")]
#[command(version = "1.0")]
#[command(about = "Fetches weather data from weatherapi.com", long_about = None)]
pub struct CliArgs {
    #[arg(short, long)]
    city: String,
}

pub fn check_ascii(args: CliArgs) -> Result<String, String> {
    let city = args.city;

    if !city.is_ascii() {
        return Err(String::from("City name must be ASCII"));
    }

    Ok(city)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_ascii_city() {
        let args = CliArgs {
            city: String::from("Jakarta"),
        };
        assert!(check_ascii(args).is_ok());
    }

    #[test]
    fn test_invalid_ascii_city() {
        let args = CliArgs {
            city: String::from("🗿"),
        };

        assert!(check_ascii(args).is_err());
    }
}

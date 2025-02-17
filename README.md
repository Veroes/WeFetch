# ☁️🛜 WeFetch - Weather Fetcher CLI

WeFetch is a simple CLI tool built with Rust for fetching weather data from [WeatherAPI](https://www.weatherapi.com/). This project was created as a learning exercise but can be used to quickly check the weather from your terminal.

## 🚀 Getting Started

### 1️⃣ Clone the Repository

```sh
git clone https://github.com/Veroes/WeFetch.git
cd wefetch
```

### 2️⃣ Set Up Environment Variables

Create a `.env` file by copying `.env.example`:

```sh
cp .env.example .env
```

Then, open `.env` and add your API key from [WeatherAPI](https://www.weatherapi.com/).

### 3️⃣ Build and Run

Run the application with:

```sh
cargo run -- --city [YOUR_CITY]
```

use `cargo run -- --help` for more options.

## 📌 Features

- Fetch current weather data by city name

- Display location, temperature, and condition

- Fetch forecast weather data (soon)

## 🧩 Next Steps?

- 🎯 Improve `--help` output for better usability

- 🎯 Add support for more weather data details
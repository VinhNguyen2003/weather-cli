# Weather CLI

## Description
Weather CLI is a simple command-line interface tool written in Rust that provides current weather information for a given city. It uses the OpenWeatherMap API to fetch weather data.

I initially started this simple project to learn Rust, but I might end up using this quite often since I work mostly on the terminal now.

## Features
- Get current weather data for any city.
- Display temperature in Celsius or Fahrenheit.
- View detailed weather information.
- Simple and easy to use command-line interface.

## Installation
Before you begin, ensure you have Rust installed on your system.

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
1. Clone the repository:
```
git clone https://github.com/your-username/weather-cli.git
cd weather-cli
```
2. Build the project:
```
cargo build
```
## Usage

To use Weather CLI, you'll need an API key from OpenWeatherMap.

Set your OpenWeatherMap API key as an environment variable:
```
export OPENWEATHERMAP_API_KEY=your_api_key
```

Run the program with the desired city:
```
cargo run -- -c "City Name"
```

### Optional Arguments

- To specify the temperature unit (Celsius or Fahrenheit):
```
cargo run -- -c "City Name" -u C
```
Replace `C` with `F` for Fahrenheit.
- To view detailed weather information:
```
cargo run -- -c "City Name" -d
```

Replace "City Name" with the name of the city you want to check the weather for.

## Configuration

The tool uses environment variables for configuration.

OPENWEATHERMAP_API_KEY: Your API key for OpenWeatherMap.

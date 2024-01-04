extern crate clap;
extern crate tokio;
extern crate reqwest;
extern crate serde_json;

use std::env;
use serde_json::Value;
use clap::{Arg, App};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    // Set up command line argument parsing
    let matches = App::new("Weather CLI")
        .version("1.0")
        .author("Vinh Nguyen")
        .about("Gets weather information")
        .arg(Arg::with_name("city")
            .short("c")
            .long("city")
            .value_name("CITY")
            .help("Sets the city to get weather information for")
            .takes_value(true))
        .arg(Arg::with_name("unit")
            .short("u")
            .long("unit")
            .value_name("UNIT")
            .help("Sets the unit for temperature (C for Celsius, F for Fahrenheit)")
            .takes_value(true))
        .get_matches();

    // Retrieve the city name from the command line arguments, default to "Minneapolis"
    let city = matches.value_of("city").unwrap_or("Minneapolis");
    let unit = matches.value_of("unit").unwrap_or("C");
    println!("Getting weather information for: {}", city);

    let api_key = env::var("OPENWEATHERMAP_API_KEY").expect("OPENWEATHERMAP_API_KEY not set");
    let url = format!("http://api.openweathermap.org/data/2.5/weather?q={}&appid={}", city, api_key);
    let response = reqwest::get(&url).await?;
    let weather_data = response.json::<serde_json::Value>().await?;
    display_weather(&weather_data, unit);

    Ok(())
}

fn display_weather(weather_data: &Value, unit: &str) {
    let city_name = weather_data["name"].as_str().unwrap_or("Unknown");
    let country = weather_data["sys"]["country"].as_str().unwrap_or("Unknown");
    let main = weather_data["weather"][0]["main"].as_str().unwrap_or("Unknown");
    let description = weather_data["weather"][0]["description"].as_str().unwrap_or("Unknown");
    let temp_kelvin = weather_data["main"]["temp"].as_f64().unwrap_or(0.0);
    let feels_like_k = weather_data["main"]["feels_like"].as_f64().unwrap_or(0.0);
    let humidity = weather_data["main"]["humidity"].as_u64().unwrap_or(0);
    let pressure = weather_data["main"]["pressure"].as_u64().unwrap_or(0);

    let (temp, feels_like, unit_label) = match unit {
        "F" | "f" => (kelvin_to_fahrenheit(temp_kelvin), kelvin_to_fahrenheit(feels_like_k), "°F"),
        _ => (kelvin_to_celsius(temp_kelvin), kelvin_to_celsius(feels_like_k), "°C"),
    };

    println!("Weather in {}, {}:", city_name, country);
    println!("  Condition: {} ({})", main, description);
    println!("  Temperature: {:.2} {}", temp, unit_label);
    println!("  Feels like: {:.2} {}", feels_like, unit_label);
    println!("  Humidity: {}%", humidity);
    println!("  Pressure: {} hPa", pressure);
}

fn kelvin_to_celsius(kelvin: f64) -> f64 {
    kelvin - 273.15
}

fn kelvin_to_fahrenheit(kelvin: f64) -> f64 {
    (kelvin - 273.15) * 9.0/5.0 + 32.0
}
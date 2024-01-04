extern crate clap;
extern crate tokio;
extern crate reqwest;
extern crate serde_json;

use std::env;
use serde_json::Value;
use clap::{Arg, App};
use chrono::{Utc, Local, TimeZone};

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
        .arg(Arg::with_name("details")
            .short("d")
            .long("details")
            .help("Displays detailed weather information")
            .takes_value(false))
        .get_matches();

    // Retrieve the city name from the command line arguments, default to "Minneapolis"
    let city = matches.value_of("city").unwrap_or("Minneapolis");
    let unit = matches.value_of("unit").unwrap_or("C");
    let details = matches.is_present("details");
    println!("Getting weather information for: {}", city);

    let api_key = env::var("OPENWEATHERMAP_API_KEY").expect("OPENWEATHERMAP_API_KEY not set");
    let url = format!("http://api.openweathermap.org/data/2.5/weather?q={}&appid={}", city, api_key);
    let response = reqwest::get(&url).await?;
    let weather_data = response.json::<serde_json::Value>().await?;
    display_weather(&weather_data, unit, details);

    Ok(())
}

fn display_weather(weather_data: &Value, unit: &str, details: bool) {
    let city_name = weather_data["name"].as_str().unwrap_or("Unknown");
    let country = weather_data["sys"]["country"].as_str().unwrap_or("Unknown");
    let main = weather_data["weather"][0]["main"].as_str().unwrap_or("Unknown");
    let description = weather_data["weather"][0]["description"].as_str().unwrap_or("Unknown");
    let temp = kelvin_to_unit(weather_data["main"]["temp"].as_f64().unwrap_or(0.0), unit);
    let feels_like = kelvin_to_unit(weather_data["main"]["feels_like"].as_f64().unwrap_or(0.0), unit);
    let humidity = weather_data["main"]["humidity"].as_u64().unwrap_or(0);
    let sunrise = weather_data["sys"]["sunrise"].as_u64().unwrap_or(0);
    let sunset = weather_data["sys"]["sunset"].as_u64().unwrap_or(0);

    let unit_label = match unit {
        "F" | "f" => "°F",
        _ => "°C",
    };

    println!("Weather in {}, {}:", city_name, country);
    println!("  Condition: {} ({})", main, description);
    println!("  Temperature: {:.2} {}", temp, unit_label);
    println!("  Feels like: {:.2} {}", feels_like, unit_label);
    println!("  Humidity: {}%", humidity);
    println!("  Sunrise: {}", convert_unix_to_readable_time(sunrise));
    println!("  Sunset: {}", convert_unix_to_readable_time(sunset));

    if details {
        println!("Detailed Weather Information:");

        let visibility = weather_data["visibility"].as_u64().unwrap_or(0);
        let wind_speed = weather_data["wind"]["speed"].as_f64().unwrap_or(0.0);
        let wind_deg = weather_data["wind"]["deg"].as_u64().unwrap_or(0);
        let cloudiness = weather_data["clouds"]["all"].as_u64().unwrap_or(0);
        let pressure = weather_data["main"]["pressure"].as_u64().unwrap_or(0);
        let sea_level = weather_data["main"]["sea_level"].as_u64().unwrap_or(0);
        let ground_level = weather_data["main"]["grnd_level"].as_u64().unwrap_or(0);
        let temp_min = kelvin_to_unit(weather_data["main"]["temp_min"].as_f64().unwrap_or(0.0), unit);
        let temp_max = kelvin_to_unit(weather_data["main"]["temp_max"].as_f64().unwrap_or(0.0), unit);

        println!("  Visibility: {} meters", visibility);
        println!("  Wind Speed: {} m/s", wind_speed);
        println!("  Wind Direction: {} degrees", wind_deg);
        println!("  Cloudiness: {}%", cloudiness);
        println!("  Pressure: {} hPa", pressure);
        println!("  Sea Level: {} hPa", sea_level);
        println!("  Ground Level: {} hPa", ground_level);
        println!("  Minimum Temperature: {:.2} {}", temp_min, unit_label);
        println!("  Maximum Temperature: {:.2} {}", temp_max, unit_label);
    }
}

fn kelvin_to_unit(kelvin: f64, unit: &str) -> f64 {
    match unit {
        "F" | "f" => (kelvin - 273.15) * 9.0/5.0 + 32.0,
        _ => kelvin - 273.15,
    }
}

fn convert_unix_to_readable_time(unix_time: u64) -> String {
    match Utc.timestamp_opt(unix_time as i64, 0) {
        chrono::LocalResult::Single(datetime) => {
            let local_datetime = datetime.with_timezone(&Local);
            local_datetime.format("%H:%M:%S").to_string()
        },
        _ => "Invalid timestamp".to_string(),
    }
}
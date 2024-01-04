use std::env;

extern crate clap;
use clap::{Arg, App};
extern crate tokio;
extern crate reqwest;
extern crate serde_json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    // Set up command line argument parsing
    let matches = App::new("Weather CLI")
        .version("1.0")
        .author("Your Name")
        .about("Gets weather information")
        .arg(Arg::with_name("city")
             .short("c")
             .long("city")
             .value_name("CITY")
             .help("Sets the city to get weather information for")
             .takes_value(true))
        .get_matches();

    // Retrieve the city name from the command line arguments, default to "Minneapolis"
    let city = matches.value_of("city").unwrap_or("Minneapolis");
    println!("Getting weather information for: {}", city);

    let api_key = env::var("OPENWEATHERMAP_API_KEY").expect("OPENWEATHERMAP_API_KEY not set");
    let url = format!("http://api.openweathermap.org/data/2.5/weather?q={}&appid={}", city, api_key);
    let response = reqwest::get(&url).await?;
    let weather_data = response.json::<serde_json::Value>().await?;
    println!("{:#?}", weather_data);

    Ok(())
}

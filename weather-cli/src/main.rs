use std::env;
use client::weather;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        print!("Usage: {} <city>",args[0]);
        return Ok(());
    }
    let city = &args[1];
    // Check environment variables
    let api_key = match env::var("WEATHER_API_KEY") {
        Ok(val) => val,
        Err(_) => {
            eprint!("Error: WEATHER_API_KEY environment variable not set");
            return Ok(());
        }
    };

    weather(&api_key,city).await
}
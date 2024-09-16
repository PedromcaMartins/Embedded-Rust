use anyhow::anyhow;
use clap::Parser;
use weather_fetcher::WeatherData;

const API_KEY: &str = "d5b73c1429f54311967103833240407";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // parse command line arguments
    let args = weather_fetcher::Args::parse();

    // http request
    let url = format!(
        "http://api.weatherapi.com/v1/forecast.json?key={}&q={}&days={}", 
        args.api_key.unwrap_or(API_KEY.to_string()),
        args.city,
        args.days
    );
    let res = reqwest::get(url)
        .await?;

    // parser json
    match res.status().as_u16() {
        200 => (),
        status => return Err(anyhow!("Failed to fetch data from API. Response status code: {}", status)),
    };

    let body = res.text().await?;

    // deserialize json
    let weather_data = match serde_json::from_str::<WeatherData>(&body) {
        Ok(data) => data,
        Err(e) => return Err(anyhow!("Failed to parse data from API: {}", e)),
    };

    println!("weather_data: \n{}", weather_data);

    Ok(())
}
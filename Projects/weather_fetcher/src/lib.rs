mod weather_data;
pub use weather_data::WeatherData;


#[derive(clap::Parser)]
#[clap(version, about, long_about = "")]
pub struct Args {
    /// city name to fetch weather data
    #[clap(short, long, required = true)]
    pub city: String,

    /// API key to fetch weather data
    #[clap(short, long = "apikey")]
    pub api_key: Option<String>,

    /// number of days to fetch weather data
    #[clap(short, long, default_value = "1")]
    pub days: u8,
}
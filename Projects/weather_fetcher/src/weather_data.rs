use std::fmt::Display;

#[derive(serde::Deserialize, Debug)]
pub struct WeatherData {
    location: Location,
    current: Current,
}

#[derive(serde::Deserialize, Debug)]
struct Location {
    name: String,
    country: String,
    localtime: String
}

#[derive(serde::Deserialize, Debug)]
struct Current {
    condition: Condition,
    temp_c: f32,
    humidity: f32,
    uv: f32,
}

#[derive(serde::Deserialize, Debug)]
struct Condition {
    text: String,
}

impl Display for WeatherData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, 
            "{}, {} ({})",
            self.location.name,
            self.location.country,
            self.location.localtime,
        )?;
        writeln!(f, "Condition: {}", self.current.condition.text)?;
        writeln!(f, "Temperature: {}C", self.current.temp_c)?;
        writeln!(f, "Humidity: {}%", self.current.humidity)?;
        writeln!(f, "UV level: {}", self.current.uv)?;

        Ok(())
    }
}
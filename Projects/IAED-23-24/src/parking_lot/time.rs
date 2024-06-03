use std::{fmt::Display, str::FromStr};

use crate::error::AppError;

#[derive(Debug)]
pub struct Time {
    hour: i32,
    minute: i32,
}


impl FromStr for Time {
    type Err = AppError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(':').collect();

        if let [hour, minute] = parts.as_slice() {
            let hour = parse_i32(hour)?; 
            let minute = parse_i32(minute)?;

            if (0..24).contains(&hour) && (0..60).contains(&minute) {
                return Ok(Time {
                    hour,
                    minute,
                });
            }
        }

        Err(AppError::InvalidTime)
    }
}

fn parse_i32(s: &str) -> Result<i32, AppError> {
    match s.parse::<i32>() {
        Ok(integer) => Ok(integer),
        Err(_) => Err(AppError::InvalidTime),
    }
}


impl Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.hour, self.minute)
    }
}
use std::{cmp::Ordering, fmt::Display, str::FromStr};

use crate::{error::AppError, RANGE_VALID_HOURS, RANGE_VALID_MINUTES};

#[derive(Debug, Clone, Default)]
pub struct Time {
    hour: i32,
    minute: i32,
}

impl Time {
    pub fn get_hour(&self) -> i32 {
        self.hour
    }

    pub fn get_minute(&self) -> i32 {
        self.minute
    }

    pub fn cmp(&self, other: &Time) -> Ordering {
        match self.hour.cmp(&other.hour) {
            std::cmp::Ordering::Equal => self.minute.cmp(&other.minute),
            ord => ord,
        }
    }
}

impl FromStr for Time {
    type Err = AppError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(':').collect();

        if let [hour, minute] = parts.as_slice() {
            let hour = parse_i32(hour)?; 
            let minute = parse_i32(minute)?;

            if RANGE_VALID_HOURS.contains(&hour) && RANGE_VALID_MINUTES.contains(&minute) {
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
    s.parse::<i32>().map_err(|_| AppError::InvalidTime)
}

impl Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:0>2}:{:0>2}", self.hour, self.minute)
    }
}
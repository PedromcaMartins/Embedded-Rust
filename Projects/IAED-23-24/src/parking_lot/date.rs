use std::{fmt::Display, str::FromStr};

use crate::error::AppError;

#[derive(Debug)]
pub struct Date {
    day: i32,
    month: i32,
    year: i32,
}
///                                                 inv jan feb mar apr may jun jul aug sept oct nov dec
const LOOKUP_TABLE_LAST_DAY_GIVEN_MONTH: [i32; 13] = [0, 31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

impl FromStr for Date {
    type Err = AppError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('-').collect();

        if let [day, month, year] = parts.as_slice() {
            let day = parse_i32(day)?; 
            let month = parse_i32(month)?;
            let year = parse_i32(year)?;

            if (1..12+1).contains(&month) 
            && (1..LOOKUP_TABLE_LAST_DAY_GIVEN_MONTH[month as usize]+1).contains(&day) 
            && (2_000..2_100).contains(&year) {
                return Ok(Date {
                    day,
                    month,
                    year,
                });
            }
        }

        Err(AppError::InvalidDate)
    }
}

fn parse_i32(s: &str) -> Result<i32, AppError> {
    match s.parse::<i32>() {
        Ok(integer) => Ok(integer),
        Err(_) => Err(AppError::InvalidTime),
    }
}


impl Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}-{}", self.day, self.month, self.year)
    }
}
use std::{cmp::Ordering, fmt::Display, str::FromStr};

use crate::{error::AppError, LOOKUP_TABLE_LAST_DAY_GIVEN_MONTH, RANGE_VALID_MONTHS, RANGE_VALID_YEARS};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Date {
    day: i32,
    month: i32,
    year: i32,
}

impl Date {
    pub fn get_day(&self) -> i32 {
        self.day
    }
    
    pub fn get_month(&self) -> i32 {
        self.month
    }
    
    pub fn get_year(&self) -> i32 {
        self.year
    }

    pub fn cmp(&self, other: &Date) -> Ordering {
        match self.year.cmp(&other.year) {
            std::cmp::Ordering::Equal => {
                match self.month.cmp(&other.month) {
                    std::cmp::Ordering::Equal => self.day.cmp(&other.day),
                    ord => ord,
                }
            },
            ord => ord,
        }
    }
}

impl Default for Date {
    fn default() -> Self {
        Self {
            day: 1,
            month: 1,
            year: RANGE_VALID_YEARS.start
        }
    }
}

impl FromStr for Date {
    type Err = AppError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('-').collect();

        if let [day, month, year] = parts.as_slice() {
            let day = parse_i32(day)?; 
            let month = parse_i32(month)?;
            let year = parse_i32(year)?;

            if RANGE_VALID_MONTHS.contains(&month) 
            && (1..get_num_days_for_mounth(month)).contains(&day) 
            && RANGE_VALID_YEARS.contains(&year) {
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
    s.parse::<i32>().map_err(|_| AppError::InvalidDate)
}

impl Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:0>2}-{:0>2}-{:0>4}", self.day, self.month, self.year)
    }
}

pub fn get_num_days_for_mounth(month: i32) -> i32 {
    LOOKUP_TABLE_LAST_DAY_GIVEN_MONTH[month as usize]
}
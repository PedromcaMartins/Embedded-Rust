use std::{fmt::Display, str::FromStr};

use crate::error::AppError;

#[derive(Debug)]
pub struct LicencePlate {
    plate: String,
}

impl FromStr for LicencePlate {
    type Err = AppError;

    /// A license plate corresponds to a sequence of 3 pairs of characters separated by 
    /// the character _'-'_. A pair can only contain uppercase letters from _'A'_ to _'Z'_ 
    /// or can only contain decimal digits. A pair cannot be composed of both a letter 
    /// and a digit. A license plate must always contain at least one pair of letters 
    /// and at least one pair of decimal digits.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('-').collect();

        if parts.len() == 3 && s.split('-').all(|part| part.len() == 2) {
            let count_alphabetic = parts
                .iter()
                .filter(|part| part.chars().all(|c| c.is_alphabetic() && c.is_uppercase()))
                .count();

            let count_numeral = parts
                .iter()
                .filter(|part| part.chars().all(|c| c.is_numeric()))
                .count();

            let total_valid_parts = count_alphabetic + count_numeral;

            if count_alphabetic != 0 && count_numeral != 0 && total_valid_parts == 3 {
                return Ok(LicencePlate {
                    plate: s.to_string(),
                });
            }
        }

        Err(AppError::InvalidLicencePlate)
    }
}


impl Display for LicencePlate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.plate)
    }
}
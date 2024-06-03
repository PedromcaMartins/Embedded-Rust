use std::{fmt::Display, str::FromStr};

use crate::error::AppError;

#[derive(Debug)]
pub struct Tariff {
    value: f32,
}


impl FromStr for Tariff {
    type Err = AppError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<f32>() {
            Ok(value) => Ok(Tariff { value }),
            Err(_) => Err(AppError::ParserTariff),
        }
    }
}


impl Display for Tariff {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.2}", self.value)
    }
}
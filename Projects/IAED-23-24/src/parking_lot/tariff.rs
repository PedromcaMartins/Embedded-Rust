use std::{fmt::Display, str::FromStr};

use crate::error::AppError;

#[derive(Debug, Default, Clone)]
pub struct Tariff {
    value: f32,
}

impl Tariff {
    pub fn new(value: f32) -> Self {
        Self { value }
    }

    pub fn get_value(&self) -> f32 {
        self.value
    }

    pub fn add_bill(&mut self, bill: &Tariff) {
        self.value += bill.get_value()
    }
}

impl FromStr for Tariff {
    type Err = AppError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Tariff {
            value: s.parse::<f32>().map_err(|_| AppError::ParserTariff)?,
        })
    }
}

impl Display for Tariff {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.2}", self.value)
    }
}
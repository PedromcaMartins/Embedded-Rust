use std::fmt::Display;

use crate::parking_lot::LicencePlate;
use std::fmt;

#[derive(Debug)]
pub enum AppError {
    InvalidCommand,
    ParserInt,
    ParserTariff,
    InvalidArgsCommandF,
    InvalidLicencePlate { licence_plate: String },
    InvalidTime,
    InvalidDate,
    ParkingNameAlreadyExists { park_name: String },
    InvalidCapacity { value: i32 },
    InvalidCost,
    TooManyParks,
    NoSuchParking { park_name: String },
    ParkIsFull { park_name: String },
    InvalidVehicleEntry { licence_plate: LicencePlate },
    InvalidVehicleExit { licence_plate: LicencePlate },
    NoEntriesFoundInAnyParking { licence_plate: LicencePlate },
}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::InvalidCommand => write!(f, "invalid command."),
            AppError::ParserInt => write!(f, "error parsing integer."),
            AppError::ParserTariff => write!(f, "error parsing tariff."),
            AppError::InvalidArgsCommandF => write!(f, "invalid arguments for command F."),
            AppError::InvalidLicencePlate { licence_plate } => write!(f, "{}: invalid licence plate.", licence_plate),
            AppError::InvalidTime => write!(f, "invalid time."),
            AppError::InvalidDate => write!(f, "invalid date."),
            AppError::ParkingNameAlreadyExists { park_name } => write!(f, "{}: already exists.", park_name),
            AppError::InvalidCapacity { value } => write!(f, "{}: invalid capacity.", value),
            AppError::InvalidCost => write!(f, "invalid cost."),
            AppError::TooManyParks => write!(f, "too many parks."),
            AppError::NoSuchParking { park_name } => write!(f, "{}: no such parking.", park_name),
            AppError::ParkIsFull { park_name } => write!(f, "{} parking is full.", park_name),
            AppError::InvalidVehicleEntry { licence_plate } => write!(f, "{}: invalid vehicle entry.", licence_plate),
            AppError::InvalidVehicleExit { licence_plate } => write!(f, "{}: invalid vehicle exit.", licence_plate),
            AppError::NoEntriesFoundInAnyParking { licence_plate } => write!(f, "{}: no entries found in any parking.", licence_plate),
        }
    }
}
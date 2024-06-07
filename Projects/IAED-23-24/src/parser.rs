use std::str::FromStr;

use crate::{error::AppError, parking_lot::{Date, LicencePlate, Tariff, Time}};

#[derive(Debug)]
pub enum Command {
    /// Input format: `q`
    Quit,
    /// Input format: `p <park-name> <capacity> <tariff-x> <tariff-y> <tariff-z>`
    CreateParkingLot { 
        park_name: String, 
        capacity: i32,
        tariff_x: Tariff,
        tariff_y: Tariff,
        tariff_z: Tariff,
    },
    /// Input format: `p`
    ListParkingLots,
    /// Input format: `e <park-name> <licence-plate> <date> <time>`
    RegisterVehicleEntry {
        park_name: String,
        licence_plate: LicencePlate,
        date: Date,
        time: Time,
    },
    /// Input format: `s <park-name> <licence-plate> <date> <time>`
    RegisterVehicleExit {
        park_name: String,
        licence_plate: LicencePlate,
        date: Date,
        time: Time,
    },
    /// Input format: `v <licence-plate>`
    ListVehicleLogEntriesExits { licence_plate: LicencePlate },
    /// Input format: `f <park-name>`
    ShowParkingLotBilling { park_name: String },
    /// Input format: `f <park-name> <date>`
    ShowParkingLotBillingSpecificDate { park_name: String, date: Date },
    /// Input format: `r <park-name>`
    RemoveParkingLot { park_name: String }
}

impl FromStr for Command {
    type Err = AppError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        let parts: Vec<&str> = s.split_whitespace().collect();

        match parts.first() {
            Some(&"q") if parts.len() == 1 => Ok(Command::Quit),
            Some(&"p") if parts.len() >= 6 => Ok(Command::CreateParkingLot { 
                park_name: parts[1..parts.len()-4].join(" "), 
                capacity: parts[parts.len()-4].parse::<i32>().map_err(|_| AppError::ParserInt)?, 
                tariff_x: parts[parts.len()-3].parse::<Tariff>()?, 
                tariff_y: parts[parts.len()-2].parse::<Tariff>()?, 
                tariff_z: parts[parts.len()-1].parse::<Tariff>()?, 
            }),
            Some(&"p") if parts.len() == 1 => Ok(Command::ListParkingLots),
            Some(&"e") if parts.len() >= 5 => Ok(Command::RegisterVehicleEntry { 
                park_name: parts[1..parts.len()-3].join(" "), 
                licence_plate: parts[parts.len()-3].parse::<LicencePlate>()?, 
                date: parts[parts.len()-2].parse::<Date>()?, 
                time: parts[parts.len()-1].parse::<Time>()?, 
            }),
            Some(&"s") if parts.len() >= 5 => Ok(Command::RegisterVehicleExit { 
                park_name: parts[1..parts.len()-3].join(" "), 
                licence_plate: parts[parts.len()-3].parse::<LicencePlate>()?, 
                date: parts[parts.len()-2].parse::<Date>()?, 
                time: parts[parts.len()-1].parse::<Time>()?, 
            }),
            Some(&"v") if parts.len() == 2 => Ok(Command::ListVehicleLogEntriesExits { 
                licence_plate: parts[1].parse::<LicencePlate>()?, 
            }),
            Some(&"f") => parse_args_command_f(&s[2..]),
            Some(&"r") if parts.len() >= 2 => Ok(Command::RemoveParkingLot { 
                park_name: parts[1..].join(" "), 
            }),
            Some(_) => Err(AppError::InvalidCommand),
            None => Err(AppError::InvalidCommand),
        }
    }
}

fn parse_args_command_f(s: &str) -> Result<Command, AppError> {
    let mut parts = s.splitn(3, '"');

    let first_part = parts.next();
    let park_name_date = parts.next();
    let extra_part = parts.next();

    match (first_part, park_name_date, extra_part) {
        (_, Some(park_name), Some("")) => Ok(Command::ShowParkingLotBilling { 
            park_name: park_name.to_string() 
        }),
        (_, Some(park_name), Some(date)) => Ok(Command::ShowParkingLotBillingSpecificDate { 
            park_name: park_name.to_string(), 
            date: date.parse::<Date>()?, 
        }),
        _ => {
            let parts: Vec<&str> = s.split_whitespace().collect();
            match parts.as_slice() {
                [park_name] => Ok(Command::ShowParkingLotBilling { 
                    park_name: park_name.to_string(), 
                }),
                [park_name, date] => Ok(Command::ShowParkingLotBillingSpecificDate { 
                    park_name: park_name.to_string(), 
                    date: date.parse::<Date>()?, 
                }),
                _ => Err(AppError::InvalidArgsCommandF),
            }
        }
    }
}
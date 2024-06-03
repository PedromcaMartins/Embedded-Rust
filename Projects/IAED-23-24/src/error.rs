use std::num::ParseIntError;


#[derive(Debug)]
pub enum AppError {
    InvalidCommand,
    ParserInt,
    ParserTariff,
    InvalidArgsCommandF,
    InvalidLicencePlate,
    InvalidTime,
    InvalidDate,
}


impl From<ParseIntError> for AppError {
    fn from(_value: ParseIntError) -> Self {
        AppError::ParserInt
    }
}
use std::{fmt, io};

use super::{ParserErrorType, VerifierErrorType};

#[derive(Debug, PartialEq)]
pub enum ErrorKind {
    ParserErrorType,
    VerifierErrorType,
    QuitCommand,
}

#[derive(Debug, PartialEq)]
pub struct AppError {
    pub error_kind: ErrorKind,
    message: String
}

impl AppError {
    pub fn new(error_kind: ErrorKind) -> AppError {
        AppError { error_kind, message: String::new() }
    }
}

impl From<ParserErrorType> for AppError {
    fn from(value: ParserErrorType) -> Self {
        AppError { error_kind: ErrorKind::ParserErrorType, message: value.to_string() }
    }
}

impl From<VerifierErrorType> for AppError {
    fn from(value: VerifierErrorType) -> Self {
        AppError { error_kind: ErrorKind::VerifierErrorType, message: value.to_string() }
    }
}

impl From<io::Error> for AppError {
    fn from(value: io::Error) -> Self {
        AppError { error_kind: ErrorKind::ParserErrorType, message: ParserErrorType::IoError { message: value.to_string() }.to_string() }
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"{}: {}",
            match self.error_kind {
                ErrorKind::ParserErrorType => "Error parsing command",
                ErrorKind::VerifierErrorType => "Error verifying arguments",
                ErrorKind::QuitCommand => "Quit command executed. Quitting...",
            },
            self.message
        )
    }
}
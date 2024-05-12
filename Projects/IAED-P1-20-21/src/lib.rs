// Data modules
mod command;
mod error;

// Functionality modules
mod parser;
mod verifier;

pub use error::{ParserErrorType, VerifierErrorType, ErrorKind, AppError};

pub fn parser_argument_verifier_and_executer() -> Result<(), AppError> {
    parser::parser()
}
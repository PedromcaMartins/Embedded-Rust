use super::{Command, ParserErrorType, VerifierErrorType};

mod parser;
mod verifier;
mod executer;

/// Advance Time
#[derive(Default, Debug, PartialEq)]
pub struct NCommand {
    duration: i32,
}

impl Command for NCommand {
    fn parse_arguments(args: &str) -> Result<NCommand, ParserErrorType> {
        parser::parse_arguments(args)
    }

    fn verify_parameters(&self) -> Result<(), VerifierErrorType> {
        verifier::verify_parameters(&self)
    }

    fn execute(self) {
        executer::execute(self)
    }
}
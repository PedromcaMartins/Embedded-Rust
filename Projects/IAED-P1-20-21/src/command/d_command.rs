use super::{Command, ParserErrorType, VerifierErrorType};

mod parser;
mod verifier;
mod executer;

/// List All Tasks In Activity
#[derive(Default, Debug, PartialEq)]
pub struct DCommand {
    activity: String,
}

impl Command for DCommand {
    fn parse_arguments(args: &str) -> Result<DCommand, ParserErrorType> {
        parser::parse_arguments(args)
    }

    fn verify_parameters(&self) -> Result<(), VerifierErrorType> {
        verifier::verify_parameters(&self)
    }

    fn execute(self) {
        executer::execute(self)
    }
}
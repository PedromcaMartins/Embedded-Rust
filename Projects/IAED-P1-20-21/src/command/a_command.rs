use super::{Command, ParserErrorType, VerifierErrorType};

mod parser;
mod verifier;
mod executer;

/// Add Activity or List All Activities
#[derive(Default, Debug, PartialEq)]
pub struct ACommand {
    activity: Option<String>,
}

impl Command for ACommand {
    fn parse_arguments(args: &str) -> Result<ACommand, ParserErrorType> {
        parser::parse_arguments(args)
    }

    fn verify_parameters(&self) -> Result<(), VerifierErrorType> {
        verifier::verify_parameters(&self)
    }

    fn execute(self) {
        executer::execute(self)
    }
}
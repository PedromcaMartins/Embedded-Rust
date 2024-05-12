use super::{Command, ParserErrorType, VerifierErrorType};

mod parser;
mod verifier;
mod executer;

/// Add User or List All Users
#[derive(Default, Debug, PartialEq)]
pub struct UCommand {
    username: Option<String>,
}

impl Command for UCommand {
    fn parse_arguments(args: &str) -> Result<UCommand, ParserErrorType> {
        parser::parse_arguments(args)
    }

    fn verify_parameters(&self) -> Result<(), VerifierErrorType> {
        verifier::verify_parameters(&self)
    }

    fn execute(self) {
        executer::execute(self)
    }
}
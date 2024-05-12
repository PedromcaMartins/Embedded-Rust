use super::{Command, ParserErrorType, VerifierErrorType};

mod parser;
mod verifier;
mod executer;

/// Add New Task
#[derive(Default, Debug, PartialEq)]
pub struct TCommand {
    duration: i32, 
    description: String, 
}

impl Command for TCommand {
    fn parse_arguments(args: &str) -> Result<TCommand, ParserErrorType> {
        parser::parse_arguments(args)
    }

    fn verify_parameters(&self) -> Result<(), VerifierErrorType> {
        verifier::verify_parameters(&self)
    }

    fn execute(self) {
        executer::execute(self)
    }
}
use super::{Command, ParserErrorType, VerifierErrorType};

mod parser;
mod verifier;
mod executer;

/// Move Task To Activity
#[derive(Default, Debug, PartialEq)]
pub struct MCommand {
    task_id: i32, 
    username: String, 
    activity: String,
}

impl Command for MCommand {
    fn parse_arguments(args: &str) -> Result<MCommand, ParserErrorType> {
        parser::parse_arguments(args)
    }

    fn verify_parameters(&self) -> Result<(), VerifierErrorType> {
        verifier::verify_parameters(&self)
    }

    fn execute(self) {
        executer::execute(self)
    }
}
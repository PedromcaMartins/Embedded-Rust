use super::{Command, ParserErrorType, VerifierErrorType};

mod parser;
mod verifier;
mod executer;

/// List All or Specific Tasks
#[derive(Default, Debug, PartialEq)]
pub struct LCommand {
    task_ids: Option<Vec<i32>>,
}

impl Command for LCommand {
    fn parse_arguments(args: &str) -> Result<LCommand, ParserErrorType> {
        parser::parse_arguments(args)
    }

    fn verify_parameters(&self) -> Result<(), VerifierErrorType> {
        verifier::verify_parameters(&self)
    }

    fn execute(self) {
        executer::execute(self)
    }
}
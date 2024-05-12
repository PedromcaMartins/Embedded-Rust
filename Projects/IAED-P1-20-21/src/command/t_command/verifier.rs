use super::{TCommand, VerifierErrorType};
use crate::arguments::{description, duration};

pub fn verify_parameters(command: &TCommand) -> Result<(), VerifierErrorType> {
    duration::verify(&command.duration)?; 
    description::verify(&command.description)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_parameters_valid() {
        let command_t = TCommand { duration: 60, description: String::from("Sample task") };

        assert_eq!(verify_parameters(&command_t), Ok(()));
    }
}
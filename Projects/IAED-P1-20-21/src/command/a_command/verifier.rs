use super::{ACommand, VerifierErrorType};
use crate::verifier::activity_op;

pub fn verify_parameters(command: &ACommand) -> Result<(), VerifierErrorType> {
    activity_op::verify(&command.activity)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_parameters_valid() {
        let command_a_1 = ACommand { activity: None };
        let command_a_2 = ACommand { activity: Some(String::from("TO-DO")) };

        assert_eq!(verify_parameters(&command_a_1), Ok(()));
        assert_eq!(verify_parameters(&command_a_2), Ok(()));
    }
}
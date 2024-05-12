use super::{UCommand, VerifierErrorType};
use crate::arguments::username_op;

pub fn verify_parameters(command: &UCommand) -> Result<(), VerifierErrorType> {
    username_op::verify(&command.username)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_parameters_valid() {
        let command_u_1 = UCommand { username: None };
        let command_u_2 = UCommand { username: Some(String::from("john_doe")) };

        assert_eq!(verify_parameters(&command_u_1), Ok(()));
        assert_eq!(verify_parameters(&command_u_2), Ok(()));
    }
}
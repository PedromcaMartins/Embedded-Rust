use super::{DCommand, VerifierErrorType};
use crate::arguments::activity;

pub fn verify_parameters(command: &DCommand) -> Result<(), VerifierErrorType> {
    activity::verify(&command.activity)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_parameters_valid() {
        let command_d = DCommand { activity: String::from("TO-DO") };

        assert_eq!(verify_parameters(&command_d), Ok(()));
    }
}
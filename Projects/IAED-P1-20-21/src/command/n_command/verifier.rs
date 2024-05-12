use super::{NCommand, VerifierErrorType};
use crate::arguments::duration;

pub fn verify_parameters(command: &NCommand) -> Result<(), VerifierErrorType> {
    duration::verify(&command.duration)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_parameters_valid() {
        let command_n = NCommand { duration: 120, };

        assert_eq!(verify_parameters(&command_n), Ok(()));
    }
}
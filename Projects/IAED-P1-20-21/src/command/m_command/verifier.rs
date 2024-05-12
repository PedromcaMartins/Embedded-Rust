use super::{MCommand, VerifierErrorType};
use crate::verifier::{activity, task_id, username};

pub fn verify_parameters(command: &MCommand) -> Result<(), VerifierErrorType> {
    task_id::verify(&command.task_id)?;
    username::verify(&command.username)?;
    activity::verify(&command.activity)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_parameters_valid() {
        let command_m = MCommand { task_id: 1, username: String::from("john_doe"), activity: String::from("TO-DO") };

        assert_eq!(verify_parameters(&command_m), Ok(()));
    }
}
use super::{LCommand, VerifierErrorType};
use crate::arguments::task_ids;

pub fn verify_parameters(command: &LCommand) -> Result<(), VerifierErrorType> {
    task_ids::verify(&command.task_ids)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_parameters_valid() {
        let command_l_1 = LCommand { task_ids: None };
        let command_l_2 = LCommand { task_ids: Some(vec![1, 2, 3]) };

        assert_eq!(verify_parameters(&command_l_1), Ok(()));
        assert_eq!(verify_parameters(&command_l_2), Ok(()));
    }
}
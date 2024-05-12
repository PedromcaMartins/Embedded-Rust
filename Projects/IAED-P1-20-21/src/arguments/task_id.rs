use super::VerifierErrorType;

use super::MIN_VALUE_TASK_ID;
use super::MAX_VALUE_TASK_ID;

pub fn verify(task_id: &i32) -> Result<(), VerifierErrorType> {
    if (*task_id < MIN_VALUE_TASK_ID) || (*task_id > MAX_VALUE_TASK_ID) {
        return Err(VerifierErrorType::TaskIdOutsideValidRange);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_valid_input() {
        assert_eq!(verify(&MIN_VALUE_TASK_ID), Ok(()));
        assert_eq!(verify(&MAX_VALUE_TASK_ID), Ok(()));
        assert_eq!(verify(&(MIN_VALUE_TASK_ID + 1)), Ok(()));
        assert_eq!(verify(&(MAX_VALUE_TASK_ID - 1)), Ok(()));
    }

    #[test]
    fn test_verify_invalid_input() {
        assert_eq!(verify(&(MIN_VALUE_TASK_ID - 1)), Err(VerifierErrorType::TaskIdOutsideValidRange));
        assert_eq!(verify(&(MAX_VALUE_TASK_ID + 1)), Err(VerifierErrorType::TaskIdOutsideValidRange));
    }
}
use super::{VerifierErrorType, task_id};


pub fn verify(task_ids: &Option<Vec<i32>>) -> Result<(), VerifierErrorType> {
    if let Some(task_ids) = task_ids {
        if task_ids.is_empty() {
            panic!("Vector task-ids should be None, instead of empty");
        }

        task_ids
            .iter()
            .try_for_each(task_id::verify)
            ?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "should be None, instead of empty")]
    fn test_verify_panic() {
        let _ = verify(&Some(vec![]));
    }

    #[test]
    fn test_verify_valid_input() {
        assert_eq!(verify(&None), Ok(()));
        assert_eq!(verify(&Some(vec![task_id::MIN_VALUE_TASK_ID])), Ok(()));
        assert_eq!(verify(&Some(vec![task_id::MAX_VALUE_TASK_ID])), Ok(()));
        assert_eq!(verify(&Some(vec![task_id::MIN_VALUE_TASK_ID + 1])), Ok(()));
        assert_eq!(verify(&Some(vec![task_id::MAX_VALUE_TASK_ID - 1])), Ok(()));
    }

    #[test]
    fn test_verify_invalid_input() {
        assert_eq!(verify(&Some(vec![task_id::MIN_VALUE_TASK_ID - 1])), Err(VerifierErrorType::TaskIdOutsideValidRange));
        assert_eq!(verify(&Some(vec![task_id::MAX_VALUE_TASK_ID + 1])), Err(VerifierErrorType::TaskIdOutsideValidRange));
    }
}
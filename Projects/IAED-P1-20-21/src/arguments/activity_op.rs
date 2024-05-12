use super::{VerifierErrorType, activity};

pub fn verify(activity: &Option<String>) -> Result<(), VerifierErrorType> {
    if let Some(activity) = activity {
        activity::verify(activity)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::arguments::MAX_LEN_ACTIVITY;

    use super::*;

    #[test]
    fn test_verify_valid_input() {
        assert_eq!(verify(&None), Ok(()));
        assert_eq!(verify(&Some(String::from("ACTIVITY"))), Ok(()));
    }

    #[test]
    fn test_verify_invalid_input() {
        assert_eq!(verify(&Some(String::from(""))), Err(VerifierErrorType::EmptyActivity));
        assert_eq!(verify(&Some(String::from("ACTIvity"))), Err(VerifierErrorType::LowerCaseCharactersInActivity));
        assert_eq!(verify(&Some("A".repeat(MAX_LEN_ACTIVITY + 1))), Err(VerifierErrorType::ExceedsMaxLenActivity));
    }
}
use super::VerifierErrorType;

use super::MAX_LEN_ACTIVITY;

pub fn verify(activity: &str) -> Result<(), VerifierErrorType> {
    if activity.is_empty() {
        return Err(VerifierErrorType::EmptyActivity);
    }
    if activity.len() > MAX_LEN_ACTIVITY {
        return Err(VerifierErrorType::ExceedsMaxLenActivity);
    }
    if activity != activity.to_uppercase() {
        return Err(VerifierErrorType::LowerCaseCharactersInActivity)
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_valid_input() {
        assert_eq!(verify("ACTIVITY"), Ok(()));
        assert_eq!(verify("A C"), Ok(()));
        assert_eq!(verify(&"A".repeat(MAX_LEN_ACTIVITY)), Ok(()));
        assert_eq!(verify(&"A".repeat(MAX_LEN_ACTIVITY - 1)), Ok(()));
    }

    #[test]
    fn test_verify_invalid_input() {
        assert_eq!(verify(""), Err(VerifierErrorType::EmptyActivity));
        assert_eq!(verify("ACTIvity"), Err(VerifierErrorType::LowerCaseCharactersInActivity));
        assert_eq!(verify(&"A".repeat(MAX_LEN_ACTIVITY + 1)), Err(VerifierErrorType::ExceedsMaxLenActivity));
    }
}
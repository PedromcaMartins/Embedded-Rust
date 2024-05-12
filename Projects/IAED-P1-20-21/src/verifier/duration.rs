use super::VerifierErrorType;

pub fn verify(duration: &i32) -> Result<(), VerifierErrorType> {
    match *duration < 0 {
        true => Err(VerifierErrorType::NonPositiveDuration),
        false => Ok(()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_valid_input() {
        assert_eq!(verify(&0), Ok(()));
        assert_eq!(verify(&10), Ok(()));
    }

    #[test]
    fn test_verify_invalid_input() {
        assert_eq!(verify(&-10), Err(VerifierErrorType::NonPositiveDuration));
    }
}
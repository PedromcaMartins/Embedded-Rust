use super::{username, VerifierErrorType};

pub fn verify(username: &Option<String>) -> Result<(), VerifierErrorType> {
    if let Some(username) = username {
        username::verify(username)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_valid_input() {
        assert_eq!(verify(&None), Ok(()));
        assert_eq!(verify(&Some(String::from("username"))), Ok(()));
    }

    #[test]
    fn test_verify_invalid_input() {
        assert_eq!(verify(&Some(String::from(""))), Err(VerifierErrorType::EmptyUsername));
        assert_eq!(verify(&Some(String::from("u s"))), Err(VerifierErrorType::WhiteSpacesInUsername));
        assert_eq!(verify(&Some("u".repeat(username::MAX_LEN_USERNAME + 1))), Err(VerifierErrorType::ExceedsMaxLenUsername));
    }
}
use super::VerifierErrorType;

pub static MAX_LEN_USERNAME: usize = 20;

pub fn verify(username: &str) -> Result<(), VerifierErrorType> {
    if username.is_empty() {
        return Err(VerifierErrorType::EmptyUsername);
    }
    if username.len() > MAX_LEN_USERNAME {
        return Err(VerifierErrorType::ExceedsMaxLenUsername);
    }
    if username.split_whitespace().count() != 1 {
        return Err(VerifierErrorType::WhiteSpacesInUsername)
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_valid_input() {
        assert_eq!(verify("username"), Ok(()));
        assert_eq!(verify(&"u".repeat(MAX_LEN_USERNAME)), Ok(()));
        assert_eq!(verify(&"u".repeat(MAX_LEN_USERNAME - 1)), Ok(()));
    }

    #[test]
    fn test_verify_invalid_input() {
        assert_eq!(verify(""), Err(VerifierErrorType::EmptyUsername));
        assert_eq!(verify("u s"), Err(VerifierErrorType::WhiteSpacesInUsername));
        assert_eq!(verify(&"u".repeat(MAX_LEN_USERNAME + 1)), Err(VerifierErrorType::ExceedsMaxLenUsername));
    }
}
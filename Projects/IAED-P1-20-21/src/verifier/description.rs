use super::VerifierErrorType;

pub static MAX_LEN_DESCRIPTION: usize = 50;

pub fn verify(description: &str) -> Result<(), VerifierErrorType> {
    if description.is_empty() {
        return Err(VerifierErrorType::EmptyDescription);
    }
    if description.len() > MAX_LEN_DESCRIPTION {
        return Err(VerifierErrorType::ExceedsMaxLenDescription);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_valid_input() {
        assert_eq!(verify("description, etc..."), Ok(()));
        assert_eq!(verify(&"a".repeat(MAX_LEN_DESCRIPTION)), Ok(()));
        assert_eq!(verify(&"a".repeat(MAX_LEN_DESCRIPTION - 1)), Ok(()));
    }

    #[test]
    fn test_verify_invalid_input() {
        assert_eq!(verify(""), Err(VerifierErrorType::EmptyDescription));
        assert_eq!(verify(&"a".repeat(MAX_LEN_DESCRIPTION + 1)), Err(VerifierErrorType::ExceedsMaxLenDescription));
    }
}
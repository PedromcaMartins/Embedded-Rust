use std::fmt::Display;

use crate::arguments::MAX_LEN_ACTIVITY;
use crate::arguments::MAX_LEN_USERNAME;
use crate::arguments::MAX_LEN_DESCRIPTION;
use crate::arguments::MIN_VALUE_TASK_ID;
use crate::arguments::MAX_VALUE_TASK_ID;

#[derive(Debug, PartialEq)]
pub enum VerifierErrorType {
    NonPositiveDuration,
    EmptyDescription,
    ExceedsMaxLenDescription,
    TaskIdOutsideValidRange,
    EmptyUsername,
    ExceedsMaxLenUsername,
    WhiteSpacesInUsername,
    EmptyActivity,
    ExceedsMaxLenActivity,
    LowerCaseCharactersInActivity,
}

impl Display for VerifierErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VerifierErrorType::NonPositiveDuration => write!(
                f, "<duration> must be a positive integer"
            ),
            VerifierErrorType::EmptyDescription => write!(
                f, "<description> must not be empty"
            ),
            VerifierErrorType::ExceedsMaxLenDescription => write!(
                f, "<description> must not exceed {MAX_LEN_DESCRIPTION} characters"
            ),
            VerifierErrorType::TaskIdOutsideValidRange => write!(
                f, "<task-id> value must be within the range of [{MIN_VALUE_TASK_ID}, {MAX_VALUE_TASK_ID}], inclusive"
            ),
            VerifierErrorType::EmptyUsername => write!(
                f, "<username> must not be empty"
            ),
            VerifierErrorType::ExceedsMaxLenUsername => write!(
                f, "<username> must not exceed {MAX_LEN_USERNAME} characters"
            ),
            VerifierErrorType::WhiteSpacesInUsername => write!(
                f, "<username> must not contain whitespaces"
            ),
            VerifierErrorType::EmptyActivity => write!(
                f, "<activity> must not be empty"
            ),
            VerifierErrorType::ExceedsMaxLenActivity => write!(
                f, "<activity> must not exceed {MAX_LEN_ACTIVITY} characters"
            ),
            VerifierErrorType::LowerCaseCharactersInActivity => write!(
                f, "<activity> must not contain lower case characters"
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_non_positive_duration() {
        let error = VerifierErrorType::NonPositiveDuration;
        assert_eq!(error.to_string(), "<duration> must be a positive integer");
    }
    
    #[test]
    fn test_empty_description() {
        let error = VerifierErrorType::EmptyDescription;
        assert_eq!(error.to_string(), "<description> must not be empty");
    }
    
    #[test]
    fn test_exceeds_max_len_description() {
        let error = VerifierErrorType::ExceedsMaxLenDescription;
        assert_eq!(error.to_string(), format!("<description> must not exceed {MAX_LEN_DESCRIPTION} characters"));
    }
    
    #[test]
    fn test_task_id_outside_valid_range() {
        let error = VerifierErrorType::TaskIdOutsideValidRange;
        assert_eq!(
            error.to_string(),
            format!("<task-id> value must be within the range of [{MIN_VALUE_TASK_ID}, {MAX_VALUE_TASK_ID}], inclusive")
        );
    }
    
    #[test]
    fn test_empty_username() {
        let error = VerifierErrorType::EmptyUsername;
        assert_eq!(error.to_string(), "<username> must not be empty");
    }
    
    #[test]
    fn test_exceeds_max_len_username() {
        let error = VerifierErrorType::ExceedsMaxLenUsername;
        assert_eq!(error.to_string(), format!("<username> must not exceed {MAX_LEN_USERNAME} characters"));
    }
    
    #[test]
    fn test_white_spaces_in_username() {
        let error = VerifierErrorType::WhiteSpacesInUsername;
        assert_eq!(error.to_string(), "<username> must not contain whitespaces");
    }
    
    #[test]
    fn test_empty_activity() {
        let error = VerifierErrorType::EmptyActivity;
        assert_eq!(error.to_string(), "<activity> must not be empty");
    }
    
    #[test]
    fn test_exceeds_max_len_activity() {
        let error = VerifierErrorType::ExceedsMaxLenActivity;
        assert_eq!(error.to_string(), format!("<activity> must not exceed {MAX_LEN_ACTIVITY} characters"));
    }
    
    #[test]
    fn test_lower_case_characters_in_activity() {
        let error = VerifierErrorType::LowerCaseCharactersInActivity;
        assert_eq!(error.to_string(), "<activity> must not contain lower case characters");
    }
}
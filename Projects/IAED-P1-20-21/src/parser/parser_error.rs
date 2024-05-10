use std::fmt;
use std::io;

#[derive(Debug, PartialEq)]
pub enum ParserErrorType {
    ShouldNotHaveArgs{ expected_command: &'static str },
    TooManyArgs{ expected_command: &'static str },
    InvalidType{ argument: &'static str, expected_type: &'static str },
    MissingArgs{ arguments: &'static str, expected_command: &'static str },
    InvalidCommand,
    MissingCommand,
    IoError{ message: String },
}

impl fmt::Display for ParserErrorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParserErrorType::ShouldNotHaveArgs { expected_command } => write!(
                f, "Invalid args: '{}' should not have arguments", expected_command
            ),
            ParserErrorType::TooManyArgs { expected_command } => write!(
                f, "Invalid args: There should not be any more arguments after '{}'", expected_command
            ),
            ParserErrorType::InvalidType { argument, expected_type } => write!(
                f, "Invalid type: Expected {} to be {}", argument, expected_type
            ),
            ParserErrorType::MissingArgs { arguments, expected_command } => write!(
                f, "Missing args {}: Expected '{}'", arguments, expected_command
            ),
            ParserErrorType::InvalidCommand => write!(
                f, "Invalid command"
            ),
            ParserErrorType::MissingCommand => write!(
                f, "Missing command"
            ),
            ParserErrorType::IoError { message: _ } => write!(
                f, "Error reading line"
            )
        }
    }
}

impl From<io::Error> for ParserErrorType {
    fn from(value: io::Error) -> Self {
        ParserErrorType::IoError { message: value.to_string() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_should_not_have_args() {
        let error = ParserErrorType::ShouldNotHaveArgs { expected_command: "quit" };
        assert_eq!(error.to_string(),"Invalid args: 'quit' should not have arguments");
    }

    #[test]
    fn test_too_many_args() {
        let error = ParserErrorType::TooManyArgs { expected_command: "add" };
        assert_eq!(error.to_string(),"Invalid args: There should not be any more arguments after 'add'");
    }

    #[test]
    fn test_invalid_type() {
        let error = ParserErrorType::InvalidType {
            argument: "age",
            expected_type: "integer",
        };
        assert_eq!(error.to_string(),"Invalid type: Expected age to be integer");
    }

    #[test]
    fn test_missing_args() {
        let error = ParserErrorType::MissingArgs {
            arguments: "name, age",
            expected_command: "update",
        };
        assert_eq!(error.to_string(),"Missing args name, age: Expected 'update'");
    }

    #[test]
    fn test_invalid_command() {
        let error = ParserErrorType::InvalidCommand;
        assert_eq!(error.to_string(), "Invalid command");
    }

    #[test]
    fn test_missing_command() {
        let error = ParserErrorType::MissingCommand;
        assert_eq!(error.to_string(), "Missing command");
    }

    #[test]
    fn test_io_error() {
        let error = ParserErrorType::IoError {
            message: "Failed to read file".to_string(),
        };
        assert_eq!(error.to_string(), "Error reading line");
    }
}
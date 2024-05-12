use super::{parser_error::ParserErrorType, Command};

pub fn parse_arguments(args: &str) -> Result<Command, ParserErrorType> {
    match args.is_empty() {
        true => Ok(Command::Q),
        false => Err(ParserErrorType::ShouldNotHaveArgs { expected_command: "q" }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_arguments_valid_input() {
        assert_eq!(parse_arguments(""), Ok(Command::Q));
    }

    #[test]
    fn test_parse_arguments_invalid_input() {
        assert_eq!(parse_arguments("arguments"), Err(ParserErrorType::ShouldNotHaveArgs { expected_command: "q" }));
    }
}
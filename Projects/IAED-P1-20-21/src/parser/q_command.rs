use super::{parser_error::ParserErrorType, Command};

pub fn parse_command(args: &str) -> Result<Command, ParserErrorType> {
    match args.is_empty() {
        true => Ok(Command::Q),
        false => Err(ParserErrorType::ShouldNotHaveArgs { expected_command: "q" }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_command_valid_input() {
        assert_eq!(parse_command(""), Ok(Command::Q));
    }

    #[test]
    fn test_parse_command_invalid_input() {
        assert_eq!(parse_command("arguments"), Err(ParserErrorType::ShouldNotHaveArgs { expected_command: "q" }));
    }
}
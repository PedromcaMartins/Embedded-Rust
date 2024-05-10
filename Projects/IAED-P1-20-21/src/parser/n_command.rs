use super::{parser_error::ParserErrorType, Command};

pub fn parse_command(args: &str) -> Result<Command, ParserErrorType> {
    let mut args = args.split_whitespace();

    let duration = match args.next() {
        Some(arg) => match arg.parse::<i32>() {
            Ok(arg) => arg,
            Err(_) => return Err(ParserErrorType::InvalidType { argument: "<duration>", expected_type: "integer" }),
        },
        None => return Err(ParserErrorType::MissingArgs { arguments: "<duration>", expected_command: "n duration" }),
    };

    if args.next().is_some() {
        return Err(ParserErrorType::TooManyArgs { expected_command: "n <duration>" });
    }

    Ok(Command::N { 
        duration
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_command_valid_input() {
        assert_eq!(parse_command("10"), Ok(Command::N { duration: 10 }));
        assert_eq!(parse_command("100"), Ok(Command::N { duration: 100 }));
    }

    #[test]
    fn test_parse_command_invalid_input() {
        assert_eq!(parse_command(""), Err(ParserErrorType::MissingArgs { arguments: "<duration>", expected_command: "n duration" }));
        assert_eq!(parse_command("abc"), Err(ParserErrorType::InvalidType { argument: "<duration>", expected_type: "integer" }));
        assert_eq!(parse_command("10 extra"), Err(ParserErrorType::TooManyArgs { expected_command: "n <duration>" }));
    }
}

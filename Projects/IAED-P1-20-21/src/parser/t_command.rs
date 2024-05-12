use super::{parser_error::ParserErrorType, Command};

pub fn parse_arguments(args: &str) -> Result<Command, ParserErrorType> {
    let (duration, description) = match args.split_once(' ') {
        Some((duration, description)) => (duration, description),
        None => return Err(ParserErrorType::MissingArgs { arguments: "", expected_command: "t <duration> <description>" }),
    };

    let duration = match duration.parse::<i32>() {
        Ok(duration) => duration,
        Err(_) => return Err(ParserErrorType::InvalidType { argument: "<duration>", expected_type: "integer" }),
    };

    let description = description.trim().to_owned();

    Ok(Command::T{
        duration,
        description,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_arguments_valid_input() {
        assert_eq!(parse_arguments("10 finish rust project!"), Ok(Command::T { duration: 10, description: String::from("finish rust project!") }));
    }

    #[test]
    fn test_parse_arguments_invalid_input() {
        assert_eq!(parse_arguments(""), Err(ParserErrorType::MissingArgs { arguments: "", expected_command: "t <duration> <description>" }));
        assert_eq!(parse_arguments("10"), Err(ParserErrorType::MissingArgs { arguments: "", expected_command: "t <duration> <description>" }));
        assert_eq!(parse_arguments("finish rust project!"), Err(ParserErrorType::InvalidType { argument: "<duration>", expected_type: "integer" }));
        assert_eq!(parse_arguments("0xa4e finish rust project!"), Err(ParserErrorType::InvalidType { argument: "<duration>", expected_type: "integer" }));
    }
}
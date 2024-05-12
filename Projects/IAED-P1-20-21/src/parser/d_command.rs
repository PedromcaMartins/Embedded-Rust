use super::{parser_error::ParserErrorType, Command};

pub fn parse_arguments(args: &str) -> Result<Command, ParserErrorType> {
    let mut args = args.split_whitespace();

    let activity = match args.next().map(String::from) {
        Some(arg) => arg,
        None => return Err(ParserErrorType::MissingArgs { arguments: "<activity>", expected_command: "d <activity>" }),
    };

    if args.next().is_some() {
        return Err(ParserErrorType::TooManyArgs { expected_command: "d <activity>" });
    }

    Ok(Command::D {
        activity,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_arguments_valid_input() {
        assert_eq!(parse_arguments("activity"), Ok(Command::D { activity: String::from("activity") }));
        assert_eq!(parse_arguments("activity2"), Ok(Command::D { activity: String::from("activity2") }));
    }

    #[test]
    fn test_parse_arguments_invalid_input() {
        assert_eq!(parse_arguments(""), Err(ParserErrorType::MissingArgs { arguments: "<activity>", expected_command: "d <activity>" }));
        assert_eq!(parse_arguments("activity jiberish"), Err(ParserErrorType::TooManyArgs { expected_command: "d <activity>" }));
    }
}
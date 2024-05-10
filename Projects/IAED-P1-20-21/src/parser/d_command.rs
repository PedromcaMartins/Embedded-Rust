use super::{parser_error::ParserErrorType, Command};

pub fn parse_command(args: &str) -> Result<Command, ParserErrorType> {
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
    fn test_parse_command_valid_input() {
        assert_eq!(parse_command("activity"), Ok(Command::D { activity: String::from("activity") }));
        assert_eq!(parse_command("activity2"), Ok(Command::D { activity: String::from("activity2") }));
    }

    #[test]
    fn test_parse_command_invalid_input() {
        assert_eq!(parse_command(""), Err(ParserErrorType::MissingArgs { arguments: "<activity>", expected_command: "d <activity>" }));
        assert_eq!(parse_command("activity jiberish"), Err(ParserErrorType::TooManyArgs { expected_command: "d <activity>" }));
    }
}
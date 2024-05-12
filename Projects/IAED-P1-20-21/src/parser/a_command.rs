use super::{parser_error::ParserErrorType, Command};

pub fn parse_arguments(args: &str) -> Result<Command, ParserErrorType> {
    let mut args = args.split_whitespace();

    let activity = args.next().map(String::from);

    if activity.is_some() && args.next().is_some() {
        return Err(ParserErrorType::TooManyArgs { expected_command: "a <activity>" });
    }

    Ok(Command::A { 
        activity,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_arguments_valid_input() {
        assert_eq!(parse_arguments(""), Ok(Command::A { activity: None }));
        assert_eq!(parse_arguments("activity"), Ok(Command::A { activity: Some(String::from("activity")) }));
    }

    #[test]
    fn test_parse_arguments_invalid_input() {
        assert_eq!(parse_arguments("activity jiberish"), Err(ParserErrorType::TooManyArgs { expected_command: "a <activity>" }));
    }
}
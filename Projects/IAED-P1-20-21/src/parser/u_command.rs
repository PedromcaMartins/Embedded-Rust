use super::{parser_error::ParserErrorType, Command};

pub fn parse_command(args: &str) -> Result<Command, ParserErrorType> {
    let mut args = args.split_whitespace();

    let username = args.next().map(String::from);

    if username.is_some() && args.next().is_some() {
        return Err(ParserErrorType::TooManyArgs { expected_command: "u <username>" });
    }

    Ok(Command::U { 
        username,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_command_valid_input() {
        assert_eq!(parse_command(""), Ok(Command::U { username: None }));
        assert_eq!(parse_command("username"), Ok(Command::U { username: Some(String::from("username")) }));
    }

    #[test]
    fn test_parse_command_invalid_input() {
        assert_eq!(parse_command("username jiberish"), Err(ParserErrorType::TooManyArgs { expected_command: "u <username>" }));
    }
}
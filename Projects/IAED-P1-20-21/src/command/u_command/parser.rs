use super::{UCommand, ParserErrorType};

pub fn parse_arguments(args: &str) -> Result<UCommand, ParserErrorType> {
    let mut args = args.split_whitespace();

    let username = args.next().map(String::from);

    if username.is_some() && args.next().is_some() {
        return Err(ParserErrorType::TooManyArgs { expected_command: "u <username>" });
    }

    Ok(UCommand { 
        username,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_arguments_valid_input() {
        assert_eq!(parse_arguments(""), Ok(UCommand { username: None }));
        assert_eq!(parse_arguments("username"), Ok(UCommand { username: Some(String::from("username")) }));
    }

    #[test]
    fn test_parse_arguments_invalid_input() {
        assert_eq!(parse_arguments("username jiberish"), Err(ParserErrorType::TooManyArgs { expected_command: "u <username>" }));
    }
}
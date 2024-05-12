use super::{ACommand, ParserErrorType};

pub fn parse_arguments(args: &str) -> Result<ACommand, ParserErrorType> {
    let mut args = args.split_whitespace();

    let activity = args.next().map(String::from);

    if activity.is_some() && args.next().is_some() {
        return Err(ParserErrorType::TooManyArgs { expected_command: "a <activity>" });
    }

    Ok(ACommand { 
        activity,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_arguments_valid_input() {
        assert_eq!(parse_arguments(""), Ok(ACommand { activity: None }));
        assert_eq!(parse_arguments("activity"), Ok(ACommand { activity: Some(String::from("activity")) }));
    }

    #[test]
    fn test_parse_arguments_invalid_input() {
        assert_eq!(parse_arguments("activity jiberish"), Err(ParserErrorType::TooManyArgs { expected_command: "a <activity>" }));
    }
}
use super::{parser_error::ParserErrorType, Command};

pub fn parse_command(args: &str) -> Result<Command, ParserErrorType> {
    let mut args = args.split_whitespace();

    let task_id = match args.next() {
        Some(arg) => match arg.parse::<i32>() {
            Ok(arg) => arg,
            Err(_) => return Err(ParserErrorType::InvalidType { argument: "<task-id>", expected_type: "integer" }),
        },
        None => return Err(ParserErrorType::MissingArgs { arguments: "<task-id>", expected_command: "m <task-id> <username> <activity>" }),
    };

    let username = match args.next().map(String::from) {
        Some(arg) => arg,
        None => return Err(ParserErrorType::MissingArgs { arguments: "<username>", expected_command: "m <task-id> <username> <activity>" }),
    };

    let activity = match args.next().map(String::from) {
        Some(arg) => arg,
        None => return Err(ParserErrorType::MissingArgs { arguments: "<activity>", expected_command: "m <task-id> <username> <activity>" }),
    };

    if args.next().is_some() {
        return Err(ParserErrorType::TooManyArgs { expected_command: "m <task-id> <username> <activity>" });
    }

    Ok(Command::M { 
        task_id,
        username,
        activity,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_command_valid_input() {
        assert_eq!(parse_command("23 username activity"), Ok(Command::M { task_id: 23, username: String::from("username"), activity: String::from("activity") }));
        assert_eq!(parse_command("5234 username2 activity2"), Ok(Command::M { task_id: 5234, username: String::from("username2"), activity: String::from("activity2") }));
    }

    #[test]
    fn test_parse_command_invalid_input() {
        assert_eq!(parse_command(""), Err(ParserErrorType::MissingArgs { arguments: "<task-id>", expected_command: "m <task-id> <username> <activity>" }));
        assert_eq!(parse_command("23"), Err(ParserErrorType::MissingArgs { arguments: "<username>", expected_command: "m <task-id> <username> <activity>" }));
        assert_eq!(parse_command("23 username"), Err(ParserErrorType::MissingArgs { arguments: "<activity>", expected_command: "m <task-id> <username> <activity>" }));
        assert_eq!(parse_command("23 activity"), Err(ParserErrorType::MissingArgs { arguments: "<activity>", expected_command: "m <task-id> <username> <activity>" }));
        assert_eq!(parse_command("username activity"), Err(ParserErrorType::InvalidType { argument: "<task-id>", expected_type: "integer" }));
        assert_eq!(parse_command("twenty-three username activity"), Err(ParserErrorType::InvalidType { argument: "<task-id>", expected_type: "integer" }));
        assert_eq!(parse_command("23 username activity something something"), Err(ParserErrorType::TooManyArgs { expected_command: "m <task-id> <username> <activity>" }));
    }
}
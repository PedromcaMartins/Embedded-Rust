use super::Command;

pub fn parse_command(args: &str) -> Result<Command, &'static str> {
    let mut args = args.split_whitespace();

    let task_id = match args.next() {
        Some(arg) => match arg.parse::<i32>() {
            Ok(arg) => arg,
            Err(_) => return Err("Invalid args: Expected <task-id> to be integer"),
        },
        None => return Err("Missing arg <task-id>: Expected 'm <task-id> <username> <activity>'"),
    };

    let username = match args.next().map(String::from) {
        Some(arg) => arg,
        None => return Err("Missing arg <username>: Expected 'm <task-id> <username> <activity>'"),
    };

    let activity = match args.next().map(String::from) {
        Some(arg) => arg,
        None => return Err("Missing arg <activity>: Expected 'm <task-id> <username> <activity>'"),
    };

    if args.next().is_some() {
        return Err("Invalid args: There should not be any more arguments after 'n <duration>'");
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
        assert_eq!(parse_command(""), Err("Missing arg <task-id>: Expected 'm <task-id> <username> <activity>'"));
        assert_eq!(parse_command("23"), Err("Missing arg <username>: Expected 'm <task-id> <username> <activity>'"));
        assert_eq!(parse_command("23 username"), Err("Missing arg <activity>: Expected 'm <task-id> <username> <activity>'"));
        assert_eq!(parse_command("23 activity"), Err("Missing arg <activity>: Expected 'm <task-id> <username> <activity>'"));
        assert_eq!(parse_command("username activity"), Err("Invalid args: Expected <task-id> to be integer"));
        assert_eq!(parse_command("twenty-three username activity"), Err("Invalid args: Expected <task-id> to be integer"));
        assert_eq!(parse_command("23 username activity something something"), Err("Invalid args: There should not be any more arguments after 'n <duration>'"));
    }
}
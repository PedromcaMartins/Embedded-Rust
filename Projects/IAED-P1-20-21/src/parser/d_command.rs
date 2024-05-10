use super::Command;

pub fn parse_command(args: &str) -> Result<Command, &'static str> {
    let mut args = args.split_whitespace();

    let activity = match args.next().map(String::from) {
        Some(arg) => arg,
        None => return Err("Missing arg <activity>: Expected 'd <activity>'"),
    };

    if args.next().is_some() {
        return Err("Invalid args: There should not be any more arguments after 'd <activity>'");
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
        assert_eq!(parse_command(""), Err("Missing arg <activity>: Expected 'd <activity>'"));
        assert_eq!(parse_command("activity jiberish"), Err("Invalid args: There should not be any more arguments after 'd <activity>'"));
    }
}
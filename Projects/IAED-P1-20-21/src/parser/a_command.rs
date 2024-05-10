use super::Command;

pub fn parse_command(args: &str) -> Result<Command, &'static str> {
    let mut args = args.split_whitespace();

    let activity = args.next().map(String::from);

    if activity.is_some() && args.next().is_some() {
        return Err("Invalid args: There should not be any more arguments after 'a <activity>'");
    }

    Ok(Command::A { 
        activity,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_command_valid_input() {
        assert_eq!(parse_command(""), Ok(Command::A { activity: None }));
        assert_eq!(parse_command("activity"), Ok(Command::A { activity: Some(String::from("activity")) }));
    }

    #[test]
    fn test_parse_command_invalid_input() {
        assert_eq!(parse_command("activity jiberish"), Err("Invalid args: There should not be any more arguments after 'a <activity>'"));
    }
}
use super::Command;

pub fn parse_command(args: &str) -> Result<Command, &'static str> {
    let mut args = args.split_whitespace();

    let duration = match args.next() {
        Some(arg) => match arg.parse::<i32>() {
            Ok(arg) => arg,
            Err(_) => return Err("Invalid args: Expected <duration> to be integer"),
        },
        None => return Err("Missing args: Expected 'n <duration>'"),
    };

    if args.next().is_some() {
        return Err("Invalid args: There should not be any more arguments after 'n <duration>'");
    }

    Ok(Command::N { 
        duration
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_command_valid_input() {
        assert_eq!(parse_command("10"), Ok(Command::N { duration: 10 }));
        assert_eq!(parse_command("100"), Ok(Command::N { duration: 100 }));
    }

    #[test]
    fn test_parse_command_invalid_input() {
        assert_eq!(parse_command(""), Err("Missing args: Expected 'n <duration>'"));
        assert_eq!(parse_command("abc"), Err("Invalid args: Expected <duration> to be integer"));
        assert_eq!(parse_command("10 extra"), Err("Invalid args: there should not be any more arguments after 'n <duration>'"));
    }
}

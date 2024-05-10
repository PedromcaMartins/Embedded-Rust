use super::Command;

pub fn parse_command(args: &str) -> Result<Command, &'static str> {
    let (duration, description) = match args.split_once(' ') {
        Some((duration, description)) => (duration, description),
        None => return Err("Missing args: Expected 't <duration> <description>'"),
    };

    let duration = match duration.parse::<i32>() {
        Ok(duration) => duration,
        Err(_) => return Err("Invalid type: Expected <duration> to be integer"),
    };

    let description = description.trim().to_owned();

    Ok(Command::T{
        duration,
        description,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_command_valid_input() {
        assert_eq!(parse_command("10 finish rust project!"), Ok(Command::T { duration: 10, description: String::from("finish rust project!") }));
    }

    #[test]
    fn test_parse_command_invalid_input() {
        assert_eq!(parse_command(""), Err("Missing arguments: Expected 't <duration> <description>'"));
        assert_eq!(parse_command("10"),Err("Missing arguments: Expected 't <duration> <description>'"));
        assert_eq!(parse_command("finish rust project!"),Err("Invalid argument: Expected <duration> to be integer"));
        assert_eq!(parse_command("0xa4e finish rust project!"),Err("Invalid argument: Expected <duration> to be integer"));
    }
}
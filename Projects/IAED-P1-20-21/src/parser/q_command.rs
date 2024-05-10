use super::Command;

pub fn parse_command(args: &str) -> Result<Command, &'static str> {
    match args.is_empty() {
        true => Ok(Command::Q),
        false => Err("Invalid args: 'q' should not have arguments"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_command_valid_input() {
        assert_eq!(parse_command(""), Ok(Command::Q));
    }

    #[test]
    fn test_parse_command_invalid_input() {
        assert_eq!(parse_command("arguments"), Err("Invalid args: 'q' should not have arguments"));
    }
}
use super::Command;

pub fn parse_command(args: &str) -> Result<Command, &'static str> {
    match args.is_empty() {
        true => Ok(Command::Quit),
        false => Err("Invalid args: 'q' should not have arguments"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_q_command_no_arguments() {
        let args = "";

        let result = parse_command(args);

        assert_eq!(result, Ok(Command::Quit));
    }

    #[test]
    #[should_panic(expected = "'q' should not have arguments")]
    fn parse_q_command_with_arguments() {
        let args = "arguments";

        let result = parse_command(args);

        result.unwrap();
    }
}
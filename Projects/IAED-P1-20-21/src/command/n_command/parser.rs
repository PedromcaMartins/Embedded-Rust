use super::{NCommand, ParserErrorType};

pub fn parse_arguments(args: &str) -> Result<NCommand, ParserErrorType> {
    let mut args = args.split_whitespace();

    let duration = match args.next() {
        Some(arg) => match arg.parse::<i32>() {
            Ok(arg) => arg,
            Err(_) => return Err(ParserErrorType::InvalidType { argument: "<duration>", expected_type: "integer" }),
        },
        None => return Err(ParserErrorType::MissingArgs { arguments: "<duration>", expected_command: "n duration" }),
    };

    if args.next().is_some() {
        return Err(ParserErrorType::TooManyArgs { expected_command: "n <duration>" });
    }

    Ok(NCommand { 
        duration
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_arguments_valid_input() {
        assert_eq!(parse_arguments("10"), Ok(NCommand { duration: 10 }));
        assert_eq!(parse_arguments("100"), Ok(NCommand { duration: 100 }));
    }

    #[test]
    fn test_parse_arguments_invalid_input() {
        assert_eq!(parse_arguments(""), Err(ParserErrorType::MissingArgs { arguments: "<duration>", expected_command: "n duration" }));
        assert_eq!(parse_arguments("abc"), Err(ParserErrorType::InvalidType { argument: "<duration>", expected_type: "integer" }));
        assert_eq!(parse_arguments("10 extra"), Err(ParserErrorType::TooManyArgs { expected_command: "n <duration>" }));
    }
}

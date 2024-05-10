use super::Command;

pub fn parse_command(args: &str) -> Result<Command, &'static str> {
    let (duration, description) = match args.split_once(' ') {
        Some((duration, description)) => (duration, description),
        None => return Err("Expected 't <duration> <description>'"),
    };

    let duration = match duration.parse::<i32>() {
        Ok(duration) => duration,
        Err(_) => return Err("Expected <duration> as integer"),
    };

    let description = description.trim().to_owned();

    Ok(Command::NewTask{
        duration,
        description,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_valid_command() {
        let expected_duration = 10;
        let expected_description = "finish rust project!";
        let args = format!("{} {}", expected_duration, expected_description);

        let result = parse_command(&args)
            .unwrap_or_else(|err| panic!("Error: {}", err));

        match result {
            Command::NewTask { duration, description }
                => {
                    assert_eq!(expected_duration, duration);
                    assert_eq!(expected_description, description);
                },
            _ => panic!("Parsed wrong command type. Expected Command::NewTask, but got {:?}", result),
        }
    }

    #[test]
    #[should_panic(expected = "'t <duration> <description>'")]
    fn parse_no_arguments() {
        let args = "";

        let result = parse_command(args);

        result.unwrap();
    }

    #[test]
    #[should_panic(expected = "<duration> as integer")]
    fn parse_no_duration() {
        let expected_description = "finish rust project!";
        let args = String::from(expected_description);

        let result = parse_command(&args);

        result.unwrap();
    }

    #[test]
    #[should_panic(expected = "'t <duration> <description>'")]
    fn parse_no_description() {
        let expected_duration = 10;
        let args = format!("{}", expected_duration);

        let result = parse_command(&args)
            .unwrap_or_else(|err| panic!("Error: {}", err));

        match result {
            Command::NewTask { duration, description }
                => {
                    assert_eq!(expected_duration, duration);
                    assert_eq!(0, description.len());
                },
            _ => panic!("Parsed wrong command type. Expected Command::NewTask, but got {:?}", result),
        }
    }

    #[test]
    #[should_panic(expected = "<duration> as integer")]
    fn parse_duration_not_a_number() {
        let expected_duration = "0xa4e";
        let expected_description = "finish rust project!";
        let args = format!("{} {}", expected_duration, expected_description);

        let result = parse_command(&args);

        result.unwrap();
    }
}
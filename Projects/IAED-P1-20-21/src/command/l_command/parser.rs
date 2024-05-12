use super::{LCommand, ParserErrorType};

pub fn parse_arguments(args: &str) -> Result<LCommand, ParserErrorType> {
    let args = args.split_whitespace();

    let task_ids: Result<Vec<i32>, _> = args
        .map(|arg| arg.parse::<i32>())
        .collect();

    let task_ids = match task_ids {
        Ok(task_ids) => match task_ids.is_empty() {
            true => None,
            false => Some(task_ids),
        }
        Err(_) => return Err(ParserErrorType::InvalidType { argument: "<task-id>", expected_type: "integer" }),
    };

    Ok(LCommand {
        task_ids,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_arguments_valid_input() {
        assert_eq!(parse_arguments(""), Ok(LCommand { task_ids: None }));
        assert_eq!(parse_arguments("1 2 3 4 5"), Ok(LCommand { task_ids: Some(vec![1, 2, 3, 4, 5]) }));
    }

    #[test]
    fn test_parse_arguments_invalid_input() {
        assert_eq!(parse_arguments("1 2 three"), Err(ParserErrorType::InvalidType { argument: "<task-id>", expected_type: "integer" }));
        assert_eq!(parse_arguments("one two three"), Err(ParserErrorType::InvalidType { argument: "<task-id>", expected_type: "integer" }));
    }
}
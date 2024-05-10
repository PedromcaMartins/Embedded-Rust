use super::Command;

pub fn parse_command(args: &str) -> Result<Command, &'static str> {
    let args = args.split_whitespace();

    let task_ids: Result<Vec<i32>, _> = args
        .map(|arg| arg.parse::<i32>())
        .collect();

    let task_ids = match task_ids {
        Ok(task_ids) => match task_ids.is_empty() {
            true => None,
            false => Some(task_ids),
        }
        Err(_) => return Err("Invalid type: Expected <task-id> to be integer"),
    };

    Ok(Command::L {
        task_ids,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_command_valid_input() {
        assert_eq!(parse_command(""), Ok(Command::L { task_ids: None }));
        assert_eq!(parse_command("1 2 3 4 5"), Ok(Command::L { task_ids: Some(vec![1, 2, 3, 4, 5]) }));
    }

    #[test]
    fn test_parse_command_invalid_input() {
        assert_eq!(parse_command("1 2 three"), Err("Invalid argument: Expected <task-id> to be integer"));
        assert_eq!(parse_command("one two three"), Err("Invalid argument: Expected <task-id> to be integer"));
    }
}
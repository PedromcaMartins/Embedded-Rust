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
        Err(_) => return Err("Expected <task-id> as integer"),
    };

    Ok(Command::L {
        task_ids,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_no_arguments() {
        let args = "";

        let result = parse_command(args);

        assert_eq!(result, Ok(Command::L { task_ids: None }));
    }

    #[test]
    fn parse_valid_arguments() {
        let args = "1 2 3 4 5";

        let result = parse_command(args);
        let expected_task_ids = vec![1, 2, 3, 4, 5];

        assert_eq!(
            result, 
            Ok(Command::L {
                task_ids: Some(expected_task_ids) 
            })
        );
    }

    #[test]
    #[should_panic(expected = "as integer")]
    fn parse_task_ids_not_a_number() {
        let args = "one two three";

        let results = parse_command(args);

        results.unwrap();
    }
}
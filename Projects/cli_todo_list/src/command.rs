use clap::{ArgGroup, Parser};
use chrono::NaiveDate;
use anyhow::Result;

#[derive(Debug, Parser)]
#[clap(name = "Todo", about = "A simple command-line todo application")]
pub enum Command {
    /// Add Task
    Add {
        /// The description of the task.
        #[arg(short, long, num_args = 1.., required = true)]
        task: Vec<String>,

        /// The due date for the task in `YYYY-MM-DD` format.
        #[arg(short, long)]
        due: Option<NaiveDate>,
    },

    /// List Tasks
    List {
        /// List all tasks (default if no other argument is provided).
        #[arg(short, long, exclusive = true)]
        all: bool,

        /// List tasks due on a specific date in `YYYY-MM-DD` format.
        #[arg(short, long, exclusive = true)]
        due: Option<NaiveDate>,
    },

    /// Remove Task
    Remove {
        /// The ID of the task to be removed.
        #[arg(short, long)]
        id: u64,
    },

    /// Update Task
    #[clap(group = ArgGroup::new("update_opts").required(true).multiple(true).args(&["task", "due"]))]
    Update {
        /// The ID of the task to be updated.
        #[arg(short, long, required = true)]
        id: u64,

        /// The new description of the task.
        #[arg(short, long, num_args = 1.., group = "update_opts")]
        task: Option<Vec<String>>,

        /// The new due date for the task in `YYYY-MM-DD` format.
        #[arg(short, long, group = "update_opts")]
        due: Option<NaiveDate>,
    },

    /// Complete Task
    Complete {
        /// The ID of the task to be marked as completed.
        #[arg(short, long)]
        id: u64,
    },
}

impl Command {
    pub fn try_parse_from_line(line: String) -> Result<Command> {
        // Split the input line into arguments
        let args: Vec<String> = line.split_whitespace().map(String::from).collect();

        let mut cli_args = vec!["todo".to_string()];
        cli_args.extend(args); // Add the program name as the first argument

        Ok(Command::try_parse_from(cli_args)?)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_valid_commands() {
        let args = Command::try_parse_from_line("add --task Buy groceries".to_string());
        assert!(args.is_ok());

        let args = Command::try_parse_from_line("add --task Buy groceries --due 2024-07-05".to_string());
        assert!(args.is_ok());

        let args = Command::try_parse_from_line("list".to_string());
        assert!(args.is_ok());

        let args = Command::try_parse_from_line("list --all".to_string());
        assert!(args.is_ok());

        let args = Command::try_parse_from_line("list --due 2024-07-05".to_string());
        assert!(args.is_ok());

        let args = Command::try_parse_from_line("remove --id 1".to_string());
        assert!(args.is_ok());

        let args = Command::try_parse_from_line("update --id 1 --task Buy groceries".to_string());
        assert!(args.is_ok());

        let args = Command::try_parse_from_line("update --id 1 --due 2022-05-24".to_string());
        assert!(args.is_ok());

        let args = Command::try_parse_from_line("update --id 1 --task Buy groceries --due 2024-07-05".to_string());
        assert!(args.is_ok());

        let args = Command::try_parse_from_line("complete --id 1".to_string());
        assert!(args.is_ok());
    }

    #[test]
    fn test_invalid_commands() {
        let args = Command::try_parse_from_line("add".to_string());
        assert!(args.is_err());

        let args = Command::try_parse_from_line("add --due 2024-06-30".to_string());
        assert!(args.is_err());

        let args = Command::try_parse_from_line("list --all --due 2024-07-05".to_string());
        assert!(args.is_err());

        let args = Command::try_parse_from_line("remove".to_string());
        assert!(args.is_err());

        let args = Command::try_parse_from_line("update".to_string());
        assert!(args.is_err());

        let args = Command::try_parse_from_line("update --id 1".to_string());
        assert!(args.is_err());

        let args = Command::try_parse_from_line("update --task Buy groceries".to_string());
        assert!(args.is_err());

        let args = Command::try_parse_from_line("update --due 2024-07-05".to_string());
        assert!(args.is_err());

        let args = Command::try_parse_from_line("update --task Buy groceries --due 2024-07-05".to_string());
        assert!(args.is_err());

        let args = Command::try_parse_from_line("complete".to_string());
        assert!(args.is_err());
    }
}
use std::str::FromStr;
use crate::{args_verifier, AppError};


#[derive(Debug)]
pub enum Command {
    Quit,
    AddTask { estimated_duration: i32, description: String },
    ListTasks { ids: Option<Vec<i32>> },
    AdvanceTime { duration: i32 },
    AddOrListUsers { username: Option<String> },
    MoveTask { id: i32, username: String, activity: String },
    ListTasksInActivity { activity: String },
    AddOrListActivities { activity: Option<String> },
}


impl FromStr for Command {
    type Err = AppError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_whitespace().collect();

        match parts.first() {
            Some(&"q") => Ok(Command::Quit),
            Some(&"t") if parts.len() >= 3 => Ok(Command::AddTask { 
                estimated_duration: args_verifier::duration(parts[1])?,
                description: args_verifier::description(&parts[2..].join(" "))?.to_string(),
            }),
            Some(&"l") => Ok(Command::ListTasks { 
                ids: match parts.len() >= 2 {
                    true => Some(
                        parts[1..]
                            .iter()
                            .map(|&part| args_verifier::id(part))
                            .collect::<Result<Vec<i32>, AppError>>()?
                    ),
                    false => None,
                }
            }),
            Some(&"n") if parts.len() == 2 => Ok(Command::AdvanceTime { 
                duration: args_verifier::duration(parts[1])?,
            }),
            Some(&"u") => Ok(Command::AddOrListUsers { 
                username: match parts.len() >= 2 {
                    true => Some(args_verifier::username(parts[1])?.to_string()),
                    false => None,
                }
            }),
            Some(&"m") if parts.len() >= 4 => Ok(Command::MoveTask { 
                id: args_verifier::id(parts[1])?,
                username: args_verifier::username(parts[2])?.to_string(),
                activity: args_verifier::activity(&parts[3..].join(" "))?.to_string(),
            }),
            Some(&"d") if parts.len() >= 2 => Ok(Command::ListTasksInActivity { 
                activity: args_verifier::activity(&parts[1..].join(" "))?.to_string(),
            }),
            Some(&"a") => Ok(Command::AddOrListActivities { 
                activity: match parts.len() >= 2 {
                    true => Some(args_verifier::activity(&parts[1..].join(" "))?.to_string()),
                    false => None,
                }
            }),
            Some(_) => Err(AppError::InvalidCommandError),
            None => Err(AppError::InvalidCommandError),
        }
    }
}
use std::str::FromStr;
use crate::AppError;

#[derive(Debug)]
pub enum Command {
    Help,
    Quit,
    Set{ path: Vec<String>, value: String },
    Print,
    Find{ path: Vec<String> },
    List{ path: Option<Vec<String>> },
    Search{ value: String },
    Delete{ path: Option<Vec<String>> },
}

impl FromStr for Command {
    type Err = AppError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_whitespace().collect();

        match parts.first() {
            Some(&"help") if parts.len() == 1 => Ok(Command::Help),
            Some(&"quit") if parts.len() == 1 => Ok(Command::Quit),
            Some(&"set") if parts.len() >= 3 => Ok(Command::Set{
                path: parse_path(parts[1])?,
                value: parts[2..].join(" "),
            }),
            Some(&"print") if parts.len() == 1 => Ok(Command::Print),
            Some(&"find") if parts.len() == 2 => Ok(Command::Find{
                path: parse_path(parts[1])?,
            }),
            Some(&"list") if parts.len() <= 2 => Ok(Command::List{
                path: match parts.len() > 1 {
                    true => Some(parse_path(parts[1])?),
                    false => None,
                },
            }),
            Some(&"search") if parts.len() >= 2 => Ok(Command::Search{ 
                value: parts[1..].join(" "),
            }),
            Some(&"delete") if parts.len() <= 2 => Ok(Command::Delete{
                path: match parts.len() > 1 {
                    true => Some(parse_path(parts[1])?),
                    false => None,
                },
            }),
            Some(_) => Err(AppError::InvalidCommand),
            None => Err(AppError::InvalidCommand),
        }
    }
}

fn parse_path(path: &str) -> Result<Vec<String>, AppError> {
    let path: Vec<String> = path
        .trim_matches('/')
        .split('/')
        .map(|s| s.to_string())
        .collect();

    if path.iter().any(|component| component.is_empty()) {
        return Err(AppError::InvalidPath)
    }

    Ok(path)
}
use std::fmt::Display;

#[derive(Debug)]
pub enum AppError {
    InvalidCommand,
    InvalidPath,
    CommandFindPathNotFound,
    CommandFindNoData,
    CommandListPathNotFound,
    CommandSearchPathNotFound,
    CommandDeletePathNotFound,
}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::InvalidCommand => write!(f, "invalid command"),
            AppError::InvalidPath => write!(f, "invalid path"),
            AppError::CommandFindPathNotFound => write!(f, "not found"),
            AppError::CommandFindNoData => write!(f, "no data"),
            AppError::CommandListPathNotFound => write!(f, "not found"),
            AppError::CommandSearchPathNotFound => write!(f, "not found"),
            AppError::CommandDeletePathNotFound => write!(f, "not found"),
        }
    }
}
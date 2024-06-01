use std::{fmt::Display};

#[derive(Debug)]
pub enum CommandTError {
    TooManyTasks,
    DuplicateDescription,
    InvalidEstimatedDuration,
}


#[derive(Debug)]
pub enum CommandLError {
    NoSuchTask{ id: i32 },
}


#[derive(Debug)]
pub enum CommandNError {
    InvalidTime,
}


#[derive(Debug)]
pub enum CommandUError {
    UserAlreadyExists,
    TooManyUsers,
}


#[derive(Debug)]
pub enum CommandMError {
    NoSuchTask,
    TaskAlreadyStarted,
    NoSuchUser,
    NoSuchActivity,
}


#[derive(Debug)]
pub enum CommandDError {
    NoSuchActivity,
}


#[derive(Debug)]
pub enum CommandAError {
    DuplicateActivity,
    TooManyActivity,
}

#[derive(Debug)]
pub enum AppError {
    InvalidCommandError,
    // ---------------
    TaskIdValueOutOfBounds,
    TaskDescriptionLengthOutOfBounds,
    UsernameLengthOutOfBounds,
    ActivityNameLengthOutOfBounds,
    ActivityNameInLowerCase,
    ParseIntegerFailed,
    // ---------------
    CommandT(CommandTError),
    CommandL(CommandLError),
    CommandN(CommandNError),
    CommandU(CommandUError),
    CommandM(CommandMError),
    CommandD(CommandDError),
    CommandA(CommandAError),
}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::InvalidCommandError => write!(f, "Invalid command error"),
            // ---------------
            AppError::TaskIdValueOutOfBounds => write!(f, "Task ID value out of bounds"),
            AppError::TaskDescriptionLengthOutOfBounds => write!(f, "Task description length out of bounds"),
            AppError::UsernameLengthOutOfBounds => write!(f, "Username length out of bounds"),
            AppError::ActivityNameLengthOutOfBounds => write!(f, "Activity name length out of bounds"),
            AppError::ActivityNameInLowerCase => write!(f, "Activity name must be in uppercase"),
            AppError::ParseIntegerFailed => write!(f, "Failed to parse integer"),
            // --------------
            AppError::CommandT(CommandTError::TooManyTasks) => write!(f, "Too many tasks"),
            AppError::CommandT(CommandTError::DuplicateDescription) => write!(f, "Duplicate description"),
            AppError::CommandT(CommandTError::InvalidEstimatedDuration) => write!(f, "Invalid duration"),
            AppError::CommandL(CommandLError::NoSuchTask{ id }) => write!(f, "{}: No such task", id),
            AppError::CommandN(CommandNError::InvalidTime) => write!(f, "Invalid time"),
            AppError::CommandU(CommandUError::UserAlreadyExists) => write!(f, "User already exists"),
            AppError::CommandU(CommandUError::TooManyUsers) => write!(f, "Too many users"),
            AppError::CommandM(CommandMError::NoSuchTask) => write!(f, "No such task"),
            AppError::CommandM(CommandMError::TaskAlreadyStarted) => write!(f, "Task already started"),
            AppError::CommandM(CommandMError::NoSuchUser) => write!(f, "No such user"),
            AppError::CommandM(CommandMError::NoSuchActivity) => write!(f, "No such activity"),
            AppError::CommandD(CommandDError::NoSuchActivity) => write!(f, "No such activity"),
            AppError::CommandA(CommandAError::DuplicateActivity) => write!(f, "Duplicate activity"),
            AppError::CommandA(CommandAError::TooManyActivity) => write!(f, "Too many activities"),
        }
    }
}
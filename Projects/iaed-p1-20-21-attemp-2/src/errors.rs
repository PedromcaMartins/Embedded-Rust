#[derive(Debug)]
pub enum CommandTError {
    TooManyTasks,
    DuplicateDescription,
    InvalidEstimatedDuration,
}


#[derive(Debug)]
pub enum CommandLError {
    NoSuchTask,
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


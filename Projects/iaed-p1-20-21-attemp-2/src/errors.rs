#[derive(Debug)]
pub enum QuestionTError {
    TooManyTasks,
    DuplicateDescription,
    InvalidDuration,
}


#[derive(Debug)]
pub enum QuestionLError {
    NoSuchTask,
}


#[derive(Debug)]
pub enum QuestionNError {
    InvalidTime,
}


#[derive(Debug)]
pub enum QuestionUError {
    UserAlreadyExists,
    InvalidUsername,
    TooManyUsers,
}


#[derive(Debug)]
pub enum QuestionMError {
    NoSuchTask,
    TaskAlreadyStarted,
    NoSuchUser,
    NoSuchActivity,
}


#[derive(Debug)]
pub enum QuestionDError {
    NoSuchActivity,
}


#[derive(Debug)]
pub enum QuestionAError {
    ActivityAlreadyExists,
    InvalidDescription,
    TooManyActivity,
}

#[derive(Debug)]
pub enum AppError {
    ParseCommandError,
    VerifyArgsError,
    // ---------------
    QuestionT(QuestionTError),
    QuestionL(QuestionLError),
    QuestionN(QuestionNError),
    QuestionU(QuestionUError),
    QuestionM(QuestionMError),
    QuestionD(QuestionDError),
    QuestionA(QuestionAError),
}
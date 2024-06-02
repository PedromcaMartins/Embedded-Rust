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
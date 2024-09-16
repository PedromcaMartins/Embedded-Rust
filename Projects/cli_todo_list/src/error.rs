use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("No tasks found.")]
    NoTasksFound,

    #[error("No tasks due on {due}.")]
    NoTasksDueOn { due: String },

    #[error("Task with id {id} not found.")]
    TaskNotFound { id: u64 },
}
use std::collections::HashMap;
use crate::{args_verifier, AppError, Command, QuestionTError, INTERVAL_VALID_TASK_ID};

mod task;
use task::Task;


pub struct System {
    tasks: HashMap<i32, Task>,
    activities: Vec<&'static str>,
    usernames: Vec<&'static str>,
    current_time: i64,
    // ---------------
    current_id_task: i32,
}


impl Default for System {

    fn default() -> Self {
        System::new()
    }

}


impl System {

    pub fn new() -> Self {
        System {
            tasks: HashMap::new(),
            activities: vec!["TO DO", "IN PROGRESS", "DONE"],
            usernames: vec![],
            current_time: 0,
            // ---------------
            current_id_task: 0,
        }
    }

    pub fn execute(&self, command: Command) -> Result<(), AppError> {
        match command {
            Command::Quit => panic!("Quit command shouldn't execute quit command"),
            Command::AddTask { duration, description } => Self::command_t(self, duration, description),
            Command::ListTasks { ids } => Self::command_l(self, ids),
            Command::AdvanceTime { duration } => Self::command_n(self, duration),
            Command::AddOrListUsers { username } => Self::command_u(self, username),
            Command::MoveTask { id, username, activity } => Self::command_m(self, id, username, activity),
            Command::ListTasksInActivity { activity } => Self::command_d(self, activity),
            Command::AddOrListActivities { activity } => Self::command_a(self, activity),
        }
    }


    fn command_t(&self, duration: i32, description: String) -> Result<(), AppError> {
        if !INTERVAL_VALID_TASK_ID.contains(&self.current_id_task) {
            return Err(AppError::QuestionT(QuestionTError::TooManyTasks));
        }

        Ok(())
    }

    fn command_l(&self, ids: Option<Vec<i32>>) -> Result<(), AppError> {
        Ok(())
    }

    fn command_n(&self, duration: i32) -> Result<(), AppError> {
        Ok(())
    }

    fn command_u(&self, username: Option<String>) -> Result<(), AppError> {
        Ok(())
    }

    fn command_m(&self, id: i32, username: String, activity: String) -> Result<(), AppError> {
        Ok(())
    }

    fn command_d(&self, activity: String) -> Result<(), AppError> {
        Ok(())
    }

    fn command_a(&self, activity: Option<String>) -> Result<(), AppError> {
        Ok(())
    }
}
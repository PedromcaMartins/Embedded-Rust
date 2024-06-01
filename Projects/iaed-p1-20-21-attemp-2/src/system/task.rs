use crate::{TASK_DEFAULT_ACTIVITY, TASK_DEFAULT_START_TIME};

#[derive(Debug, Default)]
pub struct Task {
    id: i32,
    description: String,
    username: String,
    activity: String,
    task_duration: i32,
    start_time: i32,
}


impl Task {
    fn new(id: i32, description: String, username: String, activity: String, task_duration: i32, start_time: i32) -> Self {
        Self {
            id,
            description,
            username,
            activity: TASK_DEFAULT_ACTIVITY.to_string(),
            task_duration,
            start_time: TASK_DEFAULT_START_TIME,
        }
    }
}
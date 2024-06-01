use crate::{TASK_DEFAULT_END_ACTIVITY, TASK_DEFAULT_START_ACTIVITY, TASK_DEFAULT_START_TIME};


#[derive(Debug, Default)]
pub struct Task {
    id: i32,
    description: String,
    username: String,
    activity: String,
    estimated_duration: i32,
    start_time: i32,
    end_time: i32,
}


impl Task {
    pub fn new(id: i32, description: String, estimated_duration: i32) -> Self {
        Self {
            id,
            description,
            username: String::new(),
            activity: TASK_DEFAULT_START_ACTIVITY.to_string(),
            estimated_duration,
            start_time: TASK_DEFAULT_START_TIME,
            end_time: TASK_DEFAULT_START_TIME,
        }
    }

    pub fn get_id(&self) -> &i32 {
        &self.id
    }

    pub fn get_activity(&self) -> &str {
        &self.activity
    }

    pub fn get_description(&self) -> &str {
        &self.description
    }

    pub fn get_estimated_duration(&self) -> &i32 {
        &self.estimated_duration
    }

    pub fn get_start_time(&self) -> &i32 {
        &self.start_time
    }

    pub fn get_spent(&self) -> i32 {
        self.end_time - self.start_time
    }

    pub fn get_slack(&self) -> i32 {
        self.get_spent() - self.estimated_duration
    }

    pub fn eq_description(&self, other: &str) -> bool {
        self.description.eq(other)
    }

    pub fn eq_activity(&self, other: &str) -> bool {
        self.activity.eq(other)
    }

    pub fn has_not_started(&self) -> bool {
        self.activity.eq(TASK_DEFAULT_START_ACTIVITY)
    }

    pub fn has_finished(&self) -> bool {
        self.activity.eq(TASK_DEFAULT_END_ACTIVITY)
    }

    pub fn start_task(&mut self, username: &String, activity: &String, current_time: &i32) {
        self.start_time = current_time.to_owned();
        self.change_activity(username, activity, current_time);
    }

    pub fn change_activity(&mut self, username: &String, activity: &String, current_time: &i32) {
        self.username = username.to_owned();
        self.activity = activity.to_owned();

        if activity.eq(TASK_DEFAULT_END_ACTIVITY) {
            self.end_time = current_time.to_owned();
        }
    }
}
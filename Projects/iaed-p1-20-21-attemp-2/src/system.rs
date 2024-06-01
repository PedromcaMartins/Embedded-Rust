use std::{collections::HashMap, process};
use crate::{AppError, Command, CommandAError, CommandLError, CommandNError, CommandTError, CommandUError, INTERVAL_VALID_TASK_ID, MAX_ACTIVITIES_IN_SYSTEM, MAX_USERS_IN_SYSTEM, TASK_DEFAULT_END_ACTIVITY, TASK_DEFAULT_START_ACTIVITY};

mod task;
use task::Task;


pub struct System {
    tasks: HashMap<i32, Task>,
    activities: Vec<String>,
    usernames: Vec<String>,
    current_time: i32,
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
            activities: vec![
                TASK_DEFAULT_START_ACTIVITY.to_string(), 
                "IN PROGRESS".to_string(), 
                TASK_DEFAULT_END_ACTIVITY.to_string()
            ],
            usernames: vec![],
            current_time: 0,
            // ---------------
            current_id_task: 1,
        }
    }

    /// Executes the command that it receives and 
    /// stores the result in the System received
    /// 
    /// # Panics
    /// When Command::Quit is received, executes function `command_q`. 
    pub fn execute(&mut self, command: Command) -> Result<(), AppError> {
        match command {
            Command::Quit => Self::command_q(self),
            Command::AddTask { estimated_duration, description } => Self::command_t(self, estimated_duration, description),
            Command::ListTasks { ids } => Self::command_l(self, ids),
            Command::AdvanceTime { duration } => Self::command_n(self, duration),
            Command::AddOrListUsers { username } => Self::command_u(self, username),
            Command::MoveTask { id, username, activity } => Self::command_m(self, id, username, activity),
            Command::ListTasksInActivity { activity } => Self::command_d(self, activity),
            Command::AddOrListActivities { activity } => Self::command_a(self, activity),
        }
    }

    /// Terminates the program
    /// 
    /// Input format: `q`
    /// 
    /// # Panics
    /// When this function is executed, it prints a goodbye message and terminates the program
    fn command_q(&self) -> Result<(), AppError> {
        println!("Thank you for using Kanban Desk!");
        process::exit(0);
    }

    /// Adds a new task to the system
    /// 
    /// Input format: `t <duration> <description>`
    /// 
    /// Output format: `task <id>`, where id is the task's identifier
    /// 
    /// Note: description can have whitespaces
    /// 
    /// Errors: 
    /// - `too many tasks` in case the created task exceeds the max limit of tasks allowed by the system
    /// - `duplicate description` in case there's already a task with the same name
    /// - `invalid duration` in case duration is not a positive number
    fn command_t(&mut self, estimated_duration: i32, description: String) -> Result<(), AppError> {
        if !INTERVAL_VALID_TASK_ID.contains(&self.current_id_task) {
            return Err(AppError::CommandT(CommandTError::TooManyTasks));
        }

        if self.tasks.values().any(|task| task.eq_description(&description)) {
            return Err(AppError::CommandT(CommandTError::DuplicateDescription));
        }

        if estimated_duration <= 0 {
            return Err(AppError::CommandT(CommandTError::InvalidEstimatedDuration));
        }

        let new_task_id = self.current_id_task;
        self.current_id_task += 1;

        let new_task = Task::new(new_task_id, description, estimated_duration);
        self.tasks.insert(new_task_id, new_task);

        println!("task {}", new_task_id);

        Ok(())
    }

    /// Lists tasks
    /// 
    /// Input format: `l [<id> <id> ...]`
    /// 
    /// Output format: `<id> <activity> #<duration> <description>` for each task, one per line
    /// 
    /// Note: 
    /// - If the command is invoked without arguments, all tasks are listed in alphabetical order of the description
    /// - If the command is invoked with a list of ids, tasks should be listed in the order of their respective ids
    /// 
    /// Errors: 
    /// - `<id>: no such task` in case the indicated task does not exist
    fn command_l(&self, ids: Option<Vec<i32>>) -> Result<(), AppError> {
        let mut tasks: Vec<&Task>;

        match ids {
            Some(mut ids) => {
                ids.sort_unstable();

                tasks = ids
                    .iter()
                    .map(|id| self.tasks.get(id).ok_or(AppError::CommandL(CommandLError::NoSuchTask)))
                    .collect::<Result<Vec<&Task>, AppError>>()?;
            },
            None => {
                tasks = self.tasks.values().collect();
                tasks.sort_unstable_by(
                    |a, b| 
                    a.get_description().cmp(b.get_description())
                );
            },
        }

        for task in tasks {
            println!(
                "{} {} #{} {}", 
                task.get_id(), 
                task.get_activity(), 
                task.get_estimated_duration(), 
                task.get_description()
            );
        }

        Ok(())
    }

    /// Advances the system time
    /// 
    /// Input format: `n <duration>`
    /// 
    /// Output format: `<instant>`, where instant is the new current time value
    /// 
    /// Note: a duration of zero allows you to check the current time without changing it
    /// 
    /// Errors: 
    /// - `invalid time` if the duration is not a non-negative decimal integer
    fn command_n(&mut self, duration: i32) -> Result<(), AppError> {
        if duration <= 0 {
            return Err(AppError::CommandN(CommandNError::InvalidTime));
        }

        self.current_time = match self.current_time.checked_add(duration) {
            Some(time) => time,
            None => return Err(AppError::CommandN(CommandNError::InvalidTime)),
        };

        println!("{}", self.current_time);

        Ok(())
    }

    /// Adds a user or lists all users
    /// 
    /// Input format: `u [<user>]`
    /// 
    /// Output format: `[<username>]`
    /// 
    /// Note: 
    /// - If the command is invoked without arguments, all user names are listed, one name per line, in the order of creation
    /// - If the command is invoked with a username, a new user is created and nothing is printed (except for errors)
    /// 
    /// Errors: 
    /// - `user already exists` if a user with the same name already exists
    /// - `too many users` if the new user, if created, exceeds the user limit
    fn command_u(&mut self, username: Option<String>) -> Result<(), AppError> {
        let username = match username {
            Some(username) => username,
            None => {
                for username in &self.usernames {
                    println!("{}", username);
                }
                return Ok(());
            },
        };

        if self.usernames.contains(&username) {
            return Err(AppError::CommandU(CommandUError::UserAlreadyExists));
        }

        if self.usernames.len() >= MAX_USERS_IN_SYSTEM {
            return Err(AppError::CommandU(CommandUError::TooManyUsers));
        }

        self.usernames.push(username);

        Ok(())
    }

    /// Moves a task from one activity to another
    /// 
    /// Input format: `m <id> <user> <activity>`
    /// 
    /// Output format: `duration=<spent> slack=<slack>`, where spent is the time the task spent since leaving the TO DO activity until reaching the DONE activity, and slack is the difference between spent time and the estimated time (indicated when the task was created); if the activity is not DONE, nothing should be printed except for errors
    /// 
    /// Note: Once a task is started, and its start time is recorded, the task cannot be restarted; however, a completed task can be moved to an activity other than TO DO to resolve problems encountered
    /// 
    /// Errors: 
    /// - `no such task` if there is no task with the indicated identifier
    /// - `task already started` if attempting to move the task to the TO DO activity
    /// - `no such user` if there is no user with the indicated name
    /// - `no such activity` if there is no activity with the indicated name
    fn command_m(&mut self, id: i32, username: String, activity: String) -> Result<(), AppError> {
        let task: &mut Task = match self.tasks.get_mut(&id) {
            Some(task) => task,
            None => {
                return Err(AppError::CommandM(crate::CommandMError::NoSuchTask));
            },
        };

        if activity.eq(TASK_DEFAULT_START_ACTIVITY) {
            return Err(AppError::CommandM(crate::CommandMError::TaskAlreadyStarted));
        }

        if !self.usernames.contains(&username) {
            return Err(AppError::CommandM(crate::CommandMError::NoSuchUser));
        }

        if !self.activities.contains(&activity) {
            return Err(AppError::CommandM(crate::CommandMError::NoSuchActivity));
        }

        if task.has_not_started() {
            task.start_task(&username, &activity, &self.current_time);
        } else {
            task.change_activity(&username, &activity, &self.current_time);
        }

        if task.has_finished() {
            let spent = task.get_spent();
            let slack = task.get_slack();

            println!("duration={} slack={}", spent, slack);
        }

        Ok(())
    }

    /// Lists all tasks in a given activity
    /// 
    /// Input format: `d <activity>`
    /// 
    /// Output format: `<id> <start> <description>` for each task in the activity, one per line, in ascending order of start time (moment they leave the TO DO activity) and alphabetically by description if two or more tasks have the same start time
    /// 
    /// Errors: 
    /// - `no such activity` if there is no activity with that name
    fn command_d(&mut self, activity: String) -> Result<(), AppError> {
        let mut tasks_in_activity: Vec<&Task> = self.tasks
            .values()
            .filter(|task| task.eq_activity(&activity))
            .collect();

        tasks_in_activity.sort_unstable_by(
            |a, b| 
            match a.get_start_time().cmp(b.get_start_time()) {
                std::cmp::Ordering::Equal => a.get_description().cmp(b.get_description()),
                ordering => ordering
            }
        );

        for task in tasks_in_activity {
            println!(
                "{} {} {}", 
                task.get_id(), 
                task.get_start_time(), 
                task.get_description()
            );
        }

        Ok(())
    }

    /// Adds an activity or lists all activities
    /// 
    /// Input format: `a [<activity>]`
    /// 
    /// Output format: `[<activity>]`
    /// 
    /// Note: 
    /// - If the command is invoked without arguments, all activity names are listed, one name per line, in the order of creation
    /// - If the command is invoked with an activity name, a new activity is created and nothing is printed (except for errors)
    /// 
    /// Errors: 
    /// - `duplicate activity` if an activity with the same name already exists
    /// - `too many activities` if the activity, if created, exceeds the permitted activity limit
    fn command_a(&mut self, activity: Option<String>) -> Result<(), AppError> {
        let activity = match activity {
            Some(activity) => activity,
            None => {
                for activity in &self.activities {
                    println!("{}", activity);
                }
                return Ok(());
            },
        };

        if self.activities.contains(&activity) {
            return Err(AppError::CommandA(CommandAError::DuplicateActivity));
        }

        if self.activities.len() >= MAX_ACTIVITIES_IN_SYSTEM {
            return Err(AppError::CommandA(CommandAError::TooManyActivity))
        }

        self.activities.push(activity);

        Ok(())
    }
}
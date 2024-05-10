mod parser;

pub use parser::parser;

#[derive(Debug, PartialEq)]
pub enum Command {
    Quit,
    NewTask{ duration: i32, description: String },
    ListTasks{ task_ids: Option<Vec<i32>> },
    AdvanceTime{ duration: i32 },
    AddOrListAllUsers{ username: Option<String> },
    MoveTaskToActivity{ task_id: i32, username: String, activity: String },
    ListAllTasksInActivity{ activity: String },
    AddOrListAllActivities{ activity: Option<String> },
}

impl Command {
    pub fn execute(self) {
        match self {
            Command::Quit
                => panic!("execute should not receive command Quit!"),
            Command::NewTask { duration, description } 
                => new_task::execute(duration, description),
            _ => println!("function not yet implemented!"),
        }
    }
}

mod new_task {
    pub fn execute(duration: i32, description: String) {
        // TODO: verify if the values have the correct length, size, ...

        println!("executing new_task command");
    }
}
mod parser;

pub use parser::parser;

#[derive(Debug, PartialEq)]
pub enum Command {
    /// Quit Program
    Q,
    /// Add New Task
    T{ duration: i32, description: String },
    /// List All or Specific Tasks
    L{ task_ids: Option<Vec<i32>> },
    /// Advance Time
    N{ duration: i32 },
    /// Add User or List All Users
    U{ username: Option<String> },
    /// Move Task To Activity
    M{ task_id: i32, username: String, activity: String },
    /// List All Tasks In Activity
    D{ activity: String },
    /// Add Activity or List All Activities
    A{ activity: Option<String> },
}

impl Command {
    pub fn execute(self) {
        match self {
            Command::Q
                => panic!("execute should not receive command Quit!"),
            Command::T { duration, description } 
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
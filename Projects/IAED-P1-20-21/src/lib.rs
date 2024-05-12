mod parser;
mod verifier;
mod system;

pub use parser::parser;

use verifier::VerifierErrorType;

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
    pub fn verify_paramenters(&self) -> Result<(), VerifierErrorType> {
        verifier::verify_parameters(self)
    }

    pub fn execute(self) {
        system::execute(self);
    }
}
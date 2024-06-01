mod console;
mod parser;
mod args_verifier;
mod system;
mod errors;

pub use parser::Command;
pub use system::System;
pub use errors::{AppError, CommandAError, CommandDError, CommandLError, CommandMError, CommandNError, CommandTError, CommandUError};


const INTERVAL_VALID_TASK_ID: std::ops::Range<i32> = 1..10_000;
const INTERVAL_VALID_TASK_DESCRIPTION_LENGTH: std::ops::Range<usize> = 1..50;
const INTERVAL_VALID_ACTIVITY_NAME_LENGTH: std::ops::Range<usize> = 1..20;
const INTERVAL_VALID_USERNAME_LENGTH: std::ops::Range<usize> = 1..20;

const MAX_ACTIVITIES_IN_SYSTEM: usize = 10;
const MAX_USERS_IN_SYSTEM: usize = 10;

const TASK_DEFAULT_START_ACTIVITY: &str = "TO DO";
const TASK_DEFAULT_END_ACTIVITY: &str = "DONE";
const TASK_DEFAULT_START_TIME: i32 = 0;

pub use console::run_console;
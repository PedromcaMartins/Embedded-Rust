pub mod description;
pub mod duration;
pub mod task_ids;
pub mod username_op;
pub mod task_id;
pub mod username;
pub mod activity;
pub mod activity_op;

use crate::VerifierErrorType;

pub static MAX_LEN_ACTIVITY: usize = 20;
pub static MAX_LEN_USERNAME: usize = 20;
pub static MAX_LEN_DESCRIPTION: usize = 50;
pub static MIN_VALUE_TASK_ID: i32 = 1;
pub static MAX_VALUE_TASK_ID: i32 = 10_000;
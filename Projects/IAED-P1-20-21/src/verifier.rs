use super::Command;

pub use verifier_error::VerifierErrorType;

mod verifier_error;
mod description;
mod duration;
mod task_ids;
mod username_op;
mod task_id;
mod username;
mod activity;
mod activity_op;

pub fn verify_parameters(command: &Command) -> Result<(), VerifierErrorType> {
    match command {
        Command::Q
            => panic!("execute should not receive command Quit!"),
        Command::T { duration, description } 
            => { duration::verify(duration)?; description::verify(description)?; },
        Command::L { task_ids } 
            => task_ids::verify(task_ids)?,
        Command::N { duration }
            => duration::verify(duration)?,
        Command::U { username }
            => username_op::verify(username)?,
        Command::M { task_id, username, activity }
            => { task_id::verify(task_id)?; username::verify(username)?; activity::verify(activity)?; },
        Command::D { activity }
            => activity::verify(activity)?,
        Command::A { activity }
            => activity_op::verify(activity)?,
    };

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "should not receive command Quit")]
    fn test_verify_parameters_panic() {
        let _ = verify_parameters(&Command::Q);
    }

    #[test]
    fn test_verify_parameters_valid() {
        let command_t = Command::T { duration: 60, description: String::from("Sample task") };
        let command_l_1 = Command::L { task_ids: None };
        let command_l_2 = Command::L { task_ids: Some(vec![1, 2, 3]) };
        let command_n = Command::N { duration: 120, };
        let command_u_1 = Command::U { username: None };
        let command_u_2 = Command::U { username: Some(String::from("john_doe")) };
        let command_m = Command::M { task_id: 1, username: String::from("john_doe"), activity: String::from("TO-DO") };
        let command_d = Command::D { activity: String::from("TO-DO") };
        let command_a_1 = Command::A { activity: None };
        let command_a_2 = Command::A { activity: Some(String::from("TO-DO")) };

        assert!(verify_parameters(&command_t).is_ok());
        assert!(verify_parameters(&command_l_1).is_ok());
        assert!(verify_parameters(&command_l_2).is_ok());
        assert!(verify_parameters(&command_n).is_ok());
        assert!(verify_parameters(&command_u_1).is_ok());
        assert!(verify_parameters(&command_u_2).is_ok());
        assert!(verify_parameters(&command_m).is_ok());
        assert!(verify_parameters(&command_d).is_ok());
        assert!(verify_parameters(&command_a_1).is_ok());
        assert!(verify_parameters(&command_a_2).is_ok());
    }
}
use crate::{AppError, INTERVAL_VALID_ACTIVITY_NAME_LENGTH, INTERVAL_VALID_TASK_DESCRIPTION_LENGTH, INTERVAL_VALID_TASK_ID, INTERVAL_VALID_USERNAME_LENGTH};


pub fn id(id: &str) -> Result<i32, AppError> {
    let id = parse_integer(id)?;

    if !INTERVAL_VALID_TASK_ID.contains(&id) {
        return Err(AppError::VerifyArgsError {  })
    }

    Ok(id)
}

pub fn description(description: &str) -> Result<&str, AppError> {
    if !INTERVAL_VALID_TASK_DESCRIPTION_LENGTH.contains(&description.len()) {
        return Err(AppError::VerifyArgsError {  })
    }

    Ok(description)
}

pub fn username(username: &str) -> Result<&str, AppError> {
    if !INTERVAL_VALID_USERNAME_LENGTH.contains(&username.len()) {
        return Err(AppError::VerifyArgsError {  })
    }

    let mut username_split = username.split_whitespace();
    let _ = username_split.next();
    if username_split.next().is_some() {
        return Err(AppError::VerifyArgsError {  })
    }

    Ok(username)
}

pub fn activity(activity: &str) -> Result<&str, AppError> {
    if !INTERVAL_VALID_ACTIVITY_NAME_LENGTH.contains(&activity.len()) {
        return Err(AppError::VerifyArgsError {  })
    }

    if activity.to_uppercase() != activity {
        return Err(AppError::VerifyArgsError {  })
    }

    Ok(activity)
}

pub fn duration(duration: &str) -> Result<i32, AppError> {
    let duration = parse_integer(duration)?;

    if duration <= 0 {
        return Err(AppError::VerifyArgsError {  });
    }

    Ok(duration)
}

fn parse_integer(string: &str) -> Result<i32, AppError> {
    match string.parse::<i32>() {
        Ok(integer) => Ok(integer),
        Err(_) => Err(AppError::VerifyArgsError {  }),
    }
}
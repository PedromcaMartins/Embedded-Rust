use crate::{AppError, INTERVAL_VALID_ACTIVITY_NAME_LENGTH, INTERVAL_VALID_TASK_DESCRIPTION_LENGTH, INTERVAL_VALID_TASK_ID, INTERVAL_VALID_USERNAME_LENGTH};


pub fn id(id: &str) -> Result<i32, AppError> {
    let id = parse_integer(id)?;

    if !INTERVAL_VALID_TASK_ID.contains(&id) {
        return Err(AppError::TaskIdValueOutOfBounds)
    }

    Ok(id)
}

pub fn description(description: &str) -> Result<&str, AppError> {
    if !INTERVAL_VALID_TASK_DESCRIPTION_LENGTH.contains(&description.len()) {
        return Err(AppError::TaskDescriptionLengthOutOfBounds)
    }

    Ok(description)
}

pub fn username(username: &str) -> Result<&str, AppError> {
    if !INTERVAL_VALID_USERNAME_LENGTH.contains(&username.len()) {
        return Err(AppError::UsernameLengthOutOfBounds)
    }

    Ok(username)
}

pub fn activity(activity: &str) -> Result<&str, AppError> {
    if !INTERVAL_VALID_ACTIVITY_NAME_LENGTH.contains(&activity.len()) {
        return Err(AppError::ActivityNameLengthOutOfBounds)
    }

    if activity.to_uppercase().ne(activity) {
        return Err(AppError::ActivityNameInLowerCase)
    }

    Ok(activity)
}

pub fn duration(duration: &str) -> Result<i32, AppError> {
    let duration = parse_integer(duration)?;

    Ok(duration)
}

fn parse_integer(string: &str) -> Result<i32, AppError> {
    match string.parse::<i32>() {
        Ok(integer) => Ok(integer),
        Err(_) => Err(AppError::ParseIntegerFailed),
    }
}
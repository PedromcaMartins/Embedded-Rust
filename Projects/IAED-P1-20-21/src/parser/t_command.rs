use super::Command;

pub fn parse_command(line: &str) -> Result<Command, &'static str> {
    let (duration, description) = match line.split_once(' ') {
        Some((duration, description)) => (duration, description),
        None => return Err("Expected 't <duration> <description>'"),
    };

    let duration = match duration.parse::<i32>() {
        Ok(duration) => duration,
        Err(_) => return Err("Expected <duration> as integer"),
    };

    let description = description.trim().to_owned();

    // TODO: verify if the values have the correct length, size, ...
    Ok(Command::NewTask{
        duration,
        description,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    
}
use super::Command;

pub fn verify_type_restrictions(command: &Command) {
    match command {
        Command::Q
            => panic!("execute should not receive command Quit!"),
        Command::T { duration, description } 
            => t_command::verify(duration, description),
        _ => println!("function not yet implemented!"),
    }
}

mod t_command {
    pub fn verify(duration: &i32, description: &String) {
        println!("verifying if the values have the correct length, size, ...");
    }
}
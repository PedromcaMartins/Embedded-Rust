use super::Command;

pub fn execute(command: Command) {
    match command {
        Command::Q
            => panic!("execute should not receive command Quit!"),
        Command::T { duration, description } 
            => t_command::execute(duration, description),
        _ => println!("function not yet implemented!"),
    }
}

mod t_command {
    pub fn execute(duration: i32, description: String) {
        println!("executing new_task command");
    }
}
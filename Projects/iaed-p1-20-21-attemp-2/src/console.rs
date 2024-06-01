use std::io;

use crate::{Command, System};

pub fn run_console() {    
    println!("Welcome to IAED P1 20/21: Kanban Desk! Please select your command:");

    let mut system = System::new();

    loop {
        println!();
        print_commands();

        let line = read_line();

        let command = match line.parse::<Command>() {
            Ok(command) => command,
            Err(err) => {
                println!("{:?}", err);
                continue;
            },
        };

        if let Err(err) = system.execute(command) {
            println!("{:?}", err);
        }
    }
}

fn print_commands() {
println!("q ........................................ Exit program");
println!("t <estimated-duration> <description> ..... Add task to system");
println!("l [<id> <id> ...] ........................ List all tasks");
println!("n <duration> ............................. Advance system time");
println!("u [<username>] ........................... Add or list all users");
println!("m <id> <username> <activity> ............. Move task to activities");
println!("d <activity> ............................. List all tasks in activity");
println!("a [<activity>] ........................... Adds or lists all activities");
}


fn read_line() -> String {
    let mut buf = String::new();
    while io::stdin().read_line(&mut buf).is_err() {
        println!("Error parsing the line. Try again.");
    }
    buf
}



// let commands = vec![
//     "q",
//     "t 30 \"Meeting with team\"",
//     "l 1 2 3",
//     "n 15",
//     "u",
//     "u alice",
//     "m 2 bob review",
//     "d review",
//     "a",
//     "a development",
// ];
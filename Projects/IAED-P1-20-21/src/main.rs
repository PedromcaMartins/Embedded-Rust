use iaed_p1_20_21::ErrorKind;

fn main() {
    println!("Welcome to Kanban Desk! Please select one of the commands at hand:");

    loop {
        print_commands();

        if let Err(err) = iaed_p1_20_21::parser_argument_verifier_and_executer() {
            println!("{}", err);
            if err.error_kind == ErrorKind::QuitCommand {
                break;
            }
        }

        println!();
    }

    println!("Thank you for using Kanban Desk");
}


fn print_commands() {
    println!("q: quit");
    println!("t: add new task");
    println!("l: list tasks");
    println!("n: advance system time");
    println!("u: add or list all users");
    println!("m: move task to activity");
    println!("d: list all tasks in an activity");
    println!("a: add an activity or list all activities");
}
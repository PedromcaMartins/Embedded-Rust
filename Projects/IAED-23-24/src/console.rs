use std::io;

use crate::parser::Command;

pub fn run() {
    println!("Welcome to IAED 23/24 Project: Parking Lots Management System. Here are the available commands.");
    print_commands();

    loop {
        let line = read_line();
        match line.parse::<Command>() {
            Err(err) => println!("{:#?}", err),
            Ok(command) => {
                println!("{:#?}", command);
                // execute command
            },
        }
    }
}

fn print_commands() {
    println!();
    println!("q ................................................................. Terminates the program");
    println!("p [ <park-name> <capacity> <tariff-x> <tariff-y> <tariff-z> ] ..... Creates a parking lot with a billing system or lists existing parking lots");
    println!("e <park-name> <licence-plate> <date> <hour> ....................... Registers the entry of a vehicle");
    println!("s <park-name> <licence-plate> <date> <hour> ....................... Registers the exit of a vehicle");
    println!("v <licence-plate> ................................................. Lists the entries and exits of a vehicle");
    println!("f <park-name> [ <date> ] .......................................... Shows the billing of a parking lot");
    println!("r <park-name> ..................................................... Removes a parking lot from the system and all vehicle entries and exits from that parking lot");
}

fn read_line() -> String {
    let mut buf = String::new();
    if io::stdin().read_line(&mut buf).is_err() {
        println!("Error reading line. Please try again!");
    }
    buf
}
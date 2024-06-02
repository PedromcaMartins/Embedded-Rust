use std::io;
use crate::{System, Command};

pub fn run() {
    println!("Welcome to IAED 20/21 Project 2, Storage System.");

    let mut system = System::new();

    loop {
        let line = read_line();

        match line.parse::<Command>() {
            Err(err) => println!("{:?}", err),
            Ok(command) => {
                if let Err(err) = system.execute(command) {
                    println!("{:?}", err);
                }
            },
        };
    }
}

fn read_line() -> String {
    let mut buf = String::new();
    if io::stdin().read_line(&mut buf).is_err() {
        println!("Error reading line. Try again.");
    };
    buf
}
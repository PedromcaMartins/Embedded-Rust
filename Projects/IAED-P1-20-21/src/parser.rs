use std::io;

use super::Command;

mod t_command;


pub fn parser() -> Result<Command, &'static str> {
    let mut line = String::new();
    if io::stdin().read_line(&mut line).is_err() {
        return Err("Error reading line");
    }

    parse_line(line.as_str())
}


fn parse_line(line: &str) -> Result<Command, &'static str> {
    let mut line: std::str::Chars = line.trim().chars();
    let command = match line.next() {
        Some(char) => char,
        None => return Err("Expected command"),
    };

    let line = line.as_str().trim();

    match command {
        'q' => Err("quit"),
        't' => t_command::parse_command(line),
        _ => Err("Invalid command"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    
}
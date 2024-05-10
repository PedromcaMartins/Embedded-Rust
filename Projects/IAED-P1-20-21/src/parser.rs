use std::io;

use super::Command;

mod q_command;
mod t_command;
mod l_command;
mod n_command;
mod u_command;
mod m_command;
mod d_command;
mod a_command;


pub fn parser() -> Result<Command, &'static str> {
    let mut line = String::new();
    if io::stdin().read_line(&mut line).is_err() {
        return Err("Error reading line");
    }

    parse_line(&line)
}


fn parse_line(line: &str) -> Result<Command, &'static str> {
    let mut line = line.trim().chars();
    let command = match line.next() {
        Some(char) => char,
        None => return Err("Expected command"),
    };

    let args = line.as_str().trim();

    match command {
        'q' => q_command::parse_command(args),
        't' => t_command::parse_command(args),
        'l' => l_command::parse_command(args),
        'n' => n_command::parse_command(args),
        'u' => u_command::parse_command(args),
        'm' => m_command::parse_command(args),
        'd' => d_command::parse_command(args),
        'a' => a_command::parse_command(args),
        _ => Err("Invalid command"),
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser_all_command() {
        let line_q = "q";
        let line_t = "t 2 description";
        let line_l_1 = "l";
        let line_l_2 = "l 14 53";
        let line_n = "n 2";
        let line_u_1 = "u";
        let line_u_2 = "u username";
        let line_m = "m 2 username activity";
        let line_d = "d activity";
        let line_a_1 = "a";
        let line_a_2 = "a activity";

        assert!(parse_line(line_q).is_ok());
        assert!(parse_line(line_t).is_ok());
        assert!(parse_line(line_l_1).is_ok());
        assert!(parse_line(line_l_2).is_ok());
        assert!(parse_line(line_n).is_ok());
        assert!(parse_line(line_u_1).is_ok());
        assert!(parse_line(line_u_2).is_ok());
        assert!(parse_line(line_m).is_ok());
        assert!(parse_line(line_d).is_ok());
        assert!(parse_line(line_a_1).is_ok());
        assert!(parse_line(line_a_2).is_ok());
    }

    #[test]
    fn test_parser_invalid_input() {
        assert_eq!(parse_line(""), Err("Expected command"));
        assert_eq!(parse_line("s string la la la"), Err("Invalid command"));
    }
}
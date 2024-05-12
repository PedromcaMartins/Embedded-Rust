use std::io;
use crate::{ErrorKind, AppError, ParserErrorType};
use crate::command::{ACommand, Command, DCommand, LCommand, MCommand, NCommand, TCommand, UCommand};

pub fn parser() -> Result<(), AppError> {
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;

    parse_line(&line)
}


fn parse_line(line: &str) -> Result<(), AppError> {
    let mut line = line.trim().chars();
    let command = match line.next() {
        Some(char) => char,
        None => return Err(ParserErrorType::MissingCommand.into()),
    };

    let args = line.as_str().trim();

    match command {
        'q' => Err(AppError::new(ErrorKind::QuitCommand)),
        't' => TCommand::parse_and_verify_arguments_and_execute::<TCommand>(args),
        'l' => LCommand::parse_and_verify_arguments_and_execute::<LCommand>(args),
        'n' => NCommand::parse_and_verify_arguments_and_execute::<NCommand>(args),
        'u' => UCommand::parse_and_verify_arguments_and_execute::<UCommand>(args),
        'm' => MCommand::parse_and_verify_arguments_and_execute::<MCommand>(args),
        'd' => DCommand::parse_and_verify_arguments_and_execute::<DCommand>(args),
        'a' => ACommand::parse_and_verify_arguments_and_execute::<ACommand>(args),
        _ => Err(ParserErrorType::InvalidCommand.into()),
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
        let line_m = "m 2 username ACTIVITY";
        let line_d = "d ACTIVITY";
        let line_a_1 = "a";
        let line_a_2 = "a ACTIVITY";

        assert_eq!(parse_line(line_q), Err(AppError::new(ErrorKind::QuitCommand)));
        assert_eq!(parse_line(line_t), Ok(()));
        assert_eq!(parse_line(line_l_1), Ok(()));
        assert_eq!(parse_line(line_l_2), Ok(()));
        assert_eq!(parse_line(line_n), Ok(()));
        assert_eq!(parse_line(line_u_1), Ok(()));
        assert_eq!(parse_line(line_u_2), Ok(()));
        assert_eq!(parse_line(line_m), Ok(()));
        assert_eq!(parse_line(line_d), Ok(()));
        assert_eq!(parse_line(line_a_1), Ok(()));
        assert_eq!(parse_line(line_a_2), Ok(()));
    }

    #[test]
    fn test_parser_invalid_input_empty_string() {
        match parse_line("") {
            Ok(_) => panic!("Expected {}", ParserErrorType::MissingCommand),
            Err(err) => assert_eq!(err, AppError::from(ParserErrorType::MissingCommand)),
        };
    }

    #[test]
    fn test_parser_invalid_input_non_existant_command() {
        match parse_line("s string la la la") {
            Ok(_) => panic!("'s' command does not exist, so parser should return an Err()"),
            Err(err) => assert_eq!(err, AppError::from(ParserErrorType::InvalidCommand)),
        };
    }
}
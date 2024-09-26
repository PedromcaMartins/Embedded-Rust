use core::str::{self, FromStr};
use defmt::{debug, error, Format};

use crate::drivers::{UartError, UartWrapper};

#[derive(Format, Clone)]
pub enum CliCommand {
    Help,
    UserInputTest,
}

impl FromStr for CliCommand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "help" => Ok(CliCommand::Help),
            "test user input" => Ok(CliCommand::UserInputTest),
            _ => Err(())
        }
    }
}

pub struct Cli<'d> {
    uart: UartWrapper<'d>,
}

impl<'d> Cli<'d> {
    pub fn new(uart: UartWrapper<'d>) -> Self {
        Self { uart }
    }

    async fn write_line(&mut self, line: &str) -> Result<(), UartError> {
        self.uart.write_line(line.as_bytes()).await
    }

    pub async fn process(&mut self) -> Result<CliCommand, UartError> {
        let mut line = [0u8; 128];

        loop {
            self.uart.write("> ".as_bytes()).await?;

            let n = self.uart.read_line(&mut line).await?;

            if let Ok(command) = str::from_utf8(&line[..n]) {
                if command.trim().is_empty() {
                    continue;
                }

                if let Ok(command) = command.trim().parse::<CliCommand>() {
                    debug!("Cli command: {}", command);
                    return Ok(command);
                } else {
                    self.write_line("Invalid command!").await?;
                    error!("Command does not exist");
                }
            } else {
                self.write_line("Invalid characters").await?;
                error!("Invalid characters");
            }
        }
    }

    pub async fn display_help_message(&mut self) -> Result<(), UartError> {
        let help_message = 
"Usage: <cmd>\r
Commands:\r
\thelp              Show help\r
\ttest user input   Test input\r";

        self.write_line(help_message).await
    }
}

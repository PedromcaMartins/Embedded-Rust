use core::str::{self, FromStr};

use crate::traits::UartDriver;

pub enum CliCommands {
    Help,
}

impl FromStr for CliCommands {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "help" => Ok(CliCommands::Help),
            _ => Err(())
        }
    }
}

pub struct Cli<U: UartDriver> {
    uart: U,
}

impl<U: UartDriver> Cli<U> {
    pub fn new(uart: U) -> Self {
        Self { uart }
    }

    pub async fn process(&mut self) -> CliCommands {
        let mut line = [0u8; 128];

        loop {
            self.uart.write("> ".as_bytes()).await.unwrap();

            let n = self.uart.read_line(&mut line).await.unwrap();

            if let Ok(command) = str::from_utf8(&line[..n]) {
                if let Ok(command) = command.trim().parse::<CliCommands>() {
                    return command;
                }
            }

            self.write_line("Incorrect Command!").await;
        }
    }

    async fn write_line(&mut self, line: &str) {
        self.uart.write_line(line.as_bytes()).await.unwrap()
    }
}

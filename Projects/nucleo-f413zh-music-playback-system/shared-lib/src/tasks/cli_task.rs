use crate::{models::Cli, traits::{TimerDriver, UartDriver}};

pub struct CliTask<U: UartDriver> {
    cli: Cli<U>,
}

impl<U: UartDriver> CliTask<U> {
    pub fn new(uart: U) -> Self {
        Self { 
            cli: Cli::new(uart)
        }
    }

    pub async fn run<T: TimerDriver>(mut self, _: T) -> ! {
        loop {
            let _command = self.cli.process().await;
        }
    }
}

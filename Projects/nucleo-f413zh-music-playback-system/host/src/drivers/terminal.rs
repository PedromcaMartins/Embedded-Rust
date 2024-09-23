use std::io::{self, Read, Write};

use shared_lib::traits::UartDriver;

pub struct Terminal;

#[derive(Debug)]
pub enum TerminalError {
    StdinError,
    StdoutError,
}

impl Terminal {
    pub fn new() -> Self {
        Self
    }
}

impl UartDriver for Terminal {
    type UartDriverError = TerminalError;

    async fn write(
            &mut self, 
            buf: &[u8]
        ) -> Result<(), Self::UartDriverError> {
        io::stdout().write_all(buf).map_err(|_| TerminalError::StdoutError)?;
        io::stdout().flush().map_err(|_| TerminalError::StdoutError)?;

        Ok(())
    }

    async fn write_line(
            &mut self, 
            buf: &[u8]
        ) -> Result<(), Self::UartDriverError> {
        io::stdout().write_all(buf).map_err(|_| TerminalError::StdoutError)?;
        io::stdout().write_all("\r\n".as_bytes()).map_err(|_| TerminalError::StdoutError)?;

        Ok(())
    }

    async fn read<const BUF_SIZE: usize>(
            &mut self, 
            buf: &mut [u8; BUF_SIZE]
        ) -> Result<usize, Self::UartDriverError> {
        io::stdin().read(buf)
            .map_err(|_| TerminalError::StdinError)
    }

    async fn read_line<const BUF_SIZE: usize>(
            &mut self, 
            buf: &mut [u8; BUF_SIZE]
        ) -> Result<usize, Self::UartDriverError> {
        let mut string = String::new();
        if io::stdin().read_line(&mut string).is_err() {
            println!("Error reading line. Please try again!");
        }

        let bytes = string.as_bytes();
        let len = bytes.len().min(BUF_SIZE);
        buf[..len].copy_from_slice(&bytes[..len]);

        Ok(len)
    }
}

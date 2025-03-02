use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use colored::*;
use defmt_decoder::{DecodeError, Table};
use serialport::{SerialPort};
use std::{fs::File, io::{Read, Write}, path::PathBuf, time::Duration};

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    /// Path to defmt symbol file
    #[arg(short, long)]
    elf: PathBuf,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// List available COM ports
    List,

    /// Open COM port
    Open {
        #[arg(help = "COM port name (e.g., COM3 or /dev/ttyUSB0)")]
        port: String,

        #[arg(short, long, default_value = "115200")]
        baud: u32,
    },
}

fn list_ports() -> Result<()> {
    let ports = serialport::available_ports()?;
    if ports.is_empty() {
        println!("No COM ports found.");
    } else {
        println!("Available COM Ports:");
        for port in ports {
            println!(" - {}", port.port_name);
        }
    }
    Ok(())
}

fn open_port(port: &str, baud: u32, table: &Table) -> Result<()> {
    println!("Opening port: {} at {} baud", port, baud);
    let mut port = serialport::new(port, baud)
        .timeout(Duration::from_secs(10))
        .open()
        .with_context(|| format!("Failed to open port {}", port))?;

    let mut buf = [0u8; 10];
    println!("{}", "Listening...".green());

    loop {
        port.read_exact(&mut buf)?;

        match table.decode(&buf) {
            Ok((frame, _)) => println!("{}", frame.display(false)),
            Err(DecodeError::UnexpectedEof) => return Ok(()),
            Err(DecodeError::Malformed) => match table.encoding().can_recover() {
                // if recovery is impossible, abort
                false => return Err(DecodeError::Malformed.into()),
                // if recovery is possible, skip the current frame and continue with new data
                true => {
                        println!("(HOST) malformed frame skipped");
                        println!("└─ {} @ {}:{}", env!("CARGO_PKG_NAME"), file!(), line!());
                }
            }
        }
    }
}

const READ_BUFFER_SIZE: usize = 1024;

fn main() -> Result<()> {
    let args = Args::parse();

    let elf_data = std::fs::read(&args.elf)?;
    let table = Table::parse(&elf_data)?.unwrap();

    match &args.command {
        Command::List => list_ports(),
        Command::Open { port, baud } => open_port(port, *baud, &table),
    }
}

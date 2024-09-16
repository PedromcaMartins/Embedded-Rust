use std::io::{self, Write};

use clap::Parser;
use tracing::info;

mod command;
use command::Command;

mod server;
use server::Server;

mod error;
use error::AppError;

pub fn run() {
    // install global collector configured based on RUST_LOG env var.
    tracing_subscriber::fmt::init();

    // parse arguments from comand line
    let _ = Args::parse();

    // create a new server
    let mut server = Server::new();

    loop {
        print!("> ");
        io::stdout().flush().expect("Expected to flush stdout"); // Flush the stdout buffer

        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("Expected to read line");

        let cli = match Command::try_parse_from_line(line) {
            Ok(cli) => cli,
            Err(err) => {
                println!("{}", err);
                continue;
            },
        };

        info!("Command: {:?}", cli);

        if let Err(err) = server.run(cli) {
            println!("{}", err);
        }
    }
}

#[derive(clap::Parser)]
#[clap(name = "Todo", about = "A simple command-line todo application")]
struct Args {}
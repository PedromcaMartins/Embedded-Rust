use std::io;
use clap::Parser;
use linked_list::{Cli, Commands, Node};

fn main() {
    let mut root_node = Node::new_root();

    loop {
        let mut line = String::new();
        if io::stdin().read_line(&mut line).is_err() {
            println!("Error parsing line");
            continue;
        };
        let line = format!("program {}", line.trim());

        let cli = Cli::try_parse_from(line.split_ascii_whitespace());

        match cli {
            Ok(cli) => {
                match cli.command {
                    Commands::Add { value } => {
                        root_node.add(value);
                    },
                    Commands::Remove { value } => {
                        root_node.remove(value);
                    },
                    Commands::Insert { position_in_linked_list, value } => {
                        root_node.insert(position_in_linked_list, value);
                    },
                }
            },
            Err(e) => {
                println!("{}", e);
            }
        }
    }
}
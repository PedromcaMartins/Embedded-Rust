use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Add node to the linked list
    Add { 
        /// Value to add
        value: i32
    },
    /// Remove node from the linked list
    Remove { 
        /// Value to remove
        value: i32 
    },
    /// Insert node from the linked list
    Insert { 
        /// Position in linked list
        position_in_linked_list: i32,
        /// Value to insert
        value: i32 
    },
}
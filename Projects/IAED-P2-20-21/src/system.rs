use crate::{errors::AppError, parser::Command};

#[derive(Debug)]
pub struct System {

}


impl Default for System {
    fn default() -> Self {
        Self::new()
    }
}


impl System {
    pub fn new() -> Self {
        Self {

        }
    }

    pub fn execute(&mut self, command: Command) -> Result<(), AppError> {
        match command {
            Command::Help => self.help(),
            Command::Quit => self.quit(),
            Command::Set { path, value } => self.set(path, value),
            Command::Print => self.print(),
            Command::Find { path } => self.find(path),
            Command::List { path } => self.list(path),
            Command::Search { value } => self.search(value),
            Command::Delete { path } => self.delete(path),
        }
    }

    /// Prints the available commands
    /// 
    /// Input format: `help`
    /// 
    /// Output format: Prints the list of available commands, one per line, with
    /// `<command>: <description>` in the order and text presented in the table above.
    /// 
    /// Errors: Not applicable.
    fn help(&mut self) -> Result<(), AppError> {
        Ok(())
    }

    /// Terminates the program
    /// 
    /// Input format: `quit`
    /// 
    /// Output format: NOTHING
    /// 
    /// Errors: Not applicable.
    fn quit(&mut self) -> Result<(), AppError> {
        Ok(())
    }

    /// Adds or modifies the stored value
    /// 
    /// Input format: `set <path> <value>`
    /// 
    /// Output format: NOTHING
    /// 
    /// Errors: Not applicable.
    fn set(&mut self, path: Vec<String>, value: String) -> Result<(), AppError> {
        Ok(())
    }

    /// Prints all paths and values
    /// 
    /// Input format: `print`
    /// 
    /// Output format: Prints all paths and values (one path and value per line),
    /// in depth, in the order of creation of the components. Only paths with associated
    /// values should be printed. Paths should start with the separator '/' and be
    /// separated from the value by a space.
    /// 
    /// Errors: Not applicable.
    fn print(&mut self) -> Result<(), AppError> {
        Ok(())
    }

    /// Prints the stored value of a path
    /// 
    /// Input format: `find <path>`
    /// 
    /// Output format: Prints the value associated with the `<path>`.
    /// 
    /// Errors:
    /// - `not found` in case the path does not exist.
    /// - `no data` in case the path has no associated value.
    fn find(&mut self, path: Vec<String>) -> Result<(), AppError> {
        Ok(())
    }

    /// Lists all immediate components of a sub-path
    /// 
    /// Input format: `list <path>`
    /// 
    /// Output format: Prints all immediate components of the `<path>` in alphabetical
    /// order (ASCII order, uppercase first), i.e., its directory. If the command is invoked
    /// without arguments, it lists the root components.
    /// 
    /// Errors:
    /// - `not found` in case the path does not exist.
    fn list(&mut self, path: Vec<String>) -> Result<(), AppError> {
        Ok(())
    }

    /// Searches for the path given a value
    /// 
    /// Input format: `search <value>`
    /// 
    /// Output format: Prints the first path found that contains exactly the `<value>`
    /// indicated. The path starts with the separator '/' and has only one separator '/' between
    /// each component. Each component should be searched in the order of creation.
    /// 
    /// Errors:
    /// - `not found` in case there is no path with the indicated value.
    fn search(&mut self, value: String) -> Result<(), AppError> {
        Ok(())
    }

    /// Deletes all paths of a sub-path
    /// 
    /// Input format: `delete <path>`
    /// 
    /// Output format: Deletes the indicated `<path>` and all other paths for which `<path>`
    /// is a sub-path. If invoked without arguments, deletes all stored paths.
    /// 
    /// Errors:
    /// - `not found` in case the path does not exist.
    fn delete(&mut self, path: Vec<String>) -> Result<(), AppError> {
        Ok(())
    }
}
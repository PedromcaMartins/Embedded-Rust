use std::process;
use crate::{errors::AppError, parser::Command};

mod path;
use path::Component;


#[derive(Debug)]
pub struct System {
    components: Vec<Component>,
}


impl Default for System {
    fn default() -> Self {
        Self::new()
    }
}


impl System {
    pub fn new() -> Self {
        Self {
            components: Vec::new(),
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
    fn help(&self) -> Result<(), AppError> {
        println!("help: ................... Prints the available commands");
        println!("quit: ................... Terminates the program");
        println!("set <path> <value>: ..... Adds or modifies the stored value");
        println!("print: .................. Prints all paths and values");
        println!("find <path>: ............ Prints the stored value of a path");
        println!("list [<path>]: .......... Lists all immediate components of a sub-path");
        println!("search <value>: ......... Searches for the path given a value");
        println!("delete [<path>]: ........ Deletes all paths of a sub-path");

        Ok(())
    }

    /// Terminates the program
    /// 
    /// Input format: `quit`
    /// 
    /// Output format: NOTHING
    /// 
    /// Errors: Not applicable.
    fn quit(&self) -> Result<(), AppError> {
        println!("Thank you for using out Storage System!");
        process::exit(0);
    }

    /// Adds or modifies the stored value
    /// 
    /// Input format: `set <path> <value>`
    /// 
    /// Output format: NOTHING
    /// 
    /// Errors: Not applicable.
    fn set(&mut self, path: Vec<String>, value: String) -> Result<(), AppError> {
        // create sub-components, if they don't exist
        for depth in 1..path.len() {
            let sub_component_path = path[..depth].to_owned();
            if self.components.iter().all(|component| component.ne_path(&sub_component_path)) {
                self.components.push(Component::new(sub_component_path, String::new()))
            }
        }

        // Replace value
        self.components.push(Component::new(path, value));

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
    fn print(&self) -> Result<(), AppError> {
        let components_with_value: Vec<&Component> = self.components
            .iter()
            .filter(|component| component.has_value())
            .collect();

        for component in components_with_value {
            println!("/{} {}", component.get_path().join("/"), component.get_value())
        }

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
    fn find(&self, path: Vec<String>) -> Result<(), AppError> {
        let component = match self.components.iter().find(|component| component.eq_path(&path)) {
            Some(component) => component,
            None => return Err(AppError::CommandFindPathNotFound),
        };

        if component.has_no_value() {
            return Err(AppError::CommandFindNoData);
        }

        println!("{}", component.get_value());

        Ok(())
    }

    /// Lists all immediate components of a sub-path
    /// 
    /// Input format: `list [<path>]`
    /// 
    /// Output format: Prints all immediate components of the `<path>` in alphabetical
    /// order (ASCII order, uppercase first), i.e., its directory. If the command is invoked
    /// without arguments, it lists the root components.
    /// 
    /// Errors:
    /// - `not found` in case the path does not exist.
    fn list(&self, path: Option<Vec<String>>) -> Result<(), AppError> {
        let mut components: Vec<&Component> = match path {
            Some(path) => {
                if !self.components.iter().any(|component| component.eq_path(&path)) {
                    return Err(AppError::CommandListPathNotFound);
                }
                self.components
                    .iter()
                    .filter(|component| component.is_component_of_directory(&path))
                    .collect()
            },
            None => self.components
                .iter()
                .filter(|component| component.is_root_component())
                .collect(),
        };

        components.sort_unstable_by(|a, b| a.get_path().cmp(b.get_path()));

        for component in components {
            println!("/{} {}", component.get_path().join("/"), component.get_value())
        }

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
    fn search(&self, value: String) -> Result<(), AppError> {
        let component = match self.components.iter().find(|component| component.eq_value(&value)) {
            Some(component) => component,
            None => return Err(AppError::CommandSearchPathNotFound),
        };

        println!("/{}", component.get_path().join("/"));

        Ok(())
    }

    /// Deletes all paths of a sub-path
    /// 
    /// Input format: `delete [<path>]`
    /// 
    /// Output format: Deletes the indicated `<path>` and all other paths for which `<path>`
    /// is a sub-path. If invoked without arguments, deletes all stored paths.
    /// 
    /// Errors:
    /// - `not found` in case the path does not exist.
    fn delete(&mut self, path: Option<Vec<String>>) -> Result<(), AppError> {
        match path {
            Some(path) => {
                if !self.components.iter().any(|component| component.eq_path(&path)) {
                    return Err(AppError::CommandDeletePathNotFound);
                }
                self.components.retain(|component| !component.is_sub_component_of_directory(&path));
            },
            None => self.components.clear(),
        }

        Ok(())
    }
}
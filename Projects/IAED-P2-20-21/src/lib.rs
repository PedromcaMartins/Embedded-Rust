use parser::Command;
use errors::AppError;
use system::System;


mod parser;
mod console;
mod errors;
mod system;


pub use console::run;
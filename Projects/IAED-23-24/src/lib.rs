mod console;
mod parser;
mod error;
mod parking_lot;

pub use console::run;

const MAX_PARKING_LOTS_IN_SYSTEM: i32 = 20;
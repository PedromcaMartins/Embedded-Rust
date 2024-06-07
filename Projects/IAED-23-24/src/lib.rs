mod console;
mod parser;
mod error;
mod parking_lot;
mod system;

pub use console::run;

const MAX_PARKING_LOTS_IN_SYSTEM: usize = 20;
const RANGE_VALID_YEARS: std::ops::Range<i32> = 2_000..2_100;
const RANGE_VALID_MONTHS: std::ops::RangeInclusive<i32> = 1..=12;
const RANGE_VALID_HOURS: std::ops::Range<i32> = 0..24;
const RANGE_VALID_MINUTES: std::ops::Range<i32> = 0..60;

///                                                 inv jan feb mar apr may jun jul aug sept oct nov dec
const LOOKUP_TABLE_LAST_DAY_GIVEN_MONTH: [i32; 13] = [0, 31+1, 29+1, 31+1, 30+1, 31+1, 30+1, 31+1, 31+1, 30+1, 31+1, 30+1, 31+1];
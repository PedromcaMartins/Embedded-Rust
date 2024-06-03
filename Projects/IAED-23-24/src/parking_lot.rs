pub use licence_plate::LicencePlate;
pub use date::Date;
pub use time::Time;
pub use tariff::Tariff;

mod licence_plate;
mod date;
mod time;
mod tariff;

#[derive(Debug)]
pub struct ParkingLot {
    name: String,
    capacity: i32,
    /// Tariff for every 15 mins before 1st hour
    tariff_x: Tariff,
    /// Tariff for every 15 mins after 1st hour
    tariff_y: Tariff,
    /// Max tariff for the day
    /// 
    /// Note: 29th February is free
    tariff_z: Tariff,
    // ------------------
    available_spots: i32,
    entry_exit_log: Vec<LicencePlate>
}
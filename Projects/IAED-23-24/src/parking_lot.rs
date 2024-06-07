use std::collections::HashMap;

pub use licence_plate::LicencePlate;
pub use date::Date;
pub use logs::EntryExitLog;
pub use time::Time;
pub use tariff::Tariff;

use crate::error::AppError;

mod licence_plate;
mod date;
mod time;
mod tariff;
mod logs;

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
    // ---------------
    logs_cars_parked: HashMap<LicencePlate, EntryExitLog>,
    logs_cars_exited: Vec<EntryExitLog>,
    latest_date: Date,
    latest_time: Time,
    available_spots: i32,
    log_billing: HashMap<Date, Tariff>,
}

impl ParkingLot {
    pub fn new(
        name: String,
        capacity: i32,
        tariff_x: Tariff,
        tariff_y: Tariff,
        tariff_z: Tariff
    ) -> Self {
        ParkingLot {
            name,
            capacity,
            tariff_x,
            tariff_y,
            tariff_z,
            // ---------------
            logs_cars_parked: HashMap::new(),
            logs_cars_exited: Vec::new(),
            latest_date: Date::default(),
            latest_time: Time::default(),
            available_spots: capacity,
            log_billing: HashMap::new(),
        }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_capacity(&self) -> &i32 {
        &self.capacity
    }

    pub fn get_vehicle_log_exits(&self) -> &Vec<EntryExitLog> {
        &self.logs_cars_exited
    }

    pub fn get_available_spots(&self) -> &i32 {
        &self.available_spots
    }

    pub fn get_vehicle_log_billing(&self) -> &HashMap<Date, Tariff> {
        &self.log_billing
    }

    pub fn eq_name(&self, other: &str) -> bool {
        self.name.eq(other)
    }

    pub fn ne_name(&self, other: &str) -> bool {
        self.name.ne(other)
    }

    pub fn is_park_full(&self) -> bool {
        self.available_spots == 0
    }

    pub fn is_car_in_park(&self, other: &LicencePlate) -> bool {
        self.logs_cars_parked
            .values()
            .any(|log| log.get_licence_plate().eq(other))
    }

    pub fn is_car_not_in_park(&self, other: &LicencePlate) -> bool {
        self.logs_cars_parked
            .values()
            .all(|log: &EntryExitLog| log.get_licence_plate().ne(other))
    }

    pub fn is_date_and_time_before_last_entry_date_and_time(
        &self,
        new_date: &Date,
        new_time: &Time,
    ) -> bool {
        std::cmp::Ordering::Less == match new_date.cmp(&self.latest_date) {
            std::cmp::Ordering::Equal => new_time.cmp(&self.latest_time),
            ord => ord
        }
    }

    // --------------------------------------------------

    pub fn entry(&mut self, licence_plate: LicencePlate, date: Date, time: Time) {
        self.logs_cars_parked.insert(
            licence_plate.to_owned(), 
            EntryExitLog::new(
                licence_plate, 
                date.to_owned(), 
                time.to_owned()
            )
        );

        self.available_spots -= 1;
        self.latest_date = date;
        self.latest_time = time;

        println!("{} {}", self.name, self.get_available_spots());
    }

    pub fn exit(
        &mut self, 
        licence_plate: LicencePlate, 
        date: Date, 
        time: Time
    ) -> Result<(), AppError> {
        let mut log =  self.logs_cars_parked
            .remove(&licence_plate)
            .ok_or(AppError::InvalidVehicleExit { licence_plate })?;

        log.log_exit(
            date.to_owned(), 
            time.to_owned(), 
            &self.tariff_x, 
            &self.tariff_y, 
            &self.tariff_z
        )?;

        let binding = Tariff::default();
        let new_bill = match log.get_bill(){
            Some(tariff) => tariff,
            None => &binding,
        };
        self.log_billing.entry(date.to_owned())
            .and_modify(|bill: &mut Tariff| bill.add_bill(new_bill))
            .or_insert(new_bill.clone());

        self.logs_cars_exited.push(log);

        self.available_spots += 1;
        self.latest_date = date;
        self.latest_time = time;

        Ok(())
    }

    pub fn get_vehicle_log_entries_exits(
        &self, 
        licence_plate: &LicencePlate
    ) -> Vec<&EntryExitLog> {
        let mut logs: Vec<&EntryExitLog> = self.logs_cars_exited
            .iter()
            .filter(|log| log.get_licence_plate().eq(licence_plate))
            .collect();

        if let Some(log) = self.logs_cars_parked.get(licence_plate) {
            logs.push(log);
        }
        logs
    }
}
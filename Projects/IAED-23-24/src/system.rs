use std::process;

use crate::{error::AppError, parking_lot::{Date, EntryExitLog, LicencePlate, ParkingLot, Tariff, Time}, parser::Command, MAX_PARKING_LOTS_IN_SYSTEM};

pub struct System {
    parking_lots: Vec<ParkingLot>
}

impl Default for System {
    fn default() -> Self {
        Self::new()
    }
}

impl System {

    pub fn new() -> Self {
        Self {
            parking_lots: Vec::new(),
        }
    }

    pub fn execute(&mut self, command: Command) -> Result<(), AppError> {
        match command {
            Command::Quit 
                => self.quit(), 
            Command::CreateParkingLot { park_name, capacity, tariff_x, tariff_y, tariff_z } 
                => self.create_parking_lot(park_name, capacity, tariff_x, tariff_y, tariff_z)?,
            Command::ListParkingLots 
                => self.list_parking_lots(),
            Command::RegisterVehicleEntry { park_name, licence_plate, date, time } 
                => self.register_vehicle_entry(park_name, licence_plate, date, time)?,
            Command::RegisterVehicleExit { park_name, licence_plate, date, time } 
                => self.register_vehicle_exit(park_name, licence_plate, date, time)?,
            Command::ListVehicleLogEntriesExits { licence_plate } 
                => self.list_vehicle_log_entries_exits(licence_plate)?,
            Command::ShowParkingLotBilling { park_name } 
                => self.show_parking_lot_billing(park_name)?,
            Command::ShowParkingLotBillingSpecificDate { park_name, date } 
                => self.show_parking_lot_billing_specific_date(park_name, date)?,
            Command::RemoveParkingLot { park_name } 
                => self.remove_parking_lot(park_name)?,
        }

        Ok(())
    }

    /// Terminates the program
    ///
    /// Input format: `q`
    ///
    /// Output format: NONE
    ///
    /// Note: This command has no parameters
    ///
    /// Errors: NONE
    ///
    /// # Example:
    /// ```text
    /// q
    /// ```
    fn quit(&self) {
        process::exit(0);
    }

    /// Creates a parking lot with a billing system
    ///
    /// Input format: `p <park-name> <capacity> <tariff-15-min-before-1-hour> <tariff-15-min-after-1-hour> <tariff-daily-max>`
    ///
    /// Output format: NONE
    ///
    /// Note: The command can be used with one or two arguments
    /// 
    /// Errors:
    /// - `<park-name>: parking already exists.` in case the parking lot name already exists.
    /// - `<capacity>: invalid capacity.` in case the capacity is less than or equal to 0.
    /// - `invalid cost.` in case any of the costs is less than or equal to 0 or if the tariff values are not increasing.
    /// - `too many parks.` in case the number of created parks is at the limit.
    ///
    /// # Example:
    /// ```text
    /// p Saldanha 200 0.20 0.30 12.00
    /// p "CC Colombo" 400 0.25 0.40 20.00
    /// ```
    fn create_parking_lot(
        &mut self, 
        park_name: String, 
        capacity: i32, 
        tariff_x: Tariff, 
        tariff_y: Tariff, 
        tariff_z: Tariff
    ) -> Result<(), AppError> {
        if self.parking_lots.iter().any(|park| park.eq_name(&park_name)) {
            return Err(AppError::ParkingNameAlreadyExists { park_name });
        }

        if capacity <= 0 {
            return Err(AppError::InvalidCapacity { value: capacity });
        }

        if tariff_x.get_value() <= 0.0 
        || tariff_y.get_value() <= 0.0 
        || tariff_z.get_value() <= 0.0 
        || tariff_x.get_value() >= tariff_y.get_value() 
        || tariff_y.get_value() >= tariff_z.get_value() {
            return Err(AppError::InvalidCost);
        }

        if self.parking_lots.len() >= MAX_PARKING_LOTS_IN_SYSTEM {
            return Err(AppError::TooManyParks);
        }

        let park = ParkingLot::new (
            park_name,
            capacity,
            tariff_x,
            tariff_y,
            tariff_z,
        );

        self.parking_lots.push(park);

        Ok(())
    }

    /// Lists existing parking lots
    ///
    /// Input format: `p`
    ///
    /// Output format: `<park-name> <capacity> <available-spots>`, 
    /// in the order of creation of the parking lots.
    ///
    /// Note: The command can be used with one or two arguments
    ///
    /// Errors: NONE
    /// 
    /// # Example:
    /// ```text
    /// p
    /// ```
    fn list_parking_lots(&self) {
        for park in &self.parking_lots {
            println!(
                "{} {} {}", 
                park.get_name(), 
                park.get_capacity(), 
                park.get_available_spots() 
            )
        }
    }

    /// Registers the entry of a vehicle
    ///
    /// Input format: `e <park-name> <licence-plate> <date> <time>`
    ///
    /// Output format: `<park-name> <available-spots>`
    ///
    /// Note: This command requires all parameters to be valid
    ///
    /// Errors:
    /// - `<park-name>: no such parking.` in case the parking lot name does not exist.
    /// - `<park-name>: parking is full.` in case the parking lot is full.
    /// - `<licence-plate>: invalid licence plate.` in case the license plate is invalid.
    /// - `<licence-plate>: invalid vehicle entry.` in case the vehicle is already inside a parking lot.
    /// - `invalid date.` in case the date or time is invalid 
    /// or earlier than the last recorded entry or exit in the system.
    ///
    /// # Example:
    /// ```text
    /// e Saldanha AA-00-AA 01-03-2024 08:34
    /// ```
    fn register_vehicle_entry(
        &mut self, 
        park_name: String, 
        licence_plate: LicencePlate, 
        date: Date, 
        time: Time
    ) -> Result<(), AppError> {
        let park = self.get_mut_park(&park_name)?;

        if park.is_park_full() {
            return Err(AppError::ParkIsFull { park_name });
        }

        if park.is_car_in_park(&licence_plate) {
            return Err(AppError::InvalidVehicleEntry { licence_plate });
        }

        if park.is_date_and_time_before_last_entry_date_and_time(&date, &time) {
            return Err(AppError::InvalidDate);
        }

        park.entry(licence_plate, date, time);

        // println!() handled by ParkingLot::entry(), as there is needed more access to data

        Ok(())
    }

    /// Registers the exit of a vehicle
    ///
    /// Input format: `s <park-name> <licence-plate> <date> <time>`
    ///
    /// Output format: `<licence-plate> <date-entrance> <time-entrance> <date-exit> <time-exit> <value-paid>`
    ///
    /// Note: This command requires all parameters to be valid
    ///
    /// Errors:
    /// - `<park-name>: no such parking.` in case the parking lot name does not exist.
    /// - `<licence-plate>: invalid licence plate.` in case the license plate is invalid.
    /// - `<licence-plate>: invalid vehicle exit.` in case the vehicle is not inside the specified parking lot.
    /// - `invalid date.` in case the date or time is invalid or earlier than the last recorded entry or exit in the system.
    ///
    /// # Example:
    /// ```text
    /// s Saldanha AA-00-AA 01-03-2024 10:59
    /// ```
    fn register_vehicle_exit(
        &mut self, 
        park_name: String, 
        licence_plate: LicencePlate, 
        date: Date, 
        time: Time
    ) -> Result<(), AppError> {
        let park = self.get_mut_park(&park_name)?;

        if park.is_car_not_in_park(&licence_plate) {
            return Err(AppError::InvalidVehicleExit { licence_plate });
        }

        if park.is_date_and_time_before_last_entry_date_and_time(&date, &time) {
            return Err(AppError::InvalidDate);
        }

        park.exit(licence_plate, date, time)?;

        // println!() handled by EntryExitLog::log_exit(), as there is needed more access to data

        Ok(())
    }

    /// Lists the entries and exits of a vehicle
    ///
    /// Input format: `v <licence-plate>`
    ///
    /// Output format: `<park-name> <date-entrance> <time-entrance> <date-exit> <time-exit>`, 
    /// ordered first by parking lot name and then by entry date and time. 
    /// If the vehicle is inside a parking lot, the exit date and time associated with that entry are not shown.
    ///
    /// Note: This command requires a valid license plate
    ///
    /// Errors:
    /// - `<licence-plate>: invalid licence plate.` in case the license plate is invalid.
    /// - `<licence-plate>: no entries found in any parking.` in case the license plate is valid, 
    /// but there are no records of entries in parking lots.
    ///
    /// # Example:
    /// ```text
    /// v AA-00-AA
    /// ```
    fn list_vehicle_log_entries_exits(
        &self, 
        licence_plate: LicencePlate
    ) -> Result<(), AppError> {
        let parking_lot_logs: Vec<(&String, Vec<&EntryExitLog>)> = self.parking_lots
            .iter()
            .map(
                |park| 
                (park.get_name(), park.get_vehicle_log_entries_exits(&licence_plate))
            ).collect();

        if parking_lot_logs.is_empty() {
            return Err(AppError::NoEntriesFoundInAnyParking { licence_plate });
        }

        for (parkname, logs) in parking_lot_logs {
            for log in logs {
                print!(
                    "{} {} {}",
                    parkname, log.get_date_entry(), log.get_time_entry()
                );
                if let (Some(date_exit), Some(time_exit)) 
                    = (log.get_date_exit(), log.get_time_exit()) {
                    print!(" {} {}", date_exit, time_exit);
                }
                println!();
            }
        }

        Ok(())
    }

    /// Shows the billing of a parking lot
    ///
    /// Input format: `f <park-name>`
    ///
    /// Output format: `<date> <value-bill>`, ordered by date. 
    /// This option shows the daily billing summary of the parking lot.
    ///
    /// Note: The command can be used with one or two arguments
    ///
    /// Errors:
    /// - `<park-name>: no such parking.` in case the parking lot name does not exist.
    ///
    /// # Example:
    /// ```text
    /// f Saldanha
    /// ```
    fn show_parking_lot_billing(&self, park_name: String) -> Result<(), AppError> {
        let park = self.get_park(&park_name)?;

        for (date, bill) in park.get_vehicle_log_billing() {
            println!("{} {}", date, bill);
        }

        Ok(())
    }

    /// Shows the billing of a parking lot on a specific date
    ///
    /// Input format: `f <park-name> <date>`
    ///
    /// Output format: `<licence-plate> <time-exit> <value-paid>`, ordered by exit date. 
    /// This option shows the list of billed amounts on a specific day.
    ///
    /// Note: The command can be used with one or two arguments
    ///
    /// Errors:
    /// - `<park-name>: no such parking.` in case the parking lot name does not exist.
    /// - `invalid date.` in case the date is invalid or later than the last recorded entry or exit in the system.
    ///
    /// # Example:
    /// ```text
    /// f Saldanha 01-03-2024
    /// ```
    fn show_parking_lot_billing_specific_date(
        &self, 
        park_name: String, 
        date: Date
    ) -> Result<(), AppError> {
        let park = self.get_park(&park_name)?;

        let logs: Vec<&EntryExitLog> = park.get_vehicle_log_exits()
            .iter()
            .filter(|log| log.get_date_exit().is_some_and(|log_date| log_date.eq(&date)))
            .collect();

        for log in logs {
            if let (Some(time), Some(bill)) = (log.get_time_exit(), log.get_bill()) {
                println!("{} {} {}", log.get_licence_plate(), time, bill);
            }
        }

        Ok(())
    }

    /// Removes a parking lot from the system and all vehicle entries and exits from that parking lot
    ///
    /// Input format: `r <park-name>`
    ///
    /// Output format: `<park-name>`, ordered by the parking lot name.
    ///
    /// Note: This command requires a valid parking lot name
    ///
    /// Errors:
    /// - `<park-name>: no such parking.` in case the parking lot name does not exist.
    ///
    /// # Example:
    /// ```text
    /// r "CC Colombo"
    /// ```
    fn remove_parking_lot(&mut self, park_name: String) -> Result<(), AppError> {
        match self.parking_lots.iter().any(|park| park.eq_name(&park_name)) {
            true => {
                self.parking_lots.retain(|park| park.ne_name(&park_name));
                Ok(())
            },
            false => Err(AppError::NoSuchParking { park_name })
        }
    }

    fn get_park(&self, park_name: &String) -> Result<&ParkingLot, AppError> {
        self.parking_lots
            .iter()
            .find(|park| park.eq_name(park_name))
            .ok_or(AppError::NoSuchParking { park_name: park_name.to_owned() })
    }

    fn get_mut_park(&mut self, park_name: &String) -> Result<&mut ParkingLot, AppError> {
        self.parking_lots
            .iter_mut()
            .find(|park| park.eq_name(park_name))
            .ok_or(AppError::NoSuchParking { park_name: park_name.to_owned() })
    }
}
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
fn quit() {

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
fn create_parking_lot() {

}

/// Lists existing parking lots
///
/// Input format: `p`
///
/// Output format: `<park-name> <capacity> <available-spots>`, in the order of creation of the parking lots.
///
/// Note: The command can be used with one or two arguments
///
/// # Example:
/// ```text
/// p
/// ```
fn list_parking_lots() {

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
/// - `invalid date.` in case the date or time is invalid or earlier than the last recorded entry or exit in the system.
///
/// # Example:
/// ```text
/// e Saldanha AA-00-AA 01-03-2024 08:34
/// ```
fn register_vehicle_entry() {

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
fn register_vehicle_exit() {

}

/// Lists the entries and exits of a vehicle
///
/// Input format: `v <licence-plate>`
///
/// Output format: `<park-name> <date-entrance> <time-entrance> <date-exit> <time-exit>`, ordered first by parking lot name and then by entry date and time. If the vehicle is inside a parking lot, the exit date and time associated with that entry are not shown.
///
/// Note: This command requires a valid license plate
///
/// Errors:
/// - `<licence-plate>: invalid licence plate.` in case the license plate is invalid.
/// - `<licence-plate>: no entries found in any parking.` in case the license plate is valid, but there are no records of entries in parking lots.
///
/// # Example:
/// ```text
/// v AA-00-AA
/// ```
fn list_vehicle_log_entries_exits() {

}

/// Shows the billing of a parking lot
///
/// Input format: `f <park-name>`
///
/// Output format: `<date> <value-bill>`, ordered by date. This option shows the daily billing summary of the parking lot.
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
fn show_parking_lot_billing() {

}

/// Shows the billing of a parking lot on a specific date
///
/// Input format: `f <park-name> <date>`
///
/// Output format: `<licence-plate> <time-exit> <value-paid>`, ordered by exit date. This option shows the list of billed amounts on a specific day.
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
fn show_parking_lot_billing_specific_date() {

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
fn remove_parking_lot() {

}
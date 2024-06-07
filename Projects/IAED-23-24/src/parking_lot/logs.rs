use std::cmp::Ordering;

use crate::{error::AppError, parking_lot::date::get_num_days_for_mounth, RANGE_VALID_HOURS, RANGE_VALID_MINUTES, RANGE_VALID_MONTHS, RANGE_VALID_YEARS};

use super::{Date, LicencePlate, Tariff, Time};

#[derive(Debug)]
pub struct EntryExitLog {
    licence_plate: LicencePlate,
    date_entry: Date,
    time_entry: Time,
    date_exit: Option<Date>,
    time_exit: Option<Time>,
    bill: Option<Tariff>,
}

impl EntryExitLog {
    pub fn new(
        licence_plate: LicencePlate,
        date_entry: Date,
        time_entry: Time,
    ) -> Self {
        EntryExitLog {
            licence_plate,
            date_entry,
            time_entry,
            date_exit: None,
            time_exit: None,
            bill: None,
        }
    }

    pub fn get_date_entry(&self) -> &Date {
        &self.date_entry
    }

    pub fn get_time_entry(&self) -> &Time {
        &self.time_entry
    }

    pub fn get_date_exit(&self) -> Option<&Date> {
        self.date_exit.as_ref()
    }

    pub fn get_time_exit(&self) -> Option<&Time> {
        self.time_exit.as_ref()
    }

    pub fn get_licence_plate(&self) -> &LicencePlate {
        &self.licence_plate
    }

    pub fn get_bill(&self) -> &Option<Tariff> {
        &self.bill
    }

    pub fn log_exit(
        &mut self, 
        date_exit: Date, 
        time_exit: Time,
        tariff_x: &Tariff,
        tariff_y: &Tariff,
        tariff_z: &Tariff
    ) -> Result<(), AppError> {
        self.date_exit = Some(date_exit.to_owned());
        self.time_exit = Some(time_exit.to_owned());

        let bill = self.calculate_bill(
            tariff_x,
            tariff_y,
            tariff_z
        )?;

        println!(
            "{} {} {} {} {} {}",
            self.licence_plate,
            self.date_entry,
            self.time_entry,
            date_exit,
            time_exit,
            bill
        );

        self.bill = Some(bill);

        Ok(())
    }

    fn calculate_bill(
        &self,
        tariff_x: &Tariff,
        tariff_y: &Tariff,
        tariff_z: &Tariff
    ) -> Result<Tariff, AppError> {
        let tariff_x = tariff_x.get_value();
        let tariff_y = tariff_y.get_value();
        let tariff_z = tariff_z.get_value();
        let (days, intervals_15_mins) = self.get_days_and_15_mins_interval()?;

        let mut bill = days as f32 * tariff_z;
        let day_bill = match intervals_15_mins {
            intervals if (0..4).contains(&intervals) => {
                intervals as f32 * tariff_x
            },
            mut intervals => {
                intervals -= 4;
                4.0 * tariff_x + intervals as f32 * tariff_y
            },
        };

        bill += day_bill.min(tariff_z);

        Ok(Tariff::new(bill))
    }

    fn get_days_and_15_mins_interval(&self) -> Result<(i32, i32), AppError> {
        let date_entry  = &self.date_entry;
        let time_entry  = &self.time_entry;
        let date_exit   = match &self.date_exit {
            Some(date) => date,
            None => return Err(AppError::InvalidVehicleExit { 
                licence_plate: self.licence_plate.to_owned() 
            }),
        };
        let time_exit   = match &self.time_exit {
            Some(time) => time,
            None => return Err(AppError::InvalidVehicleExit { 
                licence_plate: self.licence_plate.to_owned() 
            }),
        };

        let mut carry = 0;

        let mut years = calculate_time_interval(
            date_entry.get_year(), 
            date_exit.get_year(), 
            &mut carry, 
            RANGE_VALID_YEARS.end
        );
        if carry != 0 {
            return Err(AppError::InvalidVehicleExit { 
                licence_plate: self.licence_plate.to_owned() 
            });
        }

        let mut carry = 0;

        let mut days = calculate_time_interval(
            date_entry.get_day(), 
            date_exit.get_day(), 
            &mut carry, 
            get_num_days_for_mounth(date_exit.get_month())
        );

        let days_in_months = calculate_days_in_month_interval(
            date_entry.get_month(), 
            date_exit.get_month(), 
            &mut years, 
            carry
        );

        let mut hours = calculate_time_interval(
            time_entry.get_hour(), 
            time_exit.get_hour(), 
            &mut days, 
            RANGE_VALID_HOURS.end
        );

        let minutes = calculate_time_interval(
            time_entry.get_minute(), 
            time_exit.get_minute(), 
            &mut hours, 
            RANGE_VALID_MINUTES.end
        );

        let days = years * 365 + days_in_months + days;
        let interval_15_mins = hours * 4 + minutes / 14;

        Ok((days, interval_15_mins))
    }
}

fn calculate_time_interval(
    start_time: i32, 
    end_time: i32, 
    carry: &mut i32, 
    max_time: i32,
) -> i32 {
    match start_time.cmp(&end_time) {
        Ordering::Less => end_time - start_time,
        Ordering::Equal => 0,
        Ordering::Greater => {
            *carry -= 1;
            max_time - start_time + end_time
        },
    }
}

fn calculate_days_in_month_interval(
    start_month: i32, 
    end_month: i32, 
    carry_years: &mut i32, 
    carry_months: i32 
) -> i32 {
    let end_month = end_month + carry_months;

    match start_month.cmp(&end_month) {
        Ordering::Less => (start_month..end_month)
            .map(get_num_days_for_mounth)
            .sum(),
        Ordering::Equal => 0,
        Ordering::Greater => {
            *carry_years -= 1;

            (start_month..*RANGE_VALID_MONTHS.end())
                .map(get_num_days_for_mounth)
                .sum::<i32>()
            + 
            (*RANGE_VALID_MONTHS.start()..end_month)
                .map(get_num_days_for_mounth)
                .sum::<i32>()
        }
    }
}
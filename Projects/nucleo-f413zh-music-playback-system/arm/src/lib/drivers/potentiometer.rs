use embassy_time::{Duration, Instant};
use shared_lib::traits::{AnalogInput, PotentiometerDriver};

use super::ADC_RESOLUTION;

// Potentiometer is now completely independent of any specific ADC implementation
pub struct Potentiometer<I: AnalogInput> {
    input: I,
}

impl<I: AnalogInput> Potentiometer<I> {
    pub fn new(input: I) -> Self {
        Self {
            input
        }
    }

    /// Calculate percentage from raw value and max ADC count
    fn calculate_percentage(raw_value: u32, max_value: u32) -> u8 {
        ((raw_value as f32 / max_value as f32) * 100.0) as u8
    }
}

impl<I: AnalogInput> PotentiometerDriver for Potentiometer<I> {
    /// Read the potentiometer position as percentage
    fn read_raw_value(&mut self) -> u16 {
        self.input.read_value()
    }

    /// Read the potentiometer position as percentage
    fn read_position(&mut self) -> u8 {
        let raw_value = self.read_raw_value();
        Self::calculate_percentage(raw_value as u32, ADC_RESOLUTION.to_max_count())
    }
}


static ZERO:    u8 = 0;
static FIFTY:   u8 = 50;
static HUNDRED: u8 = 100;

impl<I: AnalogInput> Potentiometer<I> {
    pub fn test(&mut self) {
        crate::test!("Initiating Potentiometer Unit Test");

        Self::test_calculate_percentage();

        self.test_potentiometer_range();

        crate::test!("Test completed")
    }

    fn test_calculate_percentage() {
        crate::test!("Testing calculate_percentage");

        crate::assert_eq!(Self::calculate_percentage(0, 100), 0);
        crate::assert_eq!(Self::calculate_percentage(50, 100), 50);
        crate::assert_eq!(Self::calculate_percentage(100, 100), 100);
        crate::assert_eq!(Self::calculate_percentage(25, 100), 25);
        crate::assert_eq!(Self::calculate_percentage(75, 100), 75);
        crate::assert_eq!(Self::calculate_percentage(100, 200), 50);
        crate::assert_eq!(Self::calculate_percentage(150, 200), 75);
        crate::assert_eq!(Self::calculate_percentage(200, 200), 100);
        crate::assert_eq!(Self::calculate_percentage(0, 1), 0);
        crate::assert_eq!(Self::calculate_percentage(1, 1), 100);
        crate::assert_eq!(Self::calculate_percentage(150, 100), 150);
        crate::assert_eq!(Self::calculate_percentage(200, 100), 200);
        crate::assert_eq!(Self::calculate_percentage(250, 100), 250);
        crate::assert_eq!(Self::calculate_percentage(300, 100), u8::MAX);
        crate::assert_eq!(Self::calculate_percentage(400, 200), 200);
        crate::assert_eq!(Self::calculate_percentage(500, 200), 250);
        crate::assert_eq!(Self::calculate_percentage(600, 200), u8::MAX);

        crate::test!("Test successful");
    }

    fn test_potentiometer_range(&mut self) {
        crate::test!("Please set the potentiometer to the minimum");
        self.test_potentiometer_position(ZERO);

        crate::test!("Please set the potentiometer to the middle");
        self.test_potentiometer_position(FIFTY);

        crate::test!("Please set the potentiometer to the maximum");
        self.test_potentiometer_position(HUNDRED);
    }

    fn test_potentiometer_position(
        &mut self, 
        position: u8
    ) {
        let mut start = Instant::now();
        let timeout = Duration::from_secs(3);
        while self.read_position() != position {
            if Instant::now() - start > timeout {
                crate::test!(
                    "(Debug) raw adc value: {}, adc value: {}", 
                    self.read_raw_value(),
                    self.read_position()
                );
                start = Instant::now();
            }
        }
    }
}
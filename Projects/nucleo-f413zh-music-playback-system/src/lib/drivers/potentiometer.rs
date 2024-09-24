use defmt::{debug, trace};
use embassy_stm32::adc::{self, AdcPin};
use embassy_time::{Duration, Instant};

use super::{AdcManager, ADC_RESOLUTION};

// Potentiometer is now completely independent of any specific ADC implementation
pub struct Potentiometer<'a, 'b, T, P> 
where 
    T: adc::Instance,
    P: AdcPin<T>
{
    adc_manager: &'a mut AdcManager<'b, T>,
    pin: P,
}

impl<'a, 'b, T, P> Potentiometer<'a, 'b, T, P> 
where 
    T: adc::Instance,
    P: AdcPin<T>
{
    pub fn new(adc_manager: &'a mut AdcManager<'b, T>, pin: P) -> Self {
        Self {
            adc_manager,
            pin
        }
    }

    /// Read the potentiometer position as percentage
    fn read_raw_value(&mut self) -> u16 {
        self.adc_manager.read_pin(&mut self.pin)
    }

    /// Read the potentiometer position as percentage
    fn read_position(&mut self) -> u8 {
        let raw_value = self.read_raw_value();
        Self::calculate_percentage(raw_value as u32, ADC_RESOLUTION.to_max_count())
    }

    /// Calculate percentage from raw value and max ADC count
    fn calculate_percentage(raw_value: u32, max_value: u32) -> u8 {
        ((raw_value as f32 / max_value as f32) * 100.0) as u8
    }
}


static ZERO:    u8 = 0;
static FIFTY:   u8 = 50;
static HUNDRED: u8 = 100;

impl<'a, 'b, T, P> Potentiometer<'a, 'b, T, P> 
where 
    T: adc::Instance,
    P: AdcPin<T>
{
    pub fn test(&mut self) {
        debug!("Initiating Potentiometer Unit Test");

        Self::test_calculate_percentage();

        self.test_potentiometer_range();

        debug!("Test completed")
    }

    fn test_calculate_percentage() {
        debug!("Testing calculate_percentage");

        assert_eq!(Self::calculate_percentage(0, 100), 0);
        assert_eq!(Self::calculate_percentage(50, 100), 50);
        assert_eq!(Self::calculate_percentage(100, 100), 100);
        assert_eq!(Self::calculate_percentage(25, 100), 25);
        assert_eq!(Self::calculate_percentage(75, 100), 75);
        assert_eq!(Self::calculate_percentage(100, 200), 50);
        assert_eq!(Self::calculate_percentage(150, 200), 75);
        assert_eq!(Self::calculate_percentage(200, 200), 100);
        assert_eq!(Self::calculate_percentage(0, 1), 0);
        assert_eq!(Self::calculate_percentage(1, 1), 100);
        assert_eq!(Self::calculate_percentage(150, 100), 150);
        assert_eq!(Self::calculate_percentage(200, 100), 200);
        assert_eq!(Self::calculate_percentage(250, 100), 250);
        assert_eq!(Self::calculate_percentage(300, 100), u8::MAX);
        assert_eq!(Self::calculate_percentage(400, 200), 200);
        assert_eq!(Self::calculate_percentage(500, 200), 250);
        assert_eq!(Self::calculate_percentage(600, 200), u8::MAX);

        debug!("Test successful");
    }

    fn test_potentiometer_range(&mut self) {
        debug!("Please set the potentiometer to the minimum");
        self.test_potentiometer_position(ZERO);

        debug!("Please set the potentiometer to the middle");
        self.test_potentiometer_position(FIFTY);

        debug!("Please set the potentiometer to the maximum");
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
                trace!(
                    "raw adc value: {}, adc value: {}", 
                    self.read_raw_value(),
                    self.read_position()
                );
                start = Instant::now();
            }
        }
    }
}
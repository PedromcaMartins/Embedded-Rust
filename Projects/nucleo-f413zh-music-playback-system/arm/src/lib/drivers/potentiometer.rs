use core::marker::PhantomData;

use embassy_stm32::adc::{self, AdcPin};
use embassy_time::{Duration, Instant};

use super::AdcManager;

static ZERO:    u8 = 0;
static FIFTY:   u8 = 50;
static HUNDRED: u8 = 100;

pub struct Potentiometer<T: adc::Instance, P: AdcPin<T>> {
    pin: P,
    _adc_instance: PhantomData<T>
}

impl<T: adc::Instance, P: AdcPin<T>> Potentiometer<T, P> {
    pub fn new(pin: P) -> Self {
        Self {
            pin,
            _adc_instance: PhantomData
        }
    }

    pub fn read_raw_value(
        &mut self, 
        adc_manager: &mut AdcManager<'_, T>
    ) -> u16 {
        adc_manager.read_pin(&mut self.pin)
    }

    pub fn read_position(
        &mut self, 
        adc_manager: &mut AdcManager<'_, T>
    ) -> u8 {
        let raw_value = self.read_raw_value(adc_manager);

        let max_adc_value = adc_manager.get_max_count();

        Self::get_percentage(raw_value as u32, max_adc_value)
    }

    fn get_percentage(nominator: u32, denominator: u32) -> u8 {
        ((nominator as f32 / denominator as f32) * 100.0) as u8

    }
}


impl<T: adc::Instance, P: AdcPin<T>> Potentiometer<T, P> {
    pub fn test(&mut self, adc_manager: &mut AdcManager<'_, T>) {
        crate::test!("Initiating Button Pooling Unit Test");

        Self::test_get_percentage();

        self.test_potentiometer_range(adc_manager);

        crate::test!("Test completed")
    }

    fn test_get_percentage() {
        crate::test!("Testing get_percentage");

        crate::assert_eq!(Self::get_percentage(0, 100), 0);
        crate::assert_eq!(Self::get_percentage(50, 100), 50);
        crate::assert_eq!(Self::get_percentage(100, 100), 100);
        crate::assert_eq!(Self::get_percentage(25, 100), 25);
        crate::assert_eq!(Self::get_percentage(75, 100), 75);
        crate::assert_eq!(Self::get_percentage(100, 200), 50);
        crate::assert_eq!(Self::get_percentage(150, 200), 75);
        crate::assert_eq!(Self::get_percentage(200, 200), 100);
        crate::assert_eq!(Self::get_percentage(0, 1), 0);
        crate::assert_eq!(Self::get_percentage(1, 1), 100);
        crate::assert_eq!(Self::get_percentage(150, 100), 150);
        crate::assert_eq!(Self::get_percentage(200, 100), 200);
        crate::assert_eq!(Self::get_percentage(250, 100), 250);
        crate::assert_eq!(Self::get_percentage(300, 100), u8::MAX);
        crate::assert_eq!(Self::get_percentage(400, 200), 200);
        crate::assert_eq!(Self::get_percentage(500, 200), 250);
        crate::assert_eq!(Self::get_percentage(600, 200), u8::MAX);

        crate::test!("Test successful");
    }

    fn test_potentiometer_range(&mut self, adc_manager: &mut AdcManager<'_, T>) {
        crate::test!("Please set the potentiometer to the minimum");
        self.test_potentiometer_position(adc_manager, ZERO);

        crate::test!("Please set the potentiometer to the middle");
        self.test_potentiometer_position(adc_manager, FIFTY);

        crate::test!("Please set the potentiometer to the maximum");
        self.test_potentiometer_position(adc_manager, HUNDRED);
    }

    fn test_potentiometer_position(
        &mut self, 
        adc_manager: &mut AdcManager<'_, T>, 
        position: u8
    ) {
        let mut start = Instant::now();
        let timeout = Duration::from_secs(3);
        while self.read_position(adc_manager) != position {
            if Instant::now() - start > timeout {
                crate::test!(
                    "(Debug) raw adc value: {}, adc value: {}", 
                    self.read_raw_value(adc_manager),
                    self.read_position(adc_manager)
                );
                start = Instant::now();
            }
        }
    }
}
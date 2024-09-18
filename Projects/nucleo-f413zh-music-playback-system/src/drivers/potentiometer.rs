use core::marker::PhantomData;

use defmt::info;
use embassy_stm32::adc::{self, AdcPin};
use embassy_time::{Duration, Instant};

use crate::percentage::Percentage;

use super::AdcManager;

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
    ) -> Percentage {
        let raw_value = self.read_raw_value(adc_manager);

        let max_adc_value = adc_manager.get_max_count();

        Percentage::from_ratio_u32(raw_value as u32, max_adc_value)
    }
}


impl<T: adc::Instance, P: AdcPin<T>> Potentiometer<T, P> {
    pub fn test(&mut self, adc_manager: &mut AdcManager<'_, T>) {
        info!("Initiating Button Pooling Unit Test");

        self.test_potentiometer_range(adc_manager);

        info!("Test completed")
    }

    fn test_potentiometer_range(&mut self, adc_manager: &mut AdcManager<'_, T>) {
        info!("Please set the potentiometer to the minimum");
        self.test_potentiometer_position(adc_manager, Percentage::zero());

        info!("Please set the potentiometer to the middle");
        self.test_potentiometer_position(adc_manager, Percentage::fifty());

        info!("Please set the potentiometer to the maximum");
        self.test_potentiometer_position(adc_manager, Percentage::hundred());
    }

    fn test_potentiometer_position(
        &mut self, 
        adc_manager: &mut AdcManager<'_, T>, 
        position: Percentage
    ) {
        let mut start = Instant::now();
        let timeout = Duration::from_secs(3);
        while self.read_position(adc_manager) != position {
            if Instant::now() - start > timeout {
                info!(
                    "(Debug) raw adc value: {}, adc value: {}", 
                    self.read_raw_value(adc_manager),
                    self.read_position(adc_manager).value()
                );
                start = Instant::now();
            }
        }
    }
}
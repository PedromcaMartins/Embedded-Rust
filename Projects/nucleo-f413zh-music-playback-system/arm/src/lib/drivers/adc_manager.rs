use embassy_stm32::adc::{self, Adc, AdcPin, Resolution};
use shared_lib::traits::AnalogInput;

pub static ADC_RESOLUTION: Resolution = Resolution::TwelveBit;

pub struct AdcManager<'d, T: adc::Instance> {
    adc: Adc<'d, T>,
}

impl<'d, T: adc::Instance> AdcManager<'d, T> {
    pub fn new(mut adc: Adc<'d, T>) -> Self {
        adc.set_resolution(ADC_RESOLUTION);

        Self {
            adc,
        }
    }

    pub fn read_pin<P: AdcPin<T>>(&mut self, pin: &mut P) -> u16 {
        self.adc.read(pin)
    }

    pub fn get_max_count(&self) -> u32 {
        ADC_RESOLUTION.to_max_count()
    }
}

// Implement AnalogInput for any type of AdcPin (internally used)
pub struct AdcPinInput<'a, 'b, P: AdcPin<T>, T: adc::Instance> {
    pin: &'a mut P,
    adc_manager: &'a mut AdcManager<'b, T>,
}

impl<'a, 'b, P: AdcPin<T>, T: adc::Instance> AdcPinInput<'a, 'b, P, T> {
    pub fn new(pin: &'a mut P, adc_manager: &'a mut AdcManager<'b, T>) -> Self {
        Self {
            pin,
            adc_manager
        }
    }
}

impl<'a, 'b, P: AdcPin<T>, T: adc::Instance> AnalogInput for AdcPinInput<'a, 'b, P, T> {
    fn read_value(&mut self) -> u16 {
        self.adc_manager.read_pin(self.pin)
    }
}


impl<'a, 'b, P: AdcPin<T>, T: adc::Instance> AdcPinInput<'a, 'b, P, T> {
    pub fn test(&mut self) {
        crate::test!("Initiating Adc Manager Unit Test");

        self.test_read_value();

        crate::test!("Test completed");
    }

    fn test_read_value(&mut self) {
        // Test reading a value from the pin
        let value = self.read_value();
        let max_count = self.adc_manager.get_max_count();

        crate::debug_assert!(value as u32 <= max_count, "Value {} exceeds max count {}", value, max_count);
    }
}
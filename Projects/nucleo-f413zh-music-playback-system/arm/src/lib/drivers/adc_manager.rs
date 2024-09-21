use embassy_stm32::adc::{self, Adc, AdcPin, Resolution};
use shared_lib::traits::AnalogInput;

pub static ADC_RESOLUTION: Resolution = Resolution::TwelveBit;

/* -------------------------------------------------------------------------- */
/*                                 Adc Manager                                */
/* -------------------------------------------------------------------------- */

pub struct AdcManager<'d, T>
where
    T: adc::Instance
{
    adc: Adc<'d, T>,
}

impl<'d, T> AdcManager<'d, T>
where
    T: adc::Instance
{
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

/* -------------------------------------------------------------------------- */
/*                                Adc Pin Input                               */
/* -------------------------------------------------------------------------- */

// Implement AnalogInput for any type of AdcPin (internally used)
pub struct AdcPinInput<'a, 'b, T, P> 
where 
    T: adc::Instance,
    P: AdcPin<T>
{
    pin: &'a mut P,
    adc_manager: &'a mut AdcManager<'b, T>,
}

impl<'a, 'b, T, P> AdcPinInput<'a, 'b, T, P> 
where 
    T: adc::Instance,
    P: AdcPin<T>
{
    pub fn new(pin: &'a mut P, adc_manager: &'a mut AdcManager<'b, T>) -> Self {
        Self {
            pin,
            adc_manager
        }
    }
}

impl<'a, 'b, T, P> AnalogInput for AdcPinInput<'a, 'b, T, P> 
where 
    T: adc::Instance,
    P: AdcPin<T>
{
    fn read_value(&mut self) -> u16 {
        self.adc_manager.read_pin(self.pin)
    }
}


impl<'a, 'b, T, P> AdcPinInput<'a, 'b, T, P> 
where 
    T: adc::Instance,
    P: AdcPin<T>
{
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
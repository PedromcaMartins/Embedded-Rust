use embassy_stm32::adc::{self, Adc, AdcPin, Resolution};

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

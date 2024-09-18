use embassy_stm32::adc::{self, Adc, AdcPin, Resolution};

pub struct AdcManager<'d, T: adc::Instance> {
    adc: Adc<'d, T>,
    resolution: Resolution
}

static RESOLUTION: Resolution = Resolution::TwelveBit;

impl<'d, T: adc::Instance> AdcManager<'d, T> {
    pub fn new(mut adc: Adc<'d, T>) -> Self {
        adc.set_resolution(RESOLUTION);

        Self {
            adc,
            resolution: RESOLUTION
        }
    }

    pub fn read_pin<P: AdcPin<T>>(&mut self, pin: &mut P) -> u16 {
        self.adc.read(pin)
    }

    pub fn get_max_count(&self) -> u32 {
        self.resolution.to_max_count()
    }
}
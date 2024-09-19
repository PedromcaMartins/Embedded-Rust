use embassy_stm32::adc::Adc;
use embassy_stm32::exti::ExtiInput;
use embassy_stm32::gpio::Input;
use embassy_stm32::gpio::Output;
use embassy_stm32::peripherals;
use embassy_stm32::gpio;
use embassy_stm32::peripherals::ADC1;
use embassy_time::Delay;

pub struct IOMapping<'d> {
    pub led: Output<'d, peripherals::PB7>, 
    pub led_default_level: gpio::Level, 
    pub potentiometer_adc: Adc<'d, ADC1>,
    pub potentiometer_pin: peripherals::PA3,
}

impl<'d> IOMapping<'d> {
    pub fn init(p: embassy_stm32::Peripherals) -> Self {
        Self {
            led: Output::new(p.PB7, gpio::Level::Low, gpio::Speed::Low),
            led_default_level: gpio::Level::Low,
            potentiometer_adc: Adc::new(p.ADC1, &mut Delay),
            potentiometer_pin: p.PA3
        }
    }
}

use embassy_stm32::adc::Adc;
use embassy_stm32::exti::ExtiInput;
use embassy_stm32::gpio::Input;
use embassy_stm32::gpio::Output;
use embassy_stm32::peripherals;
use embassy_stm32::gpio;
use embassy_stm32::peripherals::ADC1;
use embassy_stm32::peripherals::PA3;
use embassy_stm32::peripherals::PB7;
use embassy_stm32::peripherals::PC13;
use embassy_time::Delay;

mod types {
    use embassy_stm32::peripherals::ADC1;
    use embassy_stm32::peripherals::PA3;
    use embassy_stm32::peripherals::PB7;
    use embassy_stm32::peripherals::PC13;

    pub type LedPin = PB7;
    pub type ButtonInterruptPin = PC13;
    pub type PotentiometerAdc = ADC1;
    pub type PotentiometerPin = PA3;
}

use types::*;

pub struct IOMappingTest<'d> {
    pub led: Output<'d, LedPin>, 
    pub led_default_level: gpio::Level, 
    pub button: ExtiInput<'d, ButtonInterruptPin>,
    pub button_default_level: gpio::Level,
    pub potentiometer_adc: Adc<'d, PotentiometerAdc>,
    pub potentiometer_pin: PotentiometerPin,
}

impl<'d> IOMappingTest<'d> {
    pub fn init(p: embassy_stm32::Peripherals) -> Self {
        Self {
            led: Output::new(p.PB7, gpio::Level::Low, gpio::Speed::Low),
            led_default_level: gpio::Level::Low,
            button: ExtiInput::new(Input::new(p.PC13, gpio::Pull::None), p.EXTI13),
            button_default_level: gpio::Level::Low,
            potentiometer_adc: Adc::new(p.ADC1, &mut Delay),
            potentiometer_pin: p.PA3
        }
    }
}

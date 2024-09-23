use embassy_stm32::gpio::Input;
use embassy_stm32::gpio::Output;
use embassy_stm32::peripherals;
use embassy_stm32::gpio;

pub struct IOMapping<'d> {
    pub led: Output<'d, peripherals::PB7>, 
    pub led_default_level: gpio::Level, 
    pub button: Input<'d, peripherals::PC13>,
    pub button_default_level: gpio::Level,
}

impl<'d> IOMapping<'d> {
    pub fn init(p: embassy_stm32::Peripherals) -> Self {
        Self {
            led: Output::new(p.PB7, gpio::Level::Low, gpio::Speed::Low),
            led_default_level: gpio::Level::Low,
            button: Input::new(p.PC13, gpio::Pull::None),
            button_default_level: gpio::Level::Low,
        }
    }
}

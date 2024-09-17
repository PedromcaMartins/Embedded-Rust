use embassy_stm32::exti::ExtiInput;
use embassy_stm32::gpio::Input;
use embassy_stm32::gpio::Output;
use embassy_stm32::peripherals;
use embassy_stm32::gpio;

pub struct IOMapping<'d> {
    // pub playback_status_led: Output<'d, peripherals::PB7>, 
    // pub playback_status_led_default_level: gpio::Level, 
    // pub playback_pause_play_button: Input<'d, peripherals::PC13>,
    // pub playback_pause_play_button_default_level: gpio::Level,
    pub playback_pause_play_button: ExtiInput<'d, peripherals::PC13>,
    pub playback_pause_play_button_default_level: gpio::Level,
}

impl<'d> IOMapping<'d> {
    pub fn init(p: embassy_stm32::Peripherals) -> Self {
        Self {
            // playback_status_led: Output::new(p.PB7, gpio::Level::Low, gpio::Speed::Low),
            // playback_status_led_default_level: gpio::Level::Low,
            // playback_pause_play_button: Input::new(p.PC13, gpio::Pull::None),
            // playback_pause_play_button_default_level: gpio::Level::Low,
            playback_pause_play_button: ExtiInput::new(Input::new(p.PC13, gpio::Pull::None), p.EXTI13),
            playback_pause_play_button_default_level: gpio::Level::Low
        }
    }
}

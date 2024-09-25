use core::marker::PhantomData;

use embassy_stm32::gpio::{Input, Level, Pin};
use embassy_time::Duration;

use super::{Button, ButtonMode};

pub struct PollingInput<'d, T: Pin> {
    input: Input<'d, T>,
    default_level: Level
}

impl<'d, T: Pin> ButtonMode for PollingInput<'d, T> {
    fn is_pressed_down(&self) -> bool {
        self.input.get_level() != self.default_level
    }
}

impl<'d, T: Pin> Button<'d, T, PollingInput<'d, T>> {
    pub fn new_polling(input: Input<'d, T>, default_level: Level, debounce_duration: Duration) -> Self {
        Self {
            input: PollingInput { input, default_level }, 
            debounce_duration,
            _pin: PhantomData
        }
    }
}
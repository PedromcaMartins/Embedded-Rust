use core::marker::PhantomData;

use embassy_stm32::gpio::{Input, Level, Pin};

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
    pub fn new_polling(input: Input<'d, T>, default_level: Level) -> Self {
        Self {
            input: PollingInput { input, default_level }, 
            _pin: PhantomData
        }
    }
}
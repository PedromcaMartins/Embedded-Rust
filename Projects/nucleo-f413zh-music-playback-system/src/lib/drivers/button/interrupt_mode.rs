use core::marker::PhantomData;

use embassy_stm32::{exti::ExtiInput, gpio::{Level, Pin}};
use embassy_time::Duration;

use super::{Button, ButtonMode, InterruptMode};

pub struct InterruptInput<'d, T: Pin> {
    input: ExtiInput<'d, T>,
    default_level: Level
}

impl<'d, T: Pin> ButtonMode for InterruptInput<'d, T> {
    fn is_pressed_down(&self) -> bool {
        self.input.get_level() != self.default_level
    }
}

impl<'d, T: Pin> InterruptMode for InterruptInput<'d, T> {
    async fn wait_for_press_down(&mut self) {
        match self.default_level {
            Level::High => self.input.wait_for_low().await,
            Level::Low => self.input.wait_for_high().await,
        }
    }

    async fn wait_for_release(&mut self) {
        match self.default_level {
            Level::High => self.input.wait_for_high().await,
            Level::Low => self.input.wait_for_low().await,
        }
    }
}

impl<'d, T: Pin> Button<'d, T, InterruptInput<'d, T>> {
    pub fn new_interrupt(input: ExtiInput<'d, T>, default_level: Level, debounce_duration: Duration) -> Self {
        Self {
            input: InterruptInput { input, default_level }, 
            debounce_duration,
            _pin: PhantomData
        }
    }
}
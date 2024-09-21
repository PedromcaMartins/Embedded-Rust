use core::marker::PhantomData;

use embassy_stm32::gpio::Pin;
use embassy_time::Duration;
use embassy_time::Instant;

mod button_mode;
pub use button_mode::ButtonMode;
pub use button_mode::InterruptMode;
use shared_lib::traits::ButtonDriverInterrupt;
use shared_lib::traits::ButtonDriverPolling;

mod polling_mode;
mod interrupt_mode;

pub struct Button<'d, T, Mode>
where
    T: Pin
{
    input: Mode,
    _pin: PhantomData<&'d T>, // keeps the lifetime around
}

impl<'d, T, M> ButtonDriverPolling for Button<'d, T, M>
where 
    T: Pin,
    M: ButtonMode
{
    fn is_pressed_down(&self) -> bool {
        self.input.is_pressed_down()
    }

    fn is_released(&self) -> bool {
        self.input.is_released()
    }
}

impl<'d, T, M> ButtonDriverInterrupt for Button<'d, T, M> 
where 
    T: Pin,
    M: InterruptMode
{
    async fn wait_for_press_down(&mut self) {
        self.input.wait_for_press_down().await
    }

    async fn wait_for_release(&mut self) {
        self.input.wait_for_release().await
    }
}


impl<'d, T, M> Button<'d, T, M>
where 
    T: Pin,
    M: ButtonMode
{
    pub fn test_polling(&self) {
        let mut passed = true;

        crate::test!("Initiating Button Polling Unit Test");

        if !self.test_polling_pressed_down() {
            crate::test!("test_polling_pressed_down failed");
            passed = false;
        }

        match passed {
            true => crate::test!("Test passed"),
            false => crate::test!("Test failed"),
        }
    }

    fn test_polling_pressed_down(&self) -> bool {
        crate::test!("Please release the button");
        while !self.is_released(){};

        crate::test!("Please press the button");
        while !self.is_pressed_down(){};

        crate::test!("Please release the button");
        while !self.is_released(){};

        let start = Instant::now();
        let timeout = Duration::from_millis(150);
        while Instant::now() - start < timeout {
            if self.is_pressed_down() {
                crate::test!("Error: unexpected button pressed after instructed release");
                return false
            }
        }

        true
    }
}


impl<'d, T, M> Button<'d, T, M>
where 
    T: Pin,
    M: InterruptMode
{
    pub async fn test_interrupt(&mut self) {
        let mut passed = true;

        crate::test!("Initiating Button Interrupt Unit Test");

        if !self.test_interrupt_pressed_down().await {
            crate::test!("test_interrupt_pressed_down failed");
            passed = false;
        }

        match passed {
            true => crate::test!("Test passed"),
            false => crate::test!("Test failed"),
        }
    }

    async fn test_interrupt_pressed_down(&mut self) -> bool {
        crate::test!("Please release the button");
        self.wait_for_release().await;

        crate::test!("Please press the button");
        self.wait_for_press_down().await;

        crate::test!("Please release the button");
        self.wait_for_release().await;

        let start = Instant::now();
        let timeout = Duration::from_millis(150);
        while Instant::now() - start < timeout {
            if self.is_pressed_down() {
                crate::test!("Error: unexpected button pressed after instructed release");
                return false
            }
        }

        true
    }
}
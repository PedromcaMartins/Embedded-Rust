use core::marker::PhantomData;

use defmt::info;
use embassy_stm32::gpio::Pin;
use embassy_time::Duration;
use embassy_time::Instant;

mod button_mode;
pub use button_mode::ButtonMode;
pub use button_mode::InterruptMode;

mod polling_mode;
mod interrupt_mode;
 
pub struct Button<'d, T: Pin, Mode> {
    input: Mode,
    _pin: PhantomData<&'d T>, // keeps the lifetime around
}

impl<'d, T: Pin, M: ButtonMode> Button<'d, T, M> {
    pub fn is_pressed_down(&self) -> bool {
        self.input.is_pressed_down()
    }

    pub fn is_released(&self) -> bool {
        self.input.is_released()
    }
}

impl<'d, T: Pin, M: InterruptMode> Button<'d, T, M> {
    pub async fn wait_for_press_down(&mut self) {
        self.input.wait_for_press_down().await
    }

    pub async fn wait_for_release(&mut self) {
        self.input.wait_for_release().await
    }
}


impl<'d, T: Pin, M: ButtonMode> Button<'d, T, M> {
    pub fn test_polling(&self) {
        let mut passed = true;

        info!("Initiating Button Pooling Unit Test");

        if !self.test_pooling_pressed_down() {
            info!("test_pooling_pressed_down failed");
            passed = false;
        }

        match passed {
            true => info!("Test passed"),
            false => info!("Test failed"),
        }
    }

    fn test_pooling_pressed_down(&self) -> bool {
        info!("Please release the button");
        while !self.is_released(){};

        info!("Please press the button");
        while !self.is_pressed_down(){};

        info!("Please release the button");
        while !self.is_released(){};

        let start = Instant::now();
        let timeout = Duration::from_millis(150);
        while Instant::now() - start < timeout {
            if self.is_pressed_down() {
                info!("Error: unexpected button pressed after instructed release");
                return false
            }
        }

        true
    }
}


impl<'d, T: Pin, M: InterruptMode> Button<'d, T, M> {
    pub async fn test_interrupt(&mut self) {
        let mut passed = true;

        info!("Initiating Button Interrupt Unit Test");

        if !self.test_interrupt_pressed_down().await {
            info!("test_interrupt_pressed_down failed");
            passed = false;
        }

        match passed {
            true => info!("Test passed"),
            false => info!("Test failed"),
        }
    }

    async fn test_interrupt_pressed_down(&mut self) -> bool {
        info!("Please release the button");
        self.wait_for_release().await;

        info!("Please press the button");
        self.wait_for_press_down().await;

        info!("Please release the button");
        self.wait_for_release().await;

        let start = Instant::now();
        let timeout = Duration::from_millis(150);
        while Instant::now() - start < timeout {
            if self.is_pressed_down() {
                info!("Error: unexpected button pressed after instructed release");
                return false
            }
        }

        true
    }
}
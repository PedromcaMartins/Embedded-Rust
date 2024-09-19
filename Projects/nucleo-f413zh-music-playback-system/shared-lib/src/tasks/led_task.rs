use crate::traits::{LedDriver, TimerWrapper};

pub struct LedTask<L: LedDriver> {
    led_driver: L,
}

impl<L: LedDriver> LedTask<L> {
    pub fn new(led_driver: L) -> Self {
        Self { led_driver }
    }

    pub async fn run<T: TimerWrapper>(mut self, _: T) -> ! {
        loop {
            self.led_driver.toggle();
            T::after_millis(500).await;
        }
    }
}

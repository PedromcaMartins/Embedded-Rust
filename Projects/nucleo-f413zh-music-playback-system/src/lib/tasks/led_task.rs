use crate::drivers::Led;

pub struct LedTask<'d> {
    led: Led<'d>,
}

impl<L: LedDriver> LedTask<L> {
    pub fn new(led: L) -> Self {
        Self { led }
    }

    pub async fn run<T: TimerDriver>(mut self, _: T) -> ! {
        loop {
            self.led.toggle();
            T::after_millis(500).await;
        }
    }
}

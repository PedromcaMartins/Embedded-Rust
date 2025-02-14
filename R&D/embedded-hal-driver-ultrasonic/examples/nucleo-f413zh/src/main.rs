#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::gpio::Flex;
use embassy_time::{Delay, Instant, Timer};
use {defmt_rtt as _, panic_probe as _};

use driver_ultrasonic::{Ultrasonic, TimeProvider};

struct TimeProviderImpl;

impl TimeProvider for TimeProviderImpl {
    fn now_us(&self) -> u64 {
        Instant::now().as_micros()
    }
}

impl TimeProviderImpl {
    pub fn new() -> Self {
        Self {}
    }
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());

    let mut time_provider = TimeProviderImpl::new();
    let mut ultrasonic = Ultrasonic::new(
        Flex::new(p.PC13),
    );

    loop {
        let measurement = ultrasonic.measure_in_millimeters(&mut Delay, &mut time_provider).await;
        info!("Measurement: {:?}", measurement);
        Timer::after_millis(1000).await;
    }
}

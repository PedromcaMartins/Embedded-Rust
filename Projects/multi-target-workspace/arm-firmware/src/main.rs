#![no_std]
#![no_main]

mod fmt;

#[cfg(not(feature = "defmt"))]
use panic_halt as _;
use shared_lib::Counter;
#[cfg(feature = "defmt")]
use {defmt_rtt as _, panic_probe as _};

use embassy_executor::Spawner;
use fmt::info;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let _p = embassy_stm32::init(Default::default());

    info!("hello world!");

    let mut counter = Counter::new();

    info!("Running unit tests on the stm32...");

    for i in 0..5 {
        counter.increment();
        assert_eq!(counter.get_value(), i + 1);
        info!("Test passed for value: {}", i + 1);
    }

    info!("All tests passed!");
}

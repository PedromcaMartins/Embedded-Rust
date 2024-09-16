#![no_std]
#![no_main]

mod fmt;

#[cfg(not(feature = "defmt"))]
use panic_halt as _;
#[cfg(feature = "defmt")]
use {defmt_rtt as _, panic_probe as _};

use embassy_executor::Spawner;
use embassy_stm32::adc::Adc;
use embassy_time::{Delay, Timer};
use fmt::info;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    info!("Hello, World!");

    let mut p = embassy_stm32::init(Default::default());

    let mut sound_sensor_adc = Adc::new(p.ADC1, &mut Delay);

    loop {
        let mut sum: u16 = 0;
        for _ in 0..32 {
            sum = sum.checked_add(sound_sensor_adc.read(&mut p.PA3)).unwrap_or_default();
        }
        sum >>= 5;

        info!("{}", sum);
        Timer::after_millis(10).await;
    }
}

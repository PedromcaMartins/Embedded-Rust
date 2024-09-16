#![no_std]
#![no_main]

mod fmt;

#[cfg(not(feature = "defmt"))]
use panic_halt as _;
#[cfg(feature = "defmt")]
use {defmt_rtt as _, panic_probe as _};

use embassy_executor::Spawner;
use embassy_stm32::{gpio::{Flex, Input, Level, Output, Pull, Speed}, peripherals, time::Hertz, timer::pwm_input::PwmInput};
use embassy_time::{Duration, Instant, Timer};
use fmt::info;

mod ultrasonic;


#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    info!("Hello, World!");

    let p = embassy_stm32::init(Default::default());

    let mut pin = Flex::new(p.PA3);

    // spawner.spawn(blinky(p.PC0)).unwrap();

    loop {
        if let Some(range_in_centimeters) = ultrasonic::measure_in_centimeters(&mut pin, None).await {
            info!("Range: {:?} cm", range_in_centimeters);
        }
        Timer::after_millis(250).await;
    }
}

/// Connect PB2 and PA6 with a 1k Ohm resistor

#[embassy_executor::task]
async fn blinky(led_pin: peripherals::PC0) {
    let mut led = Output::new(led_pin, Level::Low, Speed::Low);

    loop {
        led.set_high();
        Timer::after_millis(100).await;

        led.set_low();
        Timer::after_millis(900).await;
    }
}

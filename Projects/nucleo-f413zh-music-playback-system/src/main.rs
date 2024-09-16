#![no_std]
#![no_main]

mod fmt;

#[cfg(not(feature = "defmt"))]
use panic_halt as _;
#[cfg(feature = "defmt")]
use {defmt_rtt as _, panic_probe as _};

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use fmt::info;

mod io_mapping;
use io_mapping::IOMapping;

mod drivers;
use drivers::{Button, Led};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    info!("Hello, World!");

    let p = embassy_stm32::init(Default::default());
    let io_mapping = IOMapping::init(p);

    let button = Button::init(
        io_mapping.playback_pause_play_button, 
        io_mapping.playback_pause_play_button_default_level
    );
    let mut led = Led::init(
        io_mapping.playback_status_led, 
        io_mapping.playback_status_led_default_level
    );


    button.test();
    led.test();

    loop {

        Timer::after(Duration::from_millis(500)).await;
    }
}
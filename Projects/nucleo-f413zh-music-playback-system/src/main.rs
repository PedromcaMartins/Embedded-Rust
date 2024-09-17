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

    // let button = Button::new_polling(
    //     io_mapping.playback_pause_play_button, 
    //     io_mapping.playback_pause_play_button_default_level
    // );
    let mut button = Button::new_interrupt(
        io_mapping.playback_pause_play_button, 
        io_mapping.playback_pause_play_button_default_level
    );


    button.test_polling();
    button.test_interrupt().await;

    loop {

        Timer::after(Duration::from_millis(500)).await;
    }
}
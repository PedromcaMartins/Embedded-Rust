#![no_std]
#![no_main]

mod fmt;
mod percentage;

#[cfg(not(feature = "defmt"))]
use panic_halt as _;
#[cfg(feature = "defmt")]
use {defmt_rtt as _, panic_probe as _};

use embassy_executor::Spawner;
use fmt::info;

mod io_mapping;
use io_mapping::IOMapping;

mod drivers;
#[allow(unused)]
use drivers::{AdcManager, Button, Led, Potentiometer};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    info!("Hello, World!");

    let _p = embassy_stm32::init(Default::default());

    panic!();
}
#![no_std]
#![no_main]

#[cfg(not(feature = "defmt"))]
use panic_halt as _;
#[cfg(feature = "defmt")]
use {defmt_rtt as _, panic_probe as _};

mod spawner;
mod timer;

use embassy_executor::Spawner;

use arm::info;
use arm::io_mapping::IOMapping;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    let _io_mapping = IOMapping::init(p);
    info!("hello world!");
}

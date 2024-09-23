#![no_std]
#![no_main]


use {defmt_rtt as _, panic_probe as _};

use arm::{io_mapping::IOMapping, tasks::cli_task_spawn};

use embassy_executor::Spawner;

use defmt::info;

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    let io_mapping = IOMapping::init(p);
    info!("hello world!");

    spawner.spawn(cli_task_spawn(io_mapping.pc_uart)).unwrap();
}

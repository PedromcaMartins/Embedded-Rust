#![no_std]
#![no_main]

use arm::{drivers::UartWrapper, io_mapping::IOMapping};
#[cfg(not(feature = "defmt"))]
use panic_halt as _;
use shared_lib::tasks::CliTask;
use timer::TimerWrapper;
#[cfg(feature = "defmt")]
use {defmt_rtt as _, panic_probe as _};

mod spawner;
mod timer;

use embassy_executor::Spawner;

use defmt::info;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    let io_mapping = IOMapping::init(p);
    info!("hello world!");

    let mut dma_buf = [0u8; 32];
    let pc_uart = UartWrapper::new(
        io_mapping.pc_uart, 
        &mut dma_buf, 
        true
    );

    let cli_task = CliTask::new(pc_uart);

    cli_task.run(TimerWrapper).await;
}

#![no_std]
#![no_main]

use core::str;

use arm::{drivers::PCUart, io_mapping::IOMapping};
#[cfg(not(feature = "defmt"))]
use panic_halt as _;
use shared_lib::traits::UartDriver;
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

    let pc_uart = io_mapping.pc_uart;

    let mut dma_buf = [0u8; 32];
    let mut pc_uart = PCUart::new(pc_uart, &mut dma_buf, true);

    let mut external_rx_buf =  [0u8; 32];

    loop {
        let n = pc_uart.read_line(&mut external_rx_buf).await.unwrap();

        if let Ok(command) = str::from_utf8(&external_rx_buf[..n]) {
            info!("{}", command);
            pc_uart.write_line(command.as_bytes()).await.unwrap();
        }
    }
}

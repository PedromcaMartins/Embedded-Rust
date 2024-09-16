#![no_std]
#![no_main]

use core::str::FromStr;

use {defmt_rtt as _, panic_probe as _};

use cortex_m::prelude::_embedded_hal_blocking_serial_Write;
use embassy_executor::Spawner;
use embassy_stm32::{bind_interrupts, gpio::{Level, Output, Speed}, peripherals, usart::{self, BufferedUart}};
use embassy_time::{Duration, Instant, Timer};
use embedded_io_async::BufRead;
use heapless::String;

bind_interrupts!(struct Irqs {
    USART3 => usart::BufferedInterruptHandler<peripherals::USART3>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    defmt::info!("Hello, World!");

    let mut tx_buf = [0u8, 32];
    let mut rx_buf = [0u8, 1];

    let p = embassy_stm32::init(Default::default());
    let mut debug_uart = BufferedUart::new(p.USART3, Irqs, p.PD9, p.PD8, &mut tx_buf, &mut rx_buf, Default::default()).unwrap();

    loop {
        let buf = debug_uart.fill_buf().await.unwrap();

        defmt::info!("Received: {}", buf);

        // Read bytes have to be explicitly consumed, otherwise fill_buf() will return them again
        let n = buf.len();
        debug_uart.consume(n);

        debug_uart.bwrite_all("Received\r\n".as_bytes()).unwrap();
    }
}

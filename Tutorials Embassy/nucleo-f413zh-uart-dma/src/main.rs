#![no_std]
#![no_main]

use core::str::FromStr;

use {defmt_rtt as _, panic_probe as _};

use embassy_executor::Spawner;
use embassy_stm32::{bind_interrupts, dma::NoDma, gpio::{Level, Output, Speed}, peripherals, usart::{self, Uart}};
use embassy_time::{Duration, Instant, Timer};
use heapless::String;

bind_interrupts!(struct Irqs {
    USART3 => usart::InterruptHandler<peripherals::USART3>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    defmt::info!("Hello, World!");

    let p = embassy_stm32::init(Default::default());
    let mut debug_uart = Uart::new(p.USART3, p.PD9, p.PD8, Irqs, p.DMA1_CH3, p.DMA1_CH1, Default::default()).unwrap();

    let string_slice = "hello world!\r\n";
    let mut buf = [0u8; 1];

    loop {
        debug_uart.write(string_slice.as_bytes()).await.unwrap();
        debug_uart.read(&mut buf).await.unwrap();
        Timer::after_millis(1000).await;
    }
}

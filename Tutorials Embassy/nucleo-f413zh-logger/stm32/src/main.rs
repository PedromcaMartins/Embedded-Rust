#![no_std]
#![no_main]

mod logger;

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::{bind_interrupts, peripherals, usart::{self, Uart}};
use embassy_time::Timer;
use logger::init_logger;
// use defmt_rtt as _;
use panic_probe as _;

bind_interrupts!(struct Irqs {
    USART3 => usart::InterruptHandler<peripherals::USART3>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    // info!("Hello World!");

    let (uart_tx, _uart_rx) = Uart::new(p.USART3, p.PD9, p.PD8, Irqs, p.DMA1_CH3, p.DMA1_CH1, Default::default()).unwrap().split();

    init_logger(uart_tx);

    loop {
        // uart.write(b"hello world!\r\n").await.unwrap();
        // uart.flush().await.unwrap();
        info!("hello world!\r\n");
        Timer::after_millis(300).await;
    }
}

#![no_std]
#![no_main]

use core::{str::FromStr, sync::atomic::{AtomicU8, Ordering}};

use {defmt_rtt as _, panic_probe as _};

use embassy_executor::Spawner;
use embassy_stm32::{bind_interrupts, dma::NoDma, gpio::{Level, Output, Speed}, peripherals::{self, DMA1_CH1, DMA1_CH3, USART3}, usart::{self, Uart, UartRx, UartTx}};
use embassy_time::{Duration, Instant, Timer};
use heapless::String;

bind_interrupts!(struct Irqs {
    USART3 => usart::InterruptHandler<peripherals::USART3>;
});

static DEBUG_CHANNEL: AtomicU8 = AtomicU8::new(0);

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    defmt::info!("Hello, World!");

    let p = embassy_stm32::init(Default::default());

    let mut debug_uart = Uart::new(p.USART3, p.PD9, p.PD8, Irqs, p.DMA1_CH3, p.DMA1_CH1, Default::default()).unwrap();
    let (tx, rx) = debug_uart.split();

    let mut led = Output::new(p.PB7, Level::Low, Speed::Low);


    spawner.spawn(write_debug_uart(tx)).unwrap();
    spawner.spawn(read_debug_uart(rx)).unwrap();

    loop {
        led.toggle();
        Timer::after_millis(500).await;
    }
}

#[embassy_executor::task]
async fn write_debug_uart(mut tx: UartTx<'static, USART3, DMA1_CH3>) -> ! {
    let string_slice = "hello world!\r\n";

    loop {
        tx.write(string_slice.as_bytes()).await.unwrap();

        tx.write(&[DEBUG_CHANNEL.load(Ordering::Relaxed)]).await.unwrap();
        tx.write("\r\n".as_bytes()).await.unwrap();
        Timer::after_millis(1000).await;
    }
}

#[embassy_executor::task]
async fn read_debug_uart(mut rx: UartRx<'static, USART3, DMA1_CH1>) -> ! {
    let mut buf = [0u8; 1];

    loop {
        rx.read(&mut buf).await.unwrap();
        DEBUG_CHANNEL.store(buf[0], Ordering::Relaxed);
    }
}

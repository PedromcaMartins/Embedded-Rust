use defmt::{global_logger, Logger};
use embassy_stm32::{mode::Async, usart::UartTx};
// use embassy_time::Instant;

#[global_logger]
struct MyLogger;

// defmt::timestamp!("{=u64:us}", Instant::now().as_micros());

static mut LOG_UART: Option<UartTx<'static, Async>> = None;

pub fn init_logger(uart: UartTx<'static, Async>) {
    unsafe {
        LOG_UART = Some(uart);
    }
}

unsafe impl Logger for MyLogger {
    fn acquire() {
        
    }

    unsafe fn flush() {
        unsafe {
            if let Some(ref mut uart) = LOG_UART {
                let _ = uart.blocking_flush();
            }
        }
    }

    unsafe fn release() {
        
    }

    unsafe fn write(bytes: &[u8]) {
        unsafe {
            if let Some(ref mut uart) = LOG_UART {
                let _ = uart.blocking_write(bytes);
            }
        }
    }
}
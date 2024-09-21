#![no_std]
#![no_main]

#[cfg(not(feature = "defmt"))]
use panic_halt as _;
#[cfg(feature = "defmt")]
use {defmt_rtt as _, panic_probe as _};

// See https://crates.io/crates/defmt-test/0.3.0 for more documentation (e.g. about the 'state' feature)
#[defmt_test::tests]
mod test_driver_uart_wrapper {
    use arm::{drivers::UartWrapper, io_mapping::IOMappingTest, noop_waker::poll_future};

    #[test]
    fn test_driver_uart_wrapper() {
        let p = embassy_stm32::init(Default::default());
        let io_mapping = IOMappingTest::init(p);

        let mut rx_dma_buf = [0u8; 32];
        let mut uart = UartWrapper::new(io_mapping.pc_uart, &mut rx_dma_buf, true);

        let mut future = uart.test();
        poll_future(&mut future);
    }
}

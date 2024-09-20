#![no_std]
#![no_main]

#[cfg(not(feature = "defmt"))]
use panic_halt as _;
#[cfg(feature = "defmt")]
use {defmt_rtt as _, panic_probe as _};

// See https://crates.io/crates/defmt-test/0.3.0 for more documentation (e.g. about the 'state' feature)
#[defmt_test::tests]
mod test_driver_button {
    use arm::{drivers::Button, io_mapping::IOMappingTest, noop_waker::poll_future};

    use crate::noop_waker;

    #[test]
    fn test_driver_button() {
        let p = embassy_stm32::init(Default::default());
        let io_mapping = IOMappingTest::init(p);

        let mut button = Button::new_interrupt(io_mapping.button, io_mapping.button_default_level);

        button.test_polling();
        let mut future = button.test_interrupt();
        poll_future(&mut future);
    }
}
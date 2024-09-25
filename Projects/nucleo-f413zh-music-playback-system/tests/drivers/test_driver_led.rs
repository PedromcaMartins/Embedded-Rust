#![no_std]
#![no_main]

use {defmt_rtt as _, panic_probe as _};

// See https://crates.io/crates/defmt-test/0.3.0 for more documentation (e.g. about the 'state' feature)
#[defmt_test::tests]
mod test_driver_led {
    use arm::{drivers::Led, io_mapping::IOMappingTest};

    #[test]
    fn test_driver_led() {
        let p = embassy_stm32::init(Default::default());
        let io_mapping = IOMappingTest::init(p);

        let mut led = Led::new(io_mapping.led, io_mapping.led_default_level);

        led.test();
    }
}

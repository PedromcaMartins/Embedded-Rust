#![no_std]
#![no_main]

#[cfg(not(feature = "defmt"))]
use panic_halt as _;
#[cfg(feature = "defmt")]
use {defmt_rtt as _, panic_probe as _};

// See https://crates.io/crates/defmt-test/0.3.0 for more documentation (e.g. about the 'state' feature)
#[defmt_test::tests]
mod test_driver_adc_manager {
    use arm::{drivers::{AdcManager, AdcPinInput}, io_mapping::IOMappingTest};

    #[test]
    fn test_driver_adc_manager() {
        let p = embassy_stm32::init(Default::default());
        let io_mapping = IOMappingTest::init(p);

        let mut adc_manager = AdcManager::new(io_mapping.potentiometer_adc);
        let mut pin = io_mapping.potentiometer_pin;

        let mut adc_pin_input = AdcPinInput::new(&mut pin, &mut adc_manager);

        adc_pin_input.test();
    }
}

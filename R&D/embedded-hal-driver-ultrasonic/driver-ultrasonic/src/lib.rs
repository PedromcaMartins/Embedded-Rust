#![cfg_attr(not(test), no_std)]

use core::time::Duration;

use embedded_hal::digital::{InputPin, OutputPin};
use embedded_hal_async::delay::DelayNs as AsyncDelayNs;

/* -------------------------------------------------------------------------- */
/*                                   Logging                                  */
/* -------------------------------------------------------------------------- */

#[cfg(all(feature = "defmt", feature = "log"))]
compile_error!("Cannot enable both log and defmt");

#[cfg(feature = "log")]
#[allow(unused_imports)]
use log::{debug, trace, warn};

#[cfg(feature = "defmt")]
#[allow(unused_imports)]
use defmt::{debug, trace, warn};

#[cfg(all(not(feature = "defmt"), not(feature = "log")))]
#[macro_export]
/// Like log::debug! but does nothing at all
macro_rules! debug {
    ($($arg:tt)+) => {};
}

#[cfg(all(not(feature = "defmt"), not(feature = "log")))]
#[macro_export]
/// Like log::trace! but does nothing at all
macro_rules! trace {
    ($($arg:tt)+) => {};
}

#[cfg(all(not(feature = "defmt"), not(feature = "log")))]
#[macro_export]
/// Like log::warn! but does nothing at all
macro_rules! warn {
    ($($arg:tt)+) => {};
}

/* -------------------------------------------------------------------------- */
/*                                   Library                                  */
/* -------------------------------------------------------------------------- */

/// Time Provider for the Ultrasonic Sensor
pub trait TimeProvider {
    /// Return the current timestamp in microseconds.
    /// Implementations MUST ensure that:
    /// 
    /// This is guaranteed to be monotonic, i.e. a call to now() will always return a greater or equal value than earlier calls. Time can’t “roll backwards”.
    /// It “never” overflows. It must not overflow in a sufficiently long time frame, say in 10_000 years (Human civilization is likely to already have self-destructed 10_000 years from now.). This means if your hardware only has 16bit/32bit timers you MUST extend them to 64-bit, for example by counting overflows in software, or chaining multiple timers together.
    fn now_us(&self) -> u64;
}

/// Ultrasonic Sensor
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug)]
pub struct Ultrasonic<PIN> 
where 
PIN: InputPin + OutputPin,
{
    pin: PIN,
}

impl<PIN> Ultrasonic<PIN>
where 
    PIN: InputPin + OutputPin,
{
    pub fn new(pin: PIN) -> Self {
        Self {
            pin,
        }
    }

    pub fn destroy(self) -> PIN {
        self.pin
    }

    fn pulse_in(&mut self, time_provider: &mut impl TimeProvider) -> Result<Duration, PIN::Error> {
        // Wait for the pulse to start
        while self.pin.is_low()? {}
        let pulse_begin = time_provider.now_us();

        // Wait for the pulse to stop
        while self.pin.is_high()? {}
        let pulse_end = time_provider.now_us();

        trace!("Pulse begin: {:?}", pulse_begin);
        trace!("Pulse end: {:?}", pulse_end);

        // Calculate the pulse duration
        Ok(Duration::from_micros(pulse_end - pulse_begin))
    }

    async fn duration(&mut self, delay: &mut impl AsyncDelayNs, time_provider: &mut impl TimeProvider) -> Result<Duration, PIN::Error> {
        self.pin.set_low()?;
        delay.delay_us(2).await;
        self.pin.set_high()?;
        delay.delay_us(5).await;
        self.pin.set_low()?;

        self.pulse_in(time_provider)
    }

    /// The measured distance from the range 0 to 4000 Millimeters
    /// 
    /// Warning: The data rate is porpotional to the distance measured
    pub async fn measure_in_millimeters(&mut self, delay: &mut impl AsyncDelayNs, time_provider: &mut impl TimeProvider) -> Result<u32, PIN::Error> {
        let duration = self.duration(delay, time_provider).await?;
        trace!("Duration: {:?}", duration);
        if duration.as_secs() > 0 {
            panic!("duration greater than one second. This is not allowed with the current implementation");
        }
        let duration = duration.subsec_micros();

        Ok((duration * 5) / 29)
    }
}



#[cfg(test)]
mod test {
    use super::*;
    use embedded_hal_mock::eh1::digital as gpio;
    use embedded_hal_mock::eh1::delay;
    use mockall::mock;

    const PULSE_DURATION_MICROS: u64 = 23_200;
    const PULSE_DISTANCE_MILLIMETERS: u32 = 4_000;

    mock! {
        // Structure to mock
        TimeProvider {}
        // Trait to implement on C
        impl super::TimeProvider for TimeProvider {
            fn now_us(&self) -> u64;
        }
    }

    #[tokio::test]
    async fn test_mock() {
        let pin = gpio::Mock::new(
            &[
                gpio::Transaction::set(gpio::State::Low),
                gpio::Transaction::set(gpio::State::High),
                gpio::Transaction::set(gpio::State::Low),
            ]
        );

        let mut delay = delay::NoopDelay::new();

        let mut time_provider = MockTimeProvider::default();
        time_provider.expect_now_us()
            .return_const(0_u64)
            .times(1);
        time_provider.expect_now_us()
            .return_const(PULSE_DURATION_MICROS)
            .times(1);

        // Create a mock Ultrasonic Sensor
        let mut ultrasonic = Ultrasonic::new(
            pin, 
        );

        let value = ultrasonic.measure_in_millimeters(&mut delay, &mut time_provider).await.unwrap();
        assert_eq!(value, PULSE_DISTANCE_MILLIMETERS);

        let mut pin = ultrasonic.destroy();
        pin.done();
    }
}

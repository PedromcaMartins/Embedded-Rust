#![no_std]

use core::time::Duration;

/// Time Provider for the Ultrasonic Sensor
pub trait TimeProvider {
    /// Return the current timestamp in microseconds.
    /// Implementations MUST ensure that:
    /// 
    /// This is guaranteed to be monotonic, i.e. a call to now() will always return a greater or equal value than earlier calls. Time can’t “roll backwards”.
    /// It “never” overflows. It must not overflow in a sufficiently long time frame, say in 10_000 years (Human civilization is likely to already have self-destructed 10_000 years from now.). This means if your hardware only has 16bit/32bit timers you MUST extend them to 64-bit, for example by counting overflows in software, or chaining multiple timers together.
    fn now_us(&self) -> u64;
}

pub struct Ultrasonic<PIN, DELAYER, TIMEPROVIDER> 
where 
    PIN: embedded_hal::digital::StatefulOutputPin + embedded_hal_async::digital::Wait,
    DELAYER: embedded_hal_async::delay::DelayNs,
    TIMEPROVIDER: TimeProvider,
{
    pin: PIN,
    delayer: DELAYER,
    time_provider: TIMEPROVIDER,
}

impl<PIN, DELAYER, TIMEPROVIDER> Ultrasonic<PIN, DELAYER, TIMEPROVIDER>
where 
    PIN: embedded_hal::digital::StatefulOutputPin + embedded_hal_async::digital::Wait,
    DELAYER: embedded_hal_async::delay::DelayNs,
    TIMEPROVIDER: TimeProvider,
{
    async fn pulse_in(&mut self) -> Result<Duration, PIN::Error> {
        // Wait for the pulse to start
        self.pin.wait_for_rising_edge().await?;
        let pulse_begin = self.time_provider.now_us();

        // Wait for the pulse to stop
        self.pin.wait_for_falling_edge().await?;
        let pulse_end = self.time_provider.now_us();

        // Calculate the pulse duration
        Ok(Duration::from_micros(pulse_end - pulse_begin))
    }

    async fn duration(&mut self) -> Result<Duration, PIN::Error> {
        self.pin.set_low()?;
        self.delayer.delay_us(2).await;
        self.pin.set_high()?;
        self.delayer.delay_us(5).await;
        self.pin.set_low()?;

        self.pulse_in().await
    }

    /// The measured distance from the range 0 to 4000 Millimeters
    /// 
    /// Warning: The data rate is porpotional to the distance measured
    pub async fn measure_in_millimeters(&mut self) -> Result<u32, PIN::Error> {
        let duration = self.duration().await?;
        if duration.as_secs() > 0 {
            panic!("duration greater than one second. This is not allowed with the current implementation");
        }
        let duration = duration.subsec_micros();

        // Approximation of duration * 10 / 58
        // 
        // Considering range 0 to 4_000 Millimeters,
        // Duration range is 0 to 23_200 Microseconds
        // 
        // Error function: duration * X / 2^n - 4_000
        // This function was minimized for X / 2^n = 10 / 58
        // Optimal solution: X = 11, n = 6
        Ok((duration * 11) << 6)
    }
}

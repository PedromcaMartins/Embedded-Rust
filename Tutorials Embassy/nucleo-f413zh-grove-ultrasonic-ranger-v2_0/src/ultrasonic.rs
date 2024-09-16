use embassy_stm32::gpio::{Flex, Level, Pull, Speed};
use embassy_time::{Duration, Instant, Timer};

async fn pulse_in(pin: &Flex<'static>, timeout: Duration) -> Option<Duration> {
    let begin = Instant::now();

    // Wait for any previous pulse to end
    while pin.is_high() {
        if begin.elapsed() >= timeout {
            return None;
        }
    }

    // Wait for the pulse to start
    while pin.is_low() {
        if begin.elapsed() >= timeout {
            return None;
        }
    }
    let pulse_begin = Instant::now();

    // Wait for the pulse to stop
    while pin.is_high() {
        if pulse_begin.elapsed() >= timeout {
            return None;
        }
    }
    let pulse_end = Instant::now();

    Some(pulse_end.duration_since(pulse_begin))
}

async fn duration(trigger: &mut Flex<'static>, timeout: Duration) -> Option<Duration> {
    trigger.set_as_output(Speed::Low);
    trigger.set_level(Level::Low);
    Timer::after_micros(2).await;
    trigger.set_level(Level::High);
    Timer::after_micros(5).await;
    trigger.set_level(Level::Low);

    trigger.set_as_input(Pull::None);
    pulse_in(trigger, timeout).await
}

/*The measured distance from the range 0 to 400 Centimeters*/
pub async fn measure_in_centimeters(trigger: &mut Flex<'static>, timeout: Option<Duration>) -> Option<u64> {
    let timeout = timeout.unwrap_or(Duration::from_millis(30));
    let duration = duration(trigger, timeout).await?.as_micros();
    Some(duration / 29 / 2)
}

/*The measured distance from the range 0 to 4000 Millimeters*/
pub async fn measure_in_millimeters(trigger: &mut Flex<'static>, timeout: Option<Duration>) -> Option<u64> {
    let timeout = timeout.unwrap_or(Duration::from_millis(30));
    let duration = duration(trigger, timeout).await?.as_micros();
    Some(duration * (10 / 2) / 29)
}

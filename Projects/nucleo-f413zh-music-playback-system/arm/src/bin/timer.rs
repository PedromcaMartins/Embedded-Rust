use core::future::Future;

use embassy_time::Timer;
use shared_lib::traits::TimerWrapper;

#[derive(Default)]
pub struct EmbassyTimer;

impl TimerWrapper for EmbassyTimer {
    fn after(duration: core::time::Duration) -> impl Future<Output = ()> {
        Timer::after(embassy_time::Duration::try_from(duration).unwrap())
    }

    fn after_micros(micros: u64) -> impl Future<Output = ()> {
        Timer::after_micros(micros)
    }

    fn after_millis(millis: u64) -> impl Future<Output = ()> {
        Timer::after_millis(millis)
    }

    fn after_nanos(nanos: u64) -> impl Future<Output = ()> {
        Timer::after_nanos(nanos)
    }

    fn after_secs(secs: u64) -> impl Future<Output = ()> {
        Timer::after_secs(secs)
    }

    fn after_ticks(ticks: u64) -> impl Future<Output = ()> {
        Timer::after_ticks(ticks)
    }
}
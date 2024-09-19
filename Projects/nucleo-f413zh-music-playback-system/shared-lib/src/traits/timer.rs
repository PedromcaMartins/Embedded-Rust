use core::{future::Future, time::Duration};

pub trait TimerWrapper
{
    fn after(duration: Duration) -> impl Future<Output = ()>;
    fn after_ticks(ticks: u64) -> impl Future<Output = ()>;
    fn after_nanos(nanos: u64) -> impl Future<Output = ()>;
    fn after_micros(micros: u64) -> impl Future<Output = ()>;
    fn after_secs(secs: u64) -> impl Future<Output = ()>;
    fn after_millis(millis: u64) -> impl Future<Output = ()>;
}
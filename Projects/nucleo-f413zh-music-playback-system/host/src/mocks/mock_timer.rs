use std::time::Duration;

use shared_lib::traits::TimerWrapper;

pub struct MockTimer;

impl TimerWrapper for MockTimer {
    async fn after(_duration: Duration) {

    }

    async fn after_ticks(_ticks: u64) {

    }

    async fn after_nanos(_nanos: u64) {

    }

    async fn after_micros(_micros: u64) {

    }

    async fn after_secs(_secs: u64) {

    }

    async fn after_millis(_millis: u64) {

    }

}
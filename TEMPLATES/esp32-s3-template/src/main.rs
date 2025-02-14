#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_time::Timer;
use esp_backtrace as _;
use esp_hal::{
    self,
    clock::CpuClock,
    timer::timg::TimerGroup,
};
use esp_println::println;

#[esp_hal_embassy::main]
async fn main(_spawner: Spawner) {
    esp_println::logger::init_logger_from_env();

    let mut config = esp_hal::Config::default();
    config.cpu_clock = CpuClock::max();
    let peripherals = esp_hal::init(config);

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    esp_hal_embassy::init(timg0.timer0);

    // let rmt = Rmt::new(peripherals.RMT, 80.MHz(), None).unwrap();

    // let rmt_buffer = smartLedBuffer!(1);
    // let mut led = SmartLedsAdapter::new(rmt.channel0, peripherals.GPIO48, rmt_buffer);
    loop {
        println!("Hello, World!");
        // led.write_blocking(
        //     &[
        //         0x00
        //     ]
        // ).unwrap();
        Timer::after_millis(1_000).await;
    }
}

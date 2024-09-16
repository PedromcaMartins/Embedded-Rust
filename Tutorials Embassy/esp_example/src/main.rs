#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_backtrace as _;
use esp_hal::{
    clock::ClockControl,
    gpio::{Io, Level, Output},
    peripherals::Peripherals,
    prelude::*,
    system::SystemControl,
    timer::{timg::TimerGroup, ErasedTimer, OneShotTimer},
};
use esp_println::println;
use static_cell::StaticCell;

#[main]
async fn main(spawner: Spawner) {
    esp_println::logger::init_logger_from_env();

    esp_println::println!("Init!");
    let peripherals = Peripherals::take();
    let system = SystemControl::new(peripherals.SYSTEM);
    let clocks = ClockControl::max(system.clock_control).freeze();

    let timg0 = TimerGroup::new(peripherals.TIMG0, &clocks, None);
    let timer = {
        static TIMER: StaticCell<[OneShotTimer<ErasedTimer>; 1]> = StaticCell::new();

        TIMER.init([OneShotTimer::new(timg0.timer0.into())])
    };
    esp_hal_embassy::init(&clocks, timer);

    let io = Io::new(peripherals.GPIO, peripherals.IO_MUX);
    let mut led = Output::new(io.pins.gpio17, Level::Low);

    spawner.spawn(run()).unwrap();

    loop {
        println!("Hello, World!");
        led.toggle();
        Timer::after_millis(1_000).await;
    }
}

#[embassy_executor::task]
async fn run() {
    loop {
        esp_println::println!("Hello world from embassy using esp-hal-async!");
        Timer::after_millis(7_000).await;
    }
}
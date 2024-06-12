#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Input, Level, Output, Speed, Pull};
use embassy_time::{Duration, Timer};
use {defmt_rtt as _, panic_probe as _}; // global logger

const BLINK_SPEEDS_MILLIS: [u64; 4] = [100, 200, 500, 1000];

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    info!("Hello, World!");

    let mut led = Output::new(p.PB7, Level::High, Speed::Low);
    let button = Input::new(p.PC13, Pull::None);

    let mut blink_speed_index = 0;
    let mut blink_speed = get_new_blink_speed(&mut blink_speed_index);

    loop {
        if button.is_high() {
            blink_speed = get_new_blink_speed(&mut blink_speed_index);
        }
        
        led.toggle();
        Timer::after(Duration::from_millis(blink_speed)).await;
    }
}

fn get_new_blink_speed(blink_speed_index: &mut usize) -> u64 {
    if *blink_speed_index == BLINK_SPEEDS_MILLIS.len() - 1 {
        *blink_speed_index = 0;
    }

    *blink_speed_index += 1;

    BLINK_SPEEDS_MILLIS[*blink_speed_index]
}

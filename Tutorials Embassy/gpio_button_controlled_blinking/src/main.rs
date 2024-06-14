#![no_std]
#![no_main]

use core::sync::atomic::{AtomicU32, Ordering};

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::{exti::ExtiInput, gpio::{AnyPin, Input, Level, Output, Pin, Pull, Speed}};
use embassy_time::{Duration, Timer};
use {defmt_rtt as _, panic_probe as _}; // global logger

const BLINK_SPEEDS_MS: [u32; 6] = [100, 150, 200, 250, 200, 150];

static BLINK_MS: AtomicU32 = AtomicU32::new(0);

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    info!("Hello, World!");

    let button = Input::new(p.PC13, Pull::None);
    let mut button = ExtiInput::new(button, p.EXTI13);

    let mut blink_speed_index = 0;
    let mut blink_speed = get_new_blink_speed(&mut blink_speed_index);
    BLINK_MS.store(blink_speed, Ordering::Relaxed);

    spawner.spawn(led_task(p.PB7.degrade())).unwrap();

    loop {
        button.wait_for_rising_edge().await;

        blink_speed = get_new_blink_speed(&mut blink_speed_index);
        BLINK_MS.store(blink_speed, Ordering::Relaxed);
    }
}

#[embassy_executor::task]
async fn led_task(led_pin: AnyPin) {
    let mut led = Output::new(led_pin, Level::High, Speed::Low);

    loop {
        let blink_speed = BLINK_MS.load(Ordering::Relaxed);
        led.toggle();
        Timer::after(Duration::from_millis(blink_speed.into())).await;   
    }
}

fn get_new_blink_speed(blink_speed_index: &mut usize) -> u32 {
    if *blink_speed_index == BLINK_SPEEDS_MS.len() - 1 {
        *blink_speed_index = 0;
    }

    *blink_speed_index += 1;

    BLINK_SPEEDS_MS[*blink_speed_index]
}

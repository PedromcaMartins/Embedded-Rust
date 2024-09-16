#![no_std]
#![no_main]

use core::sync::atomic::{AtomicU32, Ordering};

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::{exti::{AnyChannel, Channel, ExtiInput}, gpio::{AnyPin, Input, Level, Output, Pin, Pull, Speed}};
use embassy_time::{Duration, Timer};
use {defmt_rtt as _, panic_probe as _}; // global logger

const BLINK_SPEEDS_MS: [u32; 6] = [100, 150, 200, 250, 200, 150];
static BLINK_MS: AtomicU32 = AtomicU32::new(0);


struct Peripherals {
    led_pin: AnyPin,
    button_pin: AnyPin,
    button_exti_channel: AnyChannel,
}

fn initialize_peripherals() -> Peripherals {
    let p = embassy_stm32::init(Default::default());

    Peripherals {
        led_pin: p.PB7.degrade(),
        button_pin: p.PC13.degrade(),
        button_exti_channel: p.EXTI13.degrade(),
    }
}


#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = initialize_peripherals();
    info!("Hello, World!");

    spawner.spawn(led_task(p.led_pin)).unwrap();
    spawner.spawn(button_task(p.button_pin, p.button_exti_channel)).unwrap();
}

#[embassy_executor::task]
async fn led_task(led_pin: AnyPin) -> ! {
    let mut led = Output::new(led_pin, Level::High, Speed::Low);

    loop {
        led.toggle();

        let blink_speed = BLINK_MS.load(Ordering::Relaxed);
        Timer::after(Duration::from_millis(blink_speed.into())).await;
    }
}

#[embassy_executor::task]
async fn button_task(button_pin: AnyPin, button_exti_channel: AnyChannel) -> ! {
    let button = Input::new(button_pin, Pull::None);
    let mut button = ExtiInput::new(button, button_exti_channel);

    let mut iter_blinking_speed = BLINK_SPEEDS_MS.iter().cycle();

    loop {
        if let Some(&speed) = iter_blinking_speed.next() {
            BLINK_MS.store(speed, Ordering::Relaxed)
        }

        button.wait_for_rising_edge().await;
    }
}
#![no_std]
#![no_main]

mod fmt;

use core::sync::atomic::AtomicBool;

use melody::nokia;
#[cfg(not(feature = "defmt"))]
use panic_halt as _;
#[cfg(feature = "defmt")]
use {defmt_rtt as _, panic_probe as _};

use embassy_executor::Spawner;
use embassy_stm32::{exti::ExtiInput, gpio::{AnyPin, Input, Level, Output, OutputType, Pull, Speed}, time::Hertz, timer::{simple_pwm::{PwmPin, SimplePwm}, Channel}};
use embassy_time::{Duration, Timer};
use fmt::info;

static BUTTON_PRESSED: AtomicBool = AtomicBool::new(false);

mod melody;
mod pitches;

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    info!("Hello, World!");

    let p = embassy_stm32::init(Default::default());

    let mut buzzer_pwm = SimplePwm::new(
        p.TIM5, 
        None, 
        None, 
        None, 
        Some(PwmPin::new_ch4(p.PA3, OutputType::PushPull)), 
        Hertz(100_000), 
        Default::default()
    );
    buzzer_pwm.enable(Channel::Ch4);

    info!("PWM initialized");

    let user_button = ExtiInput::new(
        Input::new(p.PC13, Pull::Down), 
        p.EXTI13
    );


    spawner.spawn(enable_buzzer(user_button)).unwrap();

    loop {
        match BUTTON_PRESSED.load(core::sync::atomic::Ordering::Relaxed) {
            false => melody::play_melody(&mut buzzer_pwm, &melody::nokia::MELODY, &melody::nokia::DURATIONS).await,
            true => {
                melody::no_tone(&mut buzzer_pwm).await;
                Timer::after_millis(500).await;
            }
        }
    }
}

#[embassy_executor::task]
async fn enable_buzzer(mut user_button: ExtiInput<'static, embassy_stm32::peripherals::PC13>) -> ! {
    loop {
        user_button.wait_for_high().await;
        BUTTON_PRESSED.store(true, core::sync::atomic::Ordering::Relaxed);
        Timer::after_millis(500).await;

        user_button.wait_for_high().await;
        BUTTON_PRESSED.store(false, core::sync::atomic::Ordering::Relaxed);
        Timer::after_millis(500).await;
    }
}
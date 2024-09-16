use embassy_stm32::{time::Hertz, timer::{simple_pwm::SimplePwm, Channel}};
use embassy_time::{Duration, Timer};
use crate::pitches::*;

pub mod end_of_begining;
pub mod happy_birthday;
pub mod nokia;
pub mod pacman;
pub mod tokio_drift;
pub mod tetris;
pub mod doom;
pub mod subway_surfers;
pub mod harry_potter;


pub async fn play_melody(
    buzzer_pwm: &mut SimplePwm<'static, embassy_stm32::peripherals::TIM5>, 
    melody: &[Hertz], 
    durations: &[u32]
) {
    for (&note, &duration) in melody.iter().zip(durations.iter()) {
        play_note(buzzer_pwm, note, Duration::from_millis((1000 / duration) as u64)).await;
        let pause_between_notes = Duration::from_millis((1300 / duration) as u64);
        Timer::after(pause_between_notes).await;
        no_tone(buzzer_pwm).await;
    }
}

async fn play_note(
    buzzer_pwm: &mut SimplePwm<'static, embassy_stm32::peripherals::TIM5>, 
    freq: Hertz, 
    duration: Duration
) {
    tone(buzzer_pwm, freq).await;
    Timer::after(duration).await;
    no_tone(buzzer_pwm).await;
}

async fn tone(
    buzzer_pwm: &mut SimplePwm<'static, embassy_stm32::peripherals::TIM5>, 
    freq: Hertz
) {
    match freq {
        Hertz(0) => no_tone(buzzer_pwm).await,
        _ => {
            buzzer_pwm.set_frequency(freq);
            buzzer_pwm.set_duty(Channel::Ch4, buzzer_pwm.get_max_duty()/2);
        },
    }
}

pub async fn no_tone(
    buzzer_pwm: &mut SimplePwm<'static, embassy_stm32::peripherals::TIM5>
) {
    buzzer_pwm.set_duty(Channel::Ch4, 0);
}
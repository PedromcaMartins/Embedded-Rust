#![no_std]
#![no_main]

use cortex_m::asm::delay;
use cortex_m_rt::entry;
use rtt_target::{rtt_init_print, rprintln};
use stm32f4xx_hal::{
    prelude::*, 
    pac
};
extern crate panic_halt;

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let gpiob = dp.GPIOB.split();
    let gpioc = dp.GPIOC.split();

    let mut green_led = gpiob.pb0.into_push_pull_output();
    let mut blue_led = gpiob.pb7.into_push_pull_output();
    let mut red_led = gpiob.pb14.into_push_pull_output();
    let user_button = gpioc.pc13.into_input();

    rtt_init_print!();
    rprintln!("Hello, world!");

    let mut count = 0;

    loop {
        
        if user_button.is_high() {
            rprintln!("Button pressed");
            
            if count == 0 {
                rprintln!("Green LED on");
                green_led.set_high();
                blue_led.set_low();
                red_led.set_low();
            } else if count == 1 {
                rprintln!("Blue LED on");
                green_led.set_low();
                blue_led.set_high();
                red_led.set_low();
            } else if count == 2 {
                rprintln!("Red LED on");
                green_led.set_low();
                blue_led.set_low();
                red_led.set_high();
                count = -1;
            }

            count += 1;
        }

        delay(300*5000);
    }
}

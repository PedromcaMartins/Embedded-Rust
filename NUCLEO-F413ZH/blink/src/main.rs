#![no_std]
#![no_main]

use cortex_m::asm::nop;
use cortex_m_rt::entry;
use rtt_target::{rtt_init_print, rprintln};
use stm32f4xx_hal::{
    gpio::GpioExt, pac
};
extern crate panic_halt;

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let gpiob = dp.GPIOB.split();
    let mut green_led = gpiob.pb0.into_push_pull_output();

    rtt_init_print!();
    rprintln!("Hello, world!");

    let mut x: usize = 0;

    loop {
        rprintln!("x = {}", x);
        x += 1;
        green_led.toggle();

        for _ in 0..100_000 {
            nop();
        }
    }
}

#![no_std]
#![no_main]

use cortex_m::asm::nop;
use cortex_m_rt::entry;
use rtt_target::{rtt_init_print, rprintln};
use stm32f4xx_hal::pac;
extern crate panic_halt;

#[entry]
fn main() -> ! {
    let _dp = pac::Peripherals::take().unwrap();

    rtt_init_print!();
    rprintln!("Hello, world!");

    loop {

        for _ in 0..1000_000 {
            nop();
        }
    }
}

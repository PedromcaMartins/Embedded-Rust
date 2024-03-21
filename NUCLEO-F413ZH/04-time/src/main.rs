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

    rtt_init_print!();
    rprintln!("Hello, world!");

    loop {
        
        rprintln!("time: {:?}", cortex_m::peripheral::DWT::get_cycle_count());

        delay(300*5000);
    }
}

#![no_std]
#![no_main]

use core::sync::atomic::{AtomicU32, Ordering};
use core::cell::RefCell;
use cortex_m::interrupt::{free as disable_interrupts, Mutex};
use cortex_m::peripheral::syst::SystClkSource;
use cortex_m_rt::{entry, exception};
use rtt_target::{rtt_init_print, rprintln};
use stm32f4xx_hal::{
    pac, prelude::*
};
extern crate panic_halt;



static MILLIS: Mutex<RefCell<AtomicU32>> = Mutex::new(RefCell::new(AtomicU32::new(0)));

/// Returns the current value of the millisecond counter.
fn get_tick() -> u32 {
    disable_interrupts(|cs| {
        MILLIS.borrow(cs).borrow().load(Ordering::Relaxed)
    })
}


#[entry]
fn main() -> ! {

    rtt_init_print!();
    rprintln!("Hello, world!");


    let cp = cortex_m::Peripherals::take().unwrap();
    let mut syst = cp.SYST;

    // Configure the systick timer for a 1ms tick
    syst.set_clock_source(SystClkSource::Core);
    syst.set_reload(48_000_000 / 1_000 - 1); // Adjust the reload value based on your core clock
    syst.enable_counter();
    syst.enable_interrupt();


    let dp = pac::Peripherals::take().unwrap();

    let rcc = dp.RCC.constrain();
    let clocks = rcc
        .cfgr
        .use_hse(48.MHz())
        .freeze();

    let mut delay = dp.TIM2.delay_ms(&clocks);

    loop {

        rprintln!("Tick: {}", get_tick());

        delay.delay_ms(10);
    }
}

#[cortex_m_rt::exception]
fn SysTick() {
    // Increment the millisecond counter safely within an interrupt context
    disable_interrupts(|cs| {
        MILLIS.borrow(cs).borrow_mut().fetch_add(1, Ordering::SeqCst);
    });
}

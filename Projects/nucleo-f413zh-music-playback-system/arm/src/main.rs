#![no_std]
#![no_main]

mod fmt;
mod timer;

use io_mapping::{IOMapping, LedPin};
#[cfg(not(feature = "defmt"))]
use panic_halt as _;
use shared_lib::tasks::LedTask;
use timer::EmbassyTimer;
#[cfg(feature = "defmt")]
use {defmt_rtt as _, panic_probe as _};

use embassy_executor::Spawner;
use embassy_stm32::gpio::{self, Output};
use led_drivre::Led;

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());

    let io_mapping = IOMapping::init(p);

    let led_task = LedTask::new(Led::new(Output::new(io_mapping.led, gpio::Level::Low, gpio::Speed::Low)));

    spawner.spawn(led_task_spawn(led_task)).unwrap();
}

mod io_mapping {
    use embassy_stm32::peripherals::PB7;

    pub type LedPin = PB7;

    pub struct IOMapping {
        pub led: LedPin, 
    }

    impl IOMapping {
        pub fn init(p: embassy_stm32::Peripherals) -> Self {
            Self {
                led: p.PB7,
            }
        }
    }
}

#[embassy_executor::task]
pub async fn led_task_spawn(task: LedTask<Led<'static, LedPin>>) -> ! {
    task.run(EmbassyTimer::default()).await
}

mod led_drivre {
    use embassy_stm32::gpio::{Output, Pin};
    use shared_lib::traits::LedDriver;

    pub struct Led<'d, T: Pin> {
        output: Output<'d, T>,
    }

    impl<'d, T: Pin> Led<'d, T> {
        pub fn new(output: Output<'d, T>) -> Self {
            Self { 
                output, 
            }
        }
    }

    impl<'d, T: Pin> LedDriver for Led<'d, T> {
        fn toggle(&mut self) {
            self.output.toggle();
        }
    }
}
use defmt::info;
use embassy_stm32::gpio::{Level, Output, Pin};
use embassy_time::{Duration, Instant};

pub struct Led<'d, T: Pin> {
    output: Output<'d, T>,
    default_level: Level
}

impl<'d, T: Pin> Led<'d, T> {
    pub fn new(output: Output<'d, T>, default_level: Level) -> Self {
        Self { 
            output, 
            default_level 
        }
    }

    pub fn toggle(&mut self) {
        self.output.toggle();
    }

    pub fn turn_on(&mut self) {
        match self.default_level {
            Level::High => self.output.set_low(),
            Level::Low => self.output.set_high()
        }
    }

    pub fn turn_off(&mut self) {
        self.output.set_level(self.default_level);
    }
}

impl<'d, T: Pin> Led<'d, T> {
    pub fn test(&mut self) {
        info!("Initiating Led Unit Test");

        self.test_on_off();
        self.test_blink();

        self.turn_off();
        info!("Test completed")
    }

    pub fn test_on_off(&mut self) {
        let timeout = Duration::from_millis(5_000);

        info!("Led off for 5 seconds");
        self.turn_off();
        let start = Instant::now();
        while Instant::now() - start < timeout {}

        info!("Led on for 5 seconds");
        self.turn_on();
        let start = Instant::now();
        while Instant::now() - start < timeout {}
    }

    pub fn test_blink(&mut self) {
        info!("Led blink every second, for 5 seconds");

        let timeout = Duration::from_millis(500);

        for _ in 0..10 {
            self.toggle();
            let start = Instant::now();
            while Instant::now() - start < timeout {}
        }
    }
}
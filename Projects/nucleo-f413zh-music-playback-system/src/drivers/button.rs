use defmt::info;
use embassy_stm32::gpio::{Input, Level, Pin};
use embassy_time::{Duration, Instant};

pub struct Button<'d, T: Pin> {
    input: Input<'d, T>,
    default_level: Level
}

impl<'d, T: Pin> Button<'d, T> {
    pub fn init(input: Input<'d, T>, default_level: Level) -> Self {
        Button {
            input,
            default_level
        }
    }

    pub fn is_pressed_down(&self) -> bool {
        self.input.get_level() != self.default_level
    }

    pub fn is_released(&self) -> bool {
        !self.is_pressed_down()
    }
}

impl<'d, T: Pin> Button<'d, T> {
    pub fn test(&self) {
        let passed = true;

        info!("Initiating Button Unit Test");

        if !self.test_pressed_down() {
            info!("test_pressed_down failed");
        }

        match passed {
            true => info!("Test passed"),
            false => info!("Test failed"),
        }
    }

    fn test_pressed_down(&self) -> bool {
        info!("Please release the button");
        while !self.is_released(){};

        info!("Please press the button");
        while !self.is_pressed_down(){};

        info!("Please release the button");
        while !self.is_released(){};

        let start = Instant::now();
        let timeout = Duration::from_millis(150);
        while Instant::now() - start < timeout {
            if self.is_pressed_down() {
                return false
            }
        }

        true
    }
}
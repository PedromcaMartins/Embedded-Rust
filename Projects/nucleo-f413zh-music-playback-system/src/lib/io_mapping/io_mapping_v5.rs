use defmt::trace;
use defmt::unwrap;
use embassy_stm32::adc::Adc;
use embassy_stm32::bind_interrupts;
use embassy_stm32::exti::ExtiInput;
use embassy_stm32::gpio::Level;
use embassy_stm32::gpio::Pull;
use embassy_stm32::gpio::Input;
use embassy_stm32::gpio::Output;
use embassy_stm32::peripherals;
use embassy_stm32::gpio;
use embassy_stm32::usart;
use embassy_stm32::usart::Uart;
use embassy_time::Delay;
use embassy_time::Duration;

pub mod types {
    use embassy_stm32::peripherals::ADC1;
    use embassy_stm32::peripherals::DMA1_CH1;
    use embassy_stm32::peripherals::DMA1_CH3;
    use embassy_stm32::peripherals::PA3;
    use embassy_stm32::peripherals::PB7;
    use embassy_stm32::peripherals::PC11;
    use embassy_stm32::peripherals::PC12;
    use embassy_stm32::peripherals::PC13;
    use embassy_stm32::peripherals::PD3;
    use embassy_stm32::peripherals::PD4;
    use embassy_stm32::peripherals::PD5;
    use embassy_stm32::peripherals::PD8;
    use embassy_stm32::peripherals::PD9;
    use embassy_stm32::peripherals::USART3;

    pub type ButtonPrevPin = PD3;
    pub type ButtonPausePlayPin = PD4;
    pub type ButtonNextPin = PD5;

    pub type VolumeDialAdc = ADC1;
    pub type VolumeDialPin = PA3;

    pub type PCUart = USART3;
    pub type PCUartRxPin = PD9;
    pub type PCUartRxDma = DMA1_CH1;
    pub type PCUartTxPin = PD8;
    pub type PCUartTxDma = DMA1_CH3;
}

use types::*;

#[cfg(not(feature = "io_mapping_test"))]
bind_interrupts!(struct Irqs {
    USART3 => usart::InterruptHandler<PCUart>;
});
use crate::drivers::Button;
use crate::drivers::InterruptInput;

#[cfg(feature = "io_mapping_test")]
use super::Irqs;

const DEBOUNCE_DURATION: Duration = Duration::from_millis(50);

pub struct IOMapping<'d> {
    pub debounce_duration: Duration,
    pub button_prev: Button<'d, ButtonPrevPin, InterruptInput<'d, ButtonPrevPin>>,
    pub button_pause_play: Button<'d, ButtonPausePlayPin, InterruptInput<'d, ButtonPausePlayPin>>,
    pub button_next: Button<'d, ButtonNextPin, InterruptInput<'d, ButtonNextPin>>,
    pub adc_manager: Adc<'d, VolumeDialAdc>,
    pub volume_dial_pin: VolumeDialPin,
    pub pc_uart: Uart<'d, PCUart, PCUartTxDma, PCUartRxDma>,
}

impl<'d> IOMapping<'d> {
    pub fn init(p: embassy_stm32::Peripherals) -> Self {
        trace!("IO Mapping v5 initialized!");

        Self {
            debounce_duration: DEBOUNCE_DURATION,
            button_prev: Button::new_interrupt(
                ExtiInput::new(Input::new(p.PD3, Pull::Up), p.EXTI3),
                Level::High,
                DEBOUNCE_DURATION
            ),
            button_pause_play: Button::new_interrupt(
                ExtiInput::new(Input::new(p.PD4, Pull::Up), p.EXTI4),
                Level::High,
                DEBOUNCE_DURATION
            ),
            button_next: Button::new_interrupt(
                ExtiInput::new(Input::new(p.PD5, Pull::Up), p.EXTI5),
                Level::High,
                DEBOUNCE_DURATION
            ),
            adc_manager: Adc::new(p.ADC1, &mut Delay),
            volume_dial_pin: p.PA3,
            pc_uart: unwrap!(Uart::new(p.USART3, p.PD9, p.PD8, Irqs, p.DMA1_CH3, p.DMA1_CH1, usart::Config::default())),
        }
    }
}

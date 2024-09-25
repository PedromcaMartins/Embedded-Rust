use defmt::debug;
use embassy_stm32::adc::Adc;
use embassy_time::Duration;

use crate::{drivers::{AdcManager, Button, InterruptInput, Potentiometer}, io_mapping::types::{ButtonNextPin, ButtonPausePlayPin, ButtonPrevPin, VolumeDialAdc, VolumeDialPin}, models::UserInput};

#[embassy_executor::task]
pub async fn user_input_task_spawn(
    debounce_duration: Duration,
    button_prev: Button<'static, ButtonPrevPin, InterruptInput<'static, ButtonPrevPin>>,
    button_pause_play: Button<'static, ButtonPausePlayPin, InterruptInput<'static, ButtonPausePlayPin>>,
    button_next: Button<'static, ButtonNextPin, InterruptInput<'static, ButtonNextPin>>,
    adc_manager: Adc<'static, VolumeDialAdc>,
    volume_dial_pin: VolumeDialPin,
) -> ! {
    debug!("user input task spawned");

    let mut adc_manager = AdcManager::new(adc_manager);
    let mut user_input = UserInput::new(
        button_prev,
        button_pause_play,
        button_next,
        Potentiometer::new(&mut adc_manager, volume_dial_pin, debounce_duration), 
    );

    loop {
        let _command = user_input.wait_for_user_input().await;
    }
}

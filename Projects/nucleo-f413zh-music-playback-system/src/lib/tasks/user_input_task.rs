use defmt::{debug, unwrap, warn};
use embassy_futures::select::{select, Either};
use embassy_stm32::adc::Adc;
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, pubsub::{PubSubChannel, WaitResult}};
use embassy_time::Duration;

use crate::{drivers::{AdcManager, Button, InterruptInput, Potentiometer}, io_mapping::types::{ButtonNextPin, ButtonPausePlayPin, ButtonPrevPin, VolumeDialAdc, VolumeDialPin}, models::{CliCommand, UserInput, UserInputCommand}, tasks::cli_task::CHANNEL_CLI_COMMAND};

const CHANNEL_CAP: usize = 3;
const CHANNEL_SUBS: usize = 2;
const CHANNEL_PUBS: usize = 1;

/// Channel used by the cli task to broadcast the cli commands to the other tasks
pub static CHANNEL_USER_INPUT_COMMAND: PubSubChannel<
    CriticalSectionRawMutex, 
    UserInputCommand, 
    CHANNEL_CAP, 
    CHANNEL_SUBS, 
    CHANNEL_PUBS
> = PubSubChannel::new();


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

    let user_input_publisher = unwrap!(CHANNEL_USER_INPUT_COMMAND.publisher());
    let mut cli_consumer = unwrap!(CHANNEL_CLI_COMMAND.subscriber());

    loop {
        let res = select(
            cli_consumer.next_message(), 
            user_input.wait_for_user_input()
        ).await;

        match res {
            Either::First(res) => match res {
                    WaitResult::Message(CliCommand::UserInputTest) => {
                        debug!("cli command received: {}", CliCommand::UserInputTest);
                        user_input.test().await;
                    },
                    WaitResult::Message(_) => (),
                    WaitResult::Lagged(n) => warn!("channel_cli_command lagged {} messages", n),
            },
            Either::Second(command) => {
                debug!("user input command: {}", command);
                user_input_publisher.publish(command).await;
            },
        }
    }
}

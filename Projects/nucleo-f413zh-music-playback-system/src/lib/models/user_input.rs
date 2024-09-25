use defmt::{debug, info, Format};
use embassy_futures::select::{select4, Either4};

use crate::{drivers::{Button, InterruptInput, Potentiometer}, io_mapping::types::{ButtonNextPin, ButtonPausePlayPin, ButtonPrevPin, VolumeDialAdc, VolumeDialPin}};

#[derive(Format)]
pub enum UserInputCommand {
    PrevSong,
    PausePlaySong,
    NextSong,
    ChangeVolume(u8),
}

pub struct UserInput<'d, 'b> {
    button_prev: Button<'d, ButtonPrevPin, InterruptInput<'d, ButtonPrevPin>>,
    button_pause_play: Button<'d, ButtonPausePlayPin, InterruptInput<'d, ButtonPausePlayPin>>,
    button_next: Button<'d, ButtonNextPin, InterruptInput<'d, ButtonNextPin>>,
    volume_dial: Potentiometer<'d, 'b, VolumeDialAdc, VolumeDialPin>,
}

impl<'d, 'b> UserInput<'d, 'b> {
    pub fn new(
        button_prev: Button<'d, ButtonPrevPin, InterruptInput<'d, ButtonPrevPin>>,
        button_pause_play: Button<'d, ButtonPausePlayPin, InterruptInput<'d, ButtonPausePlayPin>>,
        button_next: Button<'d, ButtonNextPin, InterruptInput<'d, ButtonNextPin>>,
        volume_dial: Potentiometer<'d, 'b, VolumeDialAdc, VolumeDialPin>,
    ) -> Self {
        Self {
            button_prev,
            button_pause_play,
            button_next,
            volume_dial,
        }
    }

    pub async fn wait_for_user_input(&mut self) -> UserInputCommand {
        let futures = select4(
            self.button_prev.wait_for_press_down(), 
            self.button_pause_play.wait_for_press_down(), 
            self.button_next.wait_for_press_down(), 
            self.volume_dial.wait_for_change_in_position(), 
        ).await;

        let command = match futures {
            Either4::First(())  => UserInputCommand::PrevSong,
            Either4::Second(()) => UserInputCommand::PausePlaySong,
            Either4::Third(())  => UserInputCommand::NextSong,
            Either4::Fourth(position) => UserInputCommand::ChangeVolume(position),
        };

        debug!("user input: {}", command);
        command
    }

    pub async fn test(&mut self) {
        info!("Testing User Input");

        debug!("Testing prev song button");
        self.button_prev.test_interrupt().await;

        debug!("Testing pause play button");
        self.button_pause_play.test_interrupt().await;

        debug!("Testing next song button");
        self.button_next.test_interrupt().await;

        debug!("Testing volume dial potentiometer");
        self.volume_dial.test();

        info!("Tests concluded!")
    }
}

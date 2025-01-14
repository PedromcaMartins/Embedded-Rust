use embassy_stm32::timer::CaptureCompare16bitInstance;
use song_loader::{Song, SongLoader};
use crate::drivers::BuzzerEncoder;

mod song_loader;

pub struct MusicPlayer<'d, T>
where 
    T: CaptureCompare16bitInstance
{
    buzzer: BuzzerEncoder<'d, T>,
    song_loader: SongLoader,
    current_song: Option<&'static Song>,
}

impl <'d, T> MusicPlayer<'d, T>
where 
    T: CaptureCompare16bitInstance
{
    pub fn new(buzzer: BuzzerEncoder<'d, T>) -> Self {
        Self {
            buzzer,
            song_loader: SongLoader::new(),
            current_song: None
        }
    }

    pub fn play_song(&mut self) {

    }
}

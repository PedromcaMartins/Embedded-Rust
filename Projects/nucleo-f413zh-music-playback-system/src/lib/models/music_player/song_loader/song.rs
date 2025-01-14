#![allow(unused)]

use core::slice::Iter;

use embassy_stm32::time::Hertz;
use embassy_time::Duration;

mod pitches;
use pitches::*;

mod nokia;

pub struct Song {
    melody: &'static [Hertz], 
    durations: &'static [u32], 
    name: &'static str, 
}

impl Song {
    pub fn get_melody_iter(&self) -> Iter<'_, Hertz> {
        self.melody.iter()
    }

    pub fn get_duration_iter(&self) -> Iter<'_, u32> {
        self.durations.iter()
    }

    pub fn get_song_name(&self) -> &'static str {
        self.name
    }

    pub fn calculate_length(&mut self) -> Duration {
        Duration::from_millis(
            self.durations.iter()
                .map(|&duration| 1300 / duration)
                .sum::<u32>() as u64
        )
    }
}

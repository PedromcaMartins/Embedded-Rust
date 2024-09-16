use super::*;

// Melody and durations (translated from the Arduino code)
pub static MELODY: [Hertz; 13] = [
    NOTE_E5, NOTE_D5, NOTE_FS4, NOTE_GS4, 
    NOTE_CS5, NOTE_B4, NOTE_D4, NOTE_E4, 
    NOTE_B4, NOTE_A4, NOTE_CS4, NOTE_E4,
    NOTE_A4
];

pub static DURATIONS: [u32; 13] = [
    8, 8, 4, 4,
    8, 8, 4, 4,
    8, 8, 4, 4,
    2
];
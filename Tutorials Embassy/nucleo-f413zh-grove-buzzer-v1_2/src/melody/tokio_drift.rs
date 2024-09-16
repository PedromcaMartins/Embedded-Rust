use super::*;

// Melody and durations (translated from the Arduino code)
pub static MELODY: [Hertz; 65] = [
    NOTE_AS4, REST, NOTE_AS4, REST, NOTE_AS4, REST, NOTE_AS4, REST,
    NOTE_AS4, NOTE_B4, NOTE_DS5,
    NOTE_AS4, REST, NOTE_AS4, REST,
    NOTE_AS4, NOTE_B4, NOTE_DS5,
    NOTE_AS4, REST, NOTE_AS4, REST,
    NOTE_AS4, NOTE_B4, NOTE_DS5,
    NOTE_AS4, REST, NOTE_AS4, REST,
    NOTE_AS4, NOTE_B4, NOTE_DS5,
    NOTE_F5, REST, NOTE_F5, REST,
    NOTE_GS5, NOTE_FS5, NOTE_F5,
    NOTE_AS4, REST, NOTE_AS4, REST,
    NOTE_GS5, NOTE_FS5, NOTE_F5,
    NOTE_AS4, REST, NOTE_AS4, REST,
    NOTE_AS4, NOTE_B4, NOTE_DS5,
    NOTE_AS4, REST, NOTE_AS4, REST,
    NOTE_AS4, NOTE_B4, NOTE_DS5,
    NOTE_AS4, REST, NOTE_AS4, REST,
    REST
];

pub static DURATIONS: [u32; 65] = [
    4, 4, 4, 4, 4, 4, 4, 4,
    3, 3, 4,
    4, 4, 4, 4,
    3, 3, 4,
    4, 4, 4, 4,
    3, 3, 4,
    4, 4, 4, 4,
    3, 3, 4,
    4, 4, 4, 4,
    3, 3, 4,
    4, 4, 4, 4,
    3, 3, 4,
    4, 4, 4, 4,
    3, 3, 4,
    4, 4, 4, 4,
    3, 3, 4,
    4, 4, 4, 4,
    1
];
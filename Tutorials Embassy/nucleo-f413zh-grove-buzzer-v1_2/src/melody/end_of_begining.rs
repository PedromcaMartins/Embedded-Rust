use super::*;

// Melody and durations (translated from the Arduino code)
pub static MELODY: [Hertz; 101] = [
    REST,
    
    NOTE_D5, NOTE_D5, NOTE_E5, NOTE_FS5, NOTE_G5, NOTE_A5, NOTE_E5, NOTE_D5, NOTE_FS5, NOTE_E5, NOTE_D5, REST,
    NOTE_B5, NOTE_D5, NOTE_D5, NOTE_D5, NOTE_CS5, NOTE_B5, NOTE_A5, NOTE_B5, NOTE_CS5, NOTE_D5, NOTE_B5, NOTE_A5,
    NOTE_D5, NOTE_D5, NOTE_E5, NOTE_FS5, NOTE_G5, NOTE_A5, NOTE_E5, NOTE_D5, NOTE_FS5, NOTE_E5, NOTE_D5,
    NOTE_A5, NOTE_D5, REST, NOTE_A5, NOTE_D5, REST, NOTE_A5, NOTE_D5, REST, NOTE_A5, NOTE_D5, NOTE_B5,
    NOTE_CS5, NOTE_CS5, NOTE_CS5, NOTE_CS5, NOTE_CS5,
    NOTE_A5, NOTE_CS5, NOTE_A5, NOTE_CS5, NOTE_A5, NOTE_CS5,
    NOTE_D5, NOTE_D5, REST,
    NOTE_CS5, NOTE_CS5, NOTE_CS5,
    NOTE_D5, NOTE_CS5, NOTE_D5, NOTE_CS5, NOTE_D5, NOTE_FS5,
    NOTE_D5, NOTE_CS5, NOTE_D5, NOTE_CS5, NOTE_D5, NOTE_CS5,
    NOTE_CS5, NOTE_D5, NOTE_CS5, NOTE_D5, NOTE_CS5, NOTE_CS5,
    NOTE_D5, NOTE_CS5, NOTE_CS5, NOTE_D5, NOTE_CS5, NOTE_CS5,
    NOTE_CS5, NOTE_D5, NOTE_CS5, NOTE_CS5, NOTE_G5, NOTE_FS5,
    NOTE_D5, NOTE_E5, NOTE_E5, NOTE_E5, NOTE_D5,
  
    REST
];

pub static DURATIONS: [u32; 101] = [
    4,
  
    4, 4, 4, 4, 4, 4, 2, 4, 4, 4, 1, 4,
    4, 4, 4, 4, 4, 4, 2, 4, 4, 4, 1, 2,
    4, 4, 4, 4, 4, 4, 2, 4, 4, 4, 4,
    4, 2, 4, 4, 2, 4, 4, 2, 4, 4, 4, 2,
    4, 4, 4, 4, 4,
    4, 4, 4, 4, 4, 4,
    4, 2, 2,
    4, 4, 4,
    4, 4, 4, 4, 4, 4,
    4, 4, 4, 4, 4, 4,
    4, 4, 4, 4, 4, 4,
    4, 4, 4, 4, 4, 4,
    4, 4, 4, 4, 4, 4,
    4, 4, 4, 4, 4,
  
    1
];
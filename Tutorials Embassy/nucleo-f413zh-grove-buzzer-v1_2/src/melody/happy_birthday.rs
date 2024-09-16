use super::*;

// Melody and durations (translated from the Arduino code)
pub static MELODY: [Hertz; 25] = [
    NOTE_C4, NOTE_C4, 
    NOTE_D4, NOTE_C4, NOTE_F4,
    NOTE_E4, NOTE_C4, NOTE_C4, 
    NOTE_D4, NOTE_C4, NOTE_G4,
    NOTE_F4, NOTE_C4, NOTE_C4,
    
    NOTE_C5, NOTE_A4, NOTE_F4, 
    NOTE_E4, NOTE_D4, NOTE_AS4, NOTE_AS4,
    NOTE_A4, NOTE_F4, NOTE_G4,
    NOTE_F4
];

pub static DURATIONS: [u32; 25] = [
    4, 8, 
    4, 4, 4,
    2, 4, 8, 
    4, 4, 4,
    2, 4, 8,
    
    4, 4, 4, 
    4, 4, 4, 8,
    4, 4, 4,
    2
];
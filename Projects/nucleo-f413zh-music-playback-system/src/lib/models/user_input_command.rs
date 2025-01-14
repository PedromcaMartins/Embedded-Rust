use defmt::Format;

#[derive(Format, Clone)]
pub enum UserInputCommand {
    PrevSong,
    PausePlaySong,
    NextSong,
    ChangeVolume(u8),
}

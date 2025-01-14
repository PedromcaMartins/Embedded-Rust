mod song;
pub use song::*;

pub struct SongLoader {
    songs: [Song; 1],
    current_song_idx: Option<usize>,
}

impl SongLoader {
    pub fn new() -> Self {
        let songs = [Song::nokia()];

        Self {
            songs,
            current_song_idx: None,
        }
    }

    pub fn get_next_song(&mut self) -> Option<&Song> {
        self.get_song_by_offset(1)
    }

    pub fn get_prev_song(&mut self) -> Option<&Song> {
        self.get_song_by_offset(-1)
    }

    fn get_song_by_offset(&mut self, offset: isize) -> Option<&Song> {
        let new_index = match self.current_song_idx {
            None => 0, 
            Some(idx) => (idx as isize + offset) as usize, 
        };

        let new_song = self.songs.get(new_index);

        match new_song {
            Some(_) => self.current_song_idx = Some(new_index),
            None => self.current_song_idx = None,
        }

        new_song
    }
}

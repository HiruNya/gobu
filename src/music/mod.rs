//! The music that is stored and played by the game.

use std::{
    fs::File,
    io::Read,
    path::Path,
    sync::{
        Arc,
        mpsc::SendError,
    },
    collections::HashMap,
};
use error::MusicError;

mod player;

use self::player::{
    Player,
    PlayerMsg,
};

/// The struct in charge of handling the music for the game.
pub struct Music {
    /// The player that plays the music.
    pub player: Player,
    /// A collection of music files in the form of buffered data.
    pub library: HashMap<String, Arc<[u8]>>,
}
impl Music {
    /// Create a new [`Music`] struct. Returns an error if not possible.
    pub fn new() -> Result<Self, MusicError> {
        Ok(Music {
            player: Player::new(),
            library: HashMap::new(),
        })
    }
    /// Sets the music that is to be played.
    pub fn set_music(&mut self, music: &str) -> Result<(), SendError<PlayerMsg>> {
        if let Some(m) = self.library.get(music) {
            return self.player.set(m.clone())
        }
        Ok(())
    }
    /// Add music from a file.
    pub fn add_music_from_file<P: AsRef<Path>, S: ToString>(&mut self, name: S, path: P)
        -> Result<(), MusicError> {
        let mut buf = Vec::new();
        File::open(path)?.read_to_end(&mut buf)?;
        self.add_music(name.to_string(), buf);
        Ok(())
    }
    /// Add the buffered data of a music file.
    pub fn add_music(&mut self, name: String, music: Vec<u8>) {
        self.library.insert(name, Arc::from(music));
    }
}
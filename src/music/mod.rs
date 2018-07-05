//! The music that is stored and played by the game.

use std::{
    fs::File,
    io::{
        Read,
        Cursor,
    },
    path::Path,
    sync::Arc,
    collections::HashMap,
};
use error::MusicError;
use rodio::{
    Sink,
    Source,
    Decoder,
    default_output_device,
};

/// The struct in charge of handling the music for the game.
pub struct Music {
    /// The Rodio Sink that music can be played in.
    pub sink: Sink,
    /// A collection of music data in bytes.
    pub library: HashMap<String, Arc<[u8]>>,
    /// Whether the music set will loop or not.
    pub loop_: bool,
}
impl Music {
    /// Create a new [`Music`] struct. Returns an error if not possible.
    pub fn new() -> Result<Self, MusicError> {
        if let Some(d) = default_output_device() {
            Ok(Music {
                sink: Sink::new(&d),
                library: HashMap::new(),
                loop_: true,
            })
        } else {
            Err(MusicError::NoDefaultOutputDeviceFound)
        }
    }
    /// Sets the music that is to be played.
    pub fn set_music(&mut self, music: &str) -> Result<(), MusicError> {
        if let Some(m) = self.library.get(music) {
            let c = Cursor::new(m.clone());
            let d = Decoder::new(c)?;
            if self.loop_ {
                self.sink.append(d.repeat_infinite());
            } else {
                self.sink.append(d);
            }
            self.sink.play();
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
    /// If ``True`` will cause the music that is played to loop infinitely.
    pub fn set_loop(&mut self, loop_: bool) {
        self.loop_ = loop_;
    }
}
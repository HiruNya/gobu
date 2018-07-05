//! A collection of all the errors used in this crate.
//!
//! These crates are all just enums that have all the possible different errors from
//! other crates.

use std::io::Error as IoError;
use toml::de::Error as TomlError;
use nom::Err as NomErr;
use nom::ErrorKind;
use rodio::decoder::DecoderError;

/// An error created when an error occurs reading the TOML files.
#[derive(Debug)]
pub enum ConfigImportError {
    /// Error caused by trying to opening the file.
    Io(IoError),
    /// Error caused by reading the TOML file.
    Toml(TomlError),
}
impl From<IoError> for ConfigImportError {
    fn from(err: IoError) -> Self {
        ConfigImportError::Io(err)
    }
}
impl From<TomlError> for ConfigImportError {
    fn from(err: TomlError) -> Self {
        ConfigImportError::Toml(err)
    }
}

/// An error created when an error occurs reading the script file.
#[derive(Debug)]
pub enum ScriptImportError {
    /// Error caused by trying to opening the file.
    Io(IoError),
    /// Error caused by trying to parse the script.
    Nom(ErrorKind)
}
impl From<IoError> for ScriptImportError {
    fn from(err: IoError) -> Self {
        ScriptImportError::Io(err)
    }
}
impl<I: AsRef<str>> From<NomErr<I>> for ScriptImportError {
    fn from(err: NomErr<I>) -> Self {
        ScriptImportError::Nom(err.into_error_kind())
    }
}

/// An error created when an error occurs reading the TOML files.
#[derive(Debug)]
pub enum ScriptConfigImportError {
    /// Error caused by trying to opening the file.
    Io(IoError),
    /// Error caused by trying to parse the script.
    Nom(ErrorKind),
    /// Error caused by reading the TOML file.
    Toml(TomlError),
}
impl From<ConfigImportError> for ScriptConfigImportError {
    fn from(err: ConfigImportError) -> ScriptConfigImportError {
        match err {
            ConfigImportError::Io(e) => ScriptConfigImportError::Io(e),
            ConfigImportError::Toml(e) => ScriptConfigImportError::Toml(e),
        }
    }
}
impl From<ScriptImportError> for ScriptConfigImportError {
    fn from(err: ScriptImportError) -> ScriptConfigImportError {
        match err {
            ScriptImportError::Io(e) => ScriptConfigImportError::Io(e),
            ScriptImportError::Nom(e) => ScriptConfigImportError::Nom(e),
        }
    }
}
impl From<TomlError> for ScriptConfigImportError {
    fn from(err: TomlError) -> ScriptConfigImportError {
        ScriptConfigImportError::Toml(err)
    }
}
impl From<IoError> for ScriptConfigImportError {
    fn from(err: IoError) -> ScriptConfigImportError {
        ScriptConfigImportError::Io(err)
    }
}

/// Error building the Game.
#[derive(Debug)]
pub enum GameBuildError {
    /// Error importing the GUI file.
    Gui(ConfigImportError),
    /// Error importing the Background file.
    Backgrounds(ConfigImportError),
    /// Error importing the Character file.
    Characters(ConfigImportError),
    /// Error importing the Scripts Config file.
    Story(ScriptConfigImportError),
    /// Error importing the Input file.
    Input(ConfigImportError),
    /// Error importing the Music file.
    Music(ConfigImportError),
}
impl From<ScriptConfigImportError> for GameBuildError {
    fn from(err: ScriptConfigImportError) -> GameBuildError {
        GameBuildError::Story(err)
    }
}

/// An error caused by trying to play music.
#[derive(Debug)]
pub enum MusicError {
    /// No default output device for playing music was found.
    NoDefaultOutputDeviceFound,
    /// Error in decoding the music file.
    Decoder(DecoderError),
    /// Error finding and opening the file.
    Io(IoError)
}
impl From<DecoderError> for MusicError {
    fn from(err: DecoderError) -> MusicError {
        MusicError::Decoder(err)
    }
}
impl From<IoError> for MusicError {
    fn from(err: IoError) -> MusicError {
        MusicError::Io(err)
    }
}
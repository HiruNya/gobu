use std::io::Error as IoError;
use toml::de::Error as TomlError;
use nom::Err as NomErr;
use nom::ErrorKind;

#[derive(Debug)]
pub enum ConfigImportError {
    Io(IoError),
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

#[derive(Debug)]
pub enum ScriptImportError {
    Io(IoError),
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

#[derive(Debug)]
pub enum ScriptConfigImportError {
    Io(IoError),
    Nom(ErrorKind),
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

#[derive(Debug)]
pub enum GameBuildError {
    Gui(ConfigImportError),
    Backgrounds(ConfigImportError),
    Characters(ConfigImportError),
    Story(ScriptConfigImportError),
    Input(ConfigImportError),
}
impl From<ScriptConfigImportError> for GameBuildError {
    fn from(err: ScriptConfigImportError) -> GameBuildError {
        GameBuildError::Story(err)
    }
}
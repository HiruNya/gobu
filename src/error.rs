use std::io::Error as IoError;
use toml::de::Error as TomlError;
use nom::Err as NomErr;
use nom::ErrorKind;

#[derive(Debug)]
pub enum ImportError {
    Io(IoError),
    Toml(TomlError),
}
impl From<IoError> for ImportError {
    fn from(err: IoError) -> Self {
        ImportError::Io(err)
    }
}
impl From<TomlError> for ImportError {
    fn from(err: TomlError) -> Self {
        ImportError::Toml(err)
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
use std::{
    io::Read,
    fs::File,
    path::Path,
    collections::HashMap,
};
use toml::from_str;
use indexmap::IndexMap;
use super::super::super::{
    error::ScriptConfigImportError,
    script::{
        parser::translate,
        ScriptStep,
    },
};

/// Load the scripts from a TOML file
pub fn load_scripts_from_file<P: AsRef<Path>>(path: P) -> Result<ScriptsFromFile, ScriptConfigImportError> {
    let mut buffer = String::new();
    File::open(path)?.read_to_string(&mut buffer)?;
    load_scripts_from_str(&buffer)
}

/// Load the scripts from a TOML str
pub fn load_scripts_from_str(text: &str) -> Result<ScriptsFromFile, ScriptConfigImportError> {
    let config: HashMap<String, String> = from_str(text)?;
    let mut map: IndexMap<String, IndexMap<String, Vec<ScriptStep>>> = IndexMap::new();
    let mut default = None;
    for (k, v) in config.iter() {
        match k.to_lowercase().as_str() {
            "default" => {
                default = Some(v.to_string());
            },
            key => {
                let mut buffer = String::new();
                File::open(v)?.read_to_string(&mut buffer)?;
                let script = translate(&buffer)?;
                map.insert(key.to_string(), script);
            },
        }
    }
    Ok(ScriptsFromFile{
        default,
        map,
    })
}

/// The scripts that are loaded from the file before they are turned into a [`Character`] struct.
pub struct ScriptsFromFile {
    /// The default script
    pub default: Option<String>,
    /// A map of all the scripts
    pub map: IndexMap<String, IndexMap<String, Vec<ScriptStep>>>,
}
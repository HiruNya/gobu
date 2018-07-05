use std::{
    fs::File,
    io::Read,
    sync::Arc,
    path::Path,
    collections::HashMap,
};
use super::super::super::error::ConfigImportError;
use toml::from_str;

/// Load music from a TOML file
pub fn load_music_from_file<P: AsRef<Path>>(path: P)
    -> Result<HashMap<String, Arc<[u8]>>, ConfigImportError> {
    let mut buf = String::new();
    File::open(path)?.read_to_string(&mut buf)?;
    load_music_from_str(&buf)
}

/// Load music from a TOML str
pub fn load_music_from_str(text: &str)
    -> Result<HashMap<String, Arc<[u8]>>, ConfigImportError> {
    let map: HashMap<String, String> = from_str(text)?;
    let mut new_map = HashMap::new();
    for (k, v) in map.iter() {
        if let Ok(mut e) = File::open(v) {
            let mut buf = Vec::new();
            // These results really should be handled
            let _ = e.read_to_end(&mut buf);
            new_map.insert(k.to_string(), Arc::from(buf));
        }
    }
    Ok(new_map)
}
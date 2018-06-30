use std::{
    fs::File,
    path::Path,
    sync::Arc,
    io::Read,
    collections::HashMap,
};
use toml::from_str;
use piston_window::{
    GfxFactory,
    G2dTexture
};
use super::super::super::error::ConfigImportError;
use super::load::load_background_images;

/// Load the backgrounds from a TOML file
pub fn load_backgrounds_from_file<P: AsRef<Path>>(path: P, factory: &mut GfxFactory)
    -> Result<HashMap<String, Arc<G2dTexture>>, ConfigImportError> {
    let mut buffer = String::new();
    File::open(path)?.read_to_string(&mut buffer)?;
    load_backgrounds_from_str(&buffer, factory)
}

/// Load the backgrounds from a TOML str
pub fn load_backgrounds_from_str(text: &str, factory: &mut GfxFactory)
    -> Result<HashMap<String, Arc<G2dTexture>>, ConfigImportError> {
    let map: HashMap<String, String> = from_str(text)?;
    Ok(load_background_images(map, factory))
}
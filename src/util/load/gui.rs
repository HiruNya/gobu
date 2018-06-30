use std::{
    path::Path,
    fs::File,
    collections::HashMap,
    io::Read,
};
use super::super::super::{
    error::ConfigImportError,
    gui::textbox::{
        TextBox,
        TextBoxBuilder,
    },
    game::Game,
};
use toml::from_str;

/// Load the GUI from a TOML file
pub fn load_gui_from_file<P: AsRef<Path>>(path: P, game: &Game)
    -> Result<HashMap<String, TextBox>, ConfigImportError> {
    let mut buffer = String::new();
    File::open(path)?.read_to_string(&mut buffer)?;
    load_gui_from_str(&buffer, game)
}

/// Load the GUI from a TOML str
pub fn load_gui_from_str(text: &str, game: &Game)
    -> Result<HashMap<String, TextBox>, ConfigImportError> {
    let map: HashMap<String, TextBoxBuilder> = from_str(text)?;
    Ok(load_gui_from_hashmap(map, game))
}

fn load_gui_from_hashmap(map: HashMap<String, TextBoxBuilder>, game: &Game)
    -> HashMap<String, TextBox> {
    map.iter()
        .map(|(k, builder)| (k.to_lowercase(), builder.clone().build(game)))
        .collect()
}
use std::{
    path::Path,
    fs::File,
    collections::HashMap,
    io::Read,
};
use super::super::super::{
    error::ImportError,
    gui::textbox::{
        TextBox,
        TextBoxBuilder,
    },
    game::Game,
};
use toml::from_str;

pub fn load_gui_from_file<P: AsRef<Path>>(path: P, game: &Game)
    -> Result<HashMap<String, TextBox>, ImportError> {
    let mut buffer = String::new();
    File::open(path)?.read_to_string(&mut buffer)?;
    load_gui_from_str(&buffer, game)
}

fn load_gui_from_str(text: &str, game: &Game)
    -> Result<HashMap<String, TextBox>, ImportError> {
    let map: HashMap<String, TextBoxBuilder> = from_str(text)?;
    Ok(load_gui_from_hashmap(map, game))
}

fn load_gui_from_hashmap(map: HashMap<String, TextBoxBuilder>, game: &Game)
    -> HashMap<String, TextBox> {
    map.iter()
        .map(|(k, builder)| (k.to_lowercase(), builder.clone().build(game)))
        .collect()
}
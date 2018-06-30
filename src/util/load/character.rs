use std::{
    fs::File,
    io::Read,
    path::Path,
    collections::HashMap,
};
use toml::from_str;
use super::super::super::{
    Pos,
    character::Character,
    error::ConfigImportError,
};
use piston_window::GfxFactory;
use super::load::load_character_images;

type RawCharactersHashmap = HashMap<String, HashMap<String, ValueType>>;
pub type ParsedCharactersHashmap = HashMap<String, CharacterFromFile>;
pub type CharacterHashmap = HashMap<String, Character>;

/// Load the characters from a TOML file
pub fn load_characters_from_file<P: AsRef<Path>>(path: P, window: &mut GfxFactory)
    -> Result<CharacterHashmap, ConfigImportError> {
    let mut buffer = String::new();
    File::open(path)?.read_to_string(&mut buffer)?;
    load_characters_from_str(&buffer, window)
}

/// Load the characters from a TOML str
pub fn load_characters_from_str(text: &str, factory: &mut GfxFactory)
    -> Result<CharacterHashmap, ConfigImportError> {
    let map: HashMap<String,
            HashMap<String, ValueType>> = from_str(text)?;
    Ok(raw_hashmap_to_characters(map, factory))
}

fn raw_hashmap_to_characters(map: RawCharactersHashmap, window: &mut GfxFactory) -> CharacterHashmap {
    let mut new_map = HashMap::new();
    for (k, v) in map.iter() {
        let mut default = None;
        let mut size = None;
        let mut offset = None;
        let mut character_map = HashMap::new();
        for (k2, v2) in v.iter() {
            match k2.to_lowercase().as_str() {
                "default" => {
                    if let ValueType::String(name) = v2 {
                        default = Some(name.clone());
                    }
                },
                "size" => {
                    if let ValueType::NumberMap(map) = v2 {
                        let mut w = None;
                        let mut h = None;
                        for (k3, v3) in map.iter() {
                            match k3.as_str() {
                                "width" | "w" => {
                                    w = Some(v3);
                                },
                                "height" | "h" => {
                                    h = Some(v3);
                                },
                                _ => {},
                            }
                        }
                        size = Some([
                            *w.unwrap_or(&0.),
                            *h.unwrap_or(&0.),
                        ]);
                    }
                },
                "offset" => {
                    if let ValueType::NumberMap(map) = v2 {
                        let x = *map.get("x").unwrap_or(&0.);
                        let y = *map.get("y").unwrap_or(&0.);
                        offset = Some(Pos { x, y });
                    }
                }
                k2 => {
                    if let ValueType::String(name) = v2 {
                        character_map.insert(k2.to_string(), name.to_string());
                        if default == None {
                            default = Some(k2.to_string());
                        }
                    }
                }
            }
        }
        let character = CharacterFromFile {
            default: default.unwrap_or("".to_string()),
            state_map: character_map,
            size: size.unwrap_or([0., 0.]),
            offset: offset.unwrap_or(Pos {x: 0., y: 0.}),
        };
        new_map.insert(k.clone(), character);
    }
    load_character_images(new_map, window)
}

/// A struct that represents
#[derive(Debug)]
pub struct CharacterFromFile {
    /// The default state of the character
    pub default: String,
    /// The different states of the character
    pub state_map: HashMap<String, String>,
    /// The size of the character
    pub size: [f64; 2], // [Width, Height]
    /// The offset of the character
    pub offset: Pos, // Offset is in percentage e.g. 0.5 = 50% therefore the origin is the centre.
}

#[derive(Deserialize, Clone)]
#[serde(untagged)]
enum ValueType {
    String(String),
    NumberMap(HashMap<String, f64>),
}
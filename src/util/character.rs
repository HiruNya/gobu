use std::{
    fs::File,
    io::Read,
    path::Path,
    collections::HashMap,
};
use toml::from_str;
use super::super::{
    Pos,
    character::Character,
};
use piston_window::PistonWindow;
use super::load::load_character_images;

type RawCharactersHashmap = HashMap<String, HashMap<String, ValueType>>;
pub type ParsedCharactersHashmap = HashMap<String, CharacterFromFile>;
pub type CharacterHashmap = HashMap<String, Character>;

pub fn load_characters_from_file<P: AsRef<Path>>(path: P, window: &mut PistonWindow) -> Result<CharacterHashmap, String> {
    let mut buffer = String::new();
    File::open(path).unwrap().read_to_string(&mut buffer).unwrap();
    Ok(load_characters_from_str(&buffer, window))
}

fn load_characters_from_str(text: &str, window: &mut PistonWindow) -> CharacterHashmap {
    let map1: HashMap<String,
        HashMap<String,
            HashMap<String, ValueType>>> = from_str(text)
        .unwrap();
    let map2: RawCharactersHashmap = map1.get("characters")
        .unwrap()
        .clone();
    raw_hashmap_to_characters(map2, window)
}

fn raw_hashmap_to_characters(map: RawCharactersHashmap, window: &mut PistonWindow) -> CharacterHashmap {
    let mut new_map = HashMap::new();
    for (k, v) in map.iter() {
        let mut default = None;
        let mut size = None;
        let mut offset = None;
        let mut character_map = HashMap::new();
        for (k2, v2) in v.iter() {
            match k2.as_str() {
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

#[derive(Debug)]
pub struct CharacterFromFile {
    pub default: String,
    pub state_map: HashMap<String, String>,
    pub size: [f64; 2], // [Width, Height]
    pub offset: Pos, // Offset is in percentage e.g. 0.5 = 50% therefore the origin is the centre.
}

#[derive(Deserialize, Clone)]
#[serde(untagged)]
enum ValueType {
    String(String),
    NumberMap(HashMap<String, f64>),
}

#[test]
fn test_deserialise() {
    let text = r#"
        [characters.cat_girl]
        default = "happy"
        happy = "./path/to/happy"
        sad = "./path/to/sad"
        offset = {x  = 0.5, y = 0.5}

        [characters.dog_girl]
        cute = "./path/to/cute"
        normal = "./path/to/normal"
        size = {w = 200.0, height = 200.0}
    "#;
    println!("{:?}",
             load_characters_from_str(text));
}
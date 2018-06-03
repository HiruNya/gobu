use super::character::{
    ParsedCharactersHashmap,
    CharacterHashmap
};
use super::super::character::Character;
use std::{
    sync::Arc,
    collections::HashMap,
};
use piston_window::{
    Texture,
    TextureSettings,
    Flip,
    PistonWindow
};

pub fn load_character_images(map: ParsedCharactersHashmap, window: &mut PistonWindow) -> CharacterHashmap {
    let mut character_map = HashMap::new();
    for (k, v) in map.iter() {
        let mut texture_map = HashMap::new();
        for (k2, v2) in v.state_map.iter() {
            let texture = Arc::new(Texture::from_path(
                &mut window.factory,
                &v2,
                Flip::None,
                &TextureSettings::new())
                                       .unwrap());
            texture_map.insert(k2.to_string(), texture);
        }
        let chara = Character {
            default: v.default.to_string(),
            state_map: texture_map,
            size: v.size,
            offset: v.offset,
        };
        character_map.insert(k.to_string(), chara);
    }
    character_map
}
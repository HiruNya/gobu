use super::character::{
    ParsedCharactersHashmap,
    CharacterHashmap
};
use super::super::super::character::Character;
use std::{
    sync::Arc,
    collections::HashMap,
};
use piston_window::{
    Texture,
    TextureSettings,
    Flip,
    GfxFactory,
    G2dTexture
};

pub fn load_character_images(map: ParsedCharactersHashmap, factory: &mut GfxFactory) -> CharacterHashmap {
    let mut character_map = HashMap::new();
    for (k, v) in map.iter() {
        let mut texture_map = HashMap::new();
        for (k2, v2) in v.state_map.iter() {
            if let Ok(texture) = Texture::from_path(
                factory,
                &v2,
                Flip::None,
                &TextureSettings::new()) {
                texture_map.insert(k2.to_string(), Arc::new(texture));
            }
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

pub fn load_background_images(map: HashMap<String, String>, factory: &mut GfxFactory)
    -> HashMap<String, Arc<G2dTexture>> {
    let mut new_map = HashMap::new();
    for (k, path) in map.iter() {
        if let Ok(e) = Texture::from_path(
            factory,
            path,
            Flip::None,
            &TextureSettings::new()) {
            new_map.insert(k.to_string(), Arc::new(e));
        }
    }
    new_map
}
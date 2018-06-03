use piston_window::{
    Image,
    G2d,
    G2dTexture,
    Context,
    DrawState,
};
use std::{
    sync::Arc,
    ops::Deref,
    collections::HashMap,
};

use super::{Rect, Pos};

pub struct CharacterEntity {
    pub image: Image,
    pub texture: Arc<G2dTexture>,
    pub rect: Rect,
    pub visible: bool,
    pub name: String,
//    pub pos: Pos,
    pub offset: Pos,
}
impl CharacterEntity {
    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }
    pub fn set_texture(&mut self, texture: Arc<G2dTexture>) {
        self.texture = texture;
    }
    pub fn set_pos(&mut self, pos: Pos) {
        self.rect.x = pos.x;
        self.rect.y = pos.y;
        self.image = self.image.rect([
            pos.x - (self.rect.w * self.offset.x),
            pos.y - (self.rect.h * self.offset.y),
            self.rect.w,
            self.rect.h
        ]);
    }
    pub fn draw(&self, c: Context, g: &mut G2d) {
        if self.visible {
            self.image.draw(
                self.texture.deref(),
                &DrawState::default(),
                c.transform,
                g
            );
        }
    }
}

#[derive(Clone)]
pub struct Character {
    pub default: String,
    pub state_map: HashMap<String, Arc<G2dTexture>>,
    pub size: [f64; 2], // [Width, Height]
    pub offset: Pos, // Offset is in percentage e.g. 0.5 = 50% therefore the origin is the centre.
}
impl Character {
    pub fn new(default: String, texture: Arc<G2dTexture>, size: [f64; 2]) -> Character {
        let mut state_map = HashMap::new();
        state_map.insert(default.clone(), texture);
        Character {
            default,
            state_map,
            size,
            offset: Pos {x: 0., y: 0.}
        }
    }
    pub fn spawn(&self, name: String) -> Option<CharacterEntity> {
        let texture = self.state_map.get(&self.default)?.clone();
        Some(CharacterEntity {
            image: Image::new(),
            texture,
            rect: Rect {x: 0., y: 0., w: self.size[0], h: self.size[1]},
            visible: true,
            name,
//            pos: Pos {x: 0., y: 0.},
            offset: self.offset,
        })
    }
    pub fn add_state(&mut self, name: String, texture: Arc<G2dTexture>) {
        self.state_map.insert(name, texture);
    }
    pub fn set_offset(&mut self, x: f64, y:f64) {
        self.offset.x = x;
        self.offset.y = y;
    }
}
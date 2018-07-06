//! Use both [`Character`]s and [`CharacterEntity`]s to display people or objects.
//!
//! * A [`Character'] is a sort of template which holds all the different states
//! and their corresponding textures. The characters are created first and then during
//! the game they are spawned onto the stage as a [`CharacterEntity`].
//! * A [`CharacterEntity`] is the struct that is shown on screen. These hold positions
//! but only have on state at a time.
//! The state is changed by going back to the original
//! Character and finding the desire state.
//! Entities can have the same name as the Character but it is not recommended
//! if you intend on having multiple entities of the same character in the stage
//! at the same time.

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

use super::{
    Rect,
    Pos,
    animation::CharacterTransition,
};

/// The entity of a character that is spawned into the stage.
pub struct CharacterEntity {
    /// The Image primitive provided by the Piston libraries.
    pub image: Image,
    /// The current texture being used to draw the character.
    pub texture: Arc<G2dTexture>,
    /// The position and size of the character image.
    pub rect: Rect,
    /// Whether the character is visible on screen or not.
    pub visible: bool,
    /// The name of the entity.
    pub name: String,
    /// The offset of the images so that a specific point on the image can be used as the centre.
    /// Set to (0, 0) by default.
    pub offset: Pos,
    /// The [`CharacterTransition`] that happens on the entity.
    pub anim: Option<Box<dyn CharacterTransition>>,
}
impl CharacterEntity {
    /// Sets the visibility of the character on screen.
    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }
    /// Sets the texture of the character.
    pub fn set_texture(&mut self, texture: Arc<G2dTexture>) {
        self.texture = texture;
    }
    /// Sets the position of the character. This accounts for the offset.
    pub fn set_pos(&mut self, pos: Pos) {
        self.rect.pos = pos;
        let pos = pos - (self.rect.size * self.offset);
        let rect = Rect {
            pos,
            size: self.rect.size,
        };
        self.image = self.image.rect(rect.to_slice());
    }
    /// Draws the character onto the screen.
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
    /// Applies a [`CharacterTransition`] to the entity.
    pub fn apply_trans(&mut self, trans: Box<dyn CharacterTransition>) {
        self.finish();
        self.anim = Some(trans);
    }
    /// Updates the animation struct if possible.
    pub fn update(&mut self, delta_time: f64) {
        use super::animation::TransResult;
        let result = {
            if let Some(ref mut e) = self.anim {
                e.update(&mut self.image, delta_time)
            } else {
                TransResult::Continue
            }
        };
        if result == TransResult::Finished {
            self.anim = None;
        }
    }
    /// Finishes the animation struct if possible.
    pub fn finish(&mut self) {
        if let Some(ref mut e) = self.anim {
            e.finish(&mut self.image);
        }
        self.anim = None;
    }
}

/// A character that is used to spawn an entity onto the stage.
#[derive(Clone)]
pub struct Character {
    /// The default state that the entity will have upon being spawned.
    pub default: String,
    /// A collection of all the different states and their corresponding textures.
    pub state_map: HashMap<String, Arc<G2dTexture>>,
    /// The size of the images
    /// [Width, Height]
    pub size: [f64; 2], // [Width, Height]
    /// The offset (in percentage) of the centre of the images.
    /// This is helpful when you want to move the entity.
    /// x = 0., y = 0. : Top-Left corner
    /// x = 0.5, y = 0.5 : Centre of the image
    pub offset: Pos, // Offset is in percentage e.g. 0.5 = 50% therefore the origin is the centre.
}
impl Character {
    /// Create a new character where ``texture`` is the default texture that is to be used and
    /// ``default`` is the name of the default texture.
    pub fn new(default: String, texture: Arc<G2dTexture>, size: [f64; 2]) -> Character {
        let mut state_map = HashMap::new();
        state_map.insert(default.clone(), texture);
        Character {
            default,
            state_map,
            size,
            offset: Pos::new(0., 0.)
        }
    }
    /// Spawns an entity of the character onto stage with the given ``name``.
    pub fn spawn(&self, name: String) -> Option<CharacterEntity> {
        let texture = self.state_map.get(&self.default)?.clone();
        Some(CharacterEntity {
            image: Image::new()
                .rect([0., 0., self.size[0], self.size[1]]),
            texture,
            rect: Rect {
                pos: Pos::new(0., 0.),
                size: self.size.into(),
            },
            visible: true,
            name,
            offset: self.offset,
            anim: None,
        })
    }
    /// Adds a state to the character. This can be something like "sad" or "happy"
    pub fn add_state(&mut self, name: String, texture: Arc<G2dTexture>) {
        self.state_map.insert(name, texture);
    }
    /// Sets the offset of the character. This is (0, 0) by default.
    pub fn set_offset(&mut self, x: f64, y:f64) {
        self.offset.x = x;
        self.offset.y = y;
    }
}
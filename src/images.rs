//! Deals with displaying the background image

use piston_window::{
    Image,
    G2dTexture,
    Context,
    G2d,
    DrawState,
};
use std::{
    sync::Arc,
    ops::Deref,
};

use super::Rect;

/// The background image that is shown behind all the characters in the game.
pub struct BackgroundImage {
    image: Image,
    texture: Option<Arc<G2dTexture>>,
}

impl BackgroundImage {
    /// Create a new background image.
    ///
    /// This should be done by the Game struct so there isn't a need for the user of this
    /// crate to do it.
    pub fn new(canvas: Rect) -> BackgroundImage {
        BackgroundImage {
            image: Image::new().rect(canvas.to_slice()),
            texture: None,
        }
    }
    /// Sets the texture of the background.
    ///
    /// It is recommended to use the backgrounds HashMap and the function provided by
    /// the [`Game`] struct instead.
    pub fn set_texture(&mut self, texture: Arc<G2dTexture>) {
        self.texture = Some(texture);
    }
    /// Clears the background.
    pub fn clear_texture(&mut self) {
        self.texture = None;
    }
    /// Draws the background onto the screen.
    pub fn draw(&mut self, c: Context, g: &mut G2d) {
        if let Some(ref back) = self.texture {
            self.image.draw(
                back.deref(),
                &DrawState::default(),
                c.transform,
                g
            );
        }
    }
    /// Resizes the background image to the size of the rectangle.
    pub fn resize(&mut self, rect: Rect) {
        self.image = self.image.rect(rect.to_slice());
    }
}
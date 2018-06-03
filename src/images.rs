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

pub struct BackgroundImage {
    image: Image,
    texture: Option<Arc<G2dTexture>>,
}

impl BackgroundImage {
    pub fn new(canvas: Rect) -> BackgroundImage {
        BackgroundImage {
            image: Image::new().rect([canvas.x, canvas.y, canvas.w, canvas.h]),
            texture: None,
        }
    }
    pub fn set_texture(&mut self, texture: Arc<G2dTexture>) {
        self.texture = Some(texture);
    }
    pub fn clear_texture(&mut self) {
        self.texture = None;
    }
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
    pub fn resize(&mut self, rect: Rect) {
        self.image = self.image.rect([rect.x, rect.y, rect.w, rect.h]);
    }
}
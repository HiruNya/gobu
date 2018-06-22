extern crate toml;
extern crate serde;
extern crate indexmap;
extern crate piston_window;
#[macro_use] extern crate nom;
#[macro_use] extern crate serde_derive;
#[cfg(feature = "gfx_glyph_text")] extern crate gfx_glyph;
#[cfg(feature = "gfx_glyph_text")] extern crate gfx_device_gl;

pub mod game;
pub mod gui;
pub mod images;
pub mod character;
pub mod script;
pub mod util;
pub mod input;
pub mod error;

pub use game::*;
pub use util::GameBuilder;

#[cfg(test)]
mod tests {}

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct Rect {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
}

#[derive(Copy, Clone, Debug)]
pub struct Pos {
    pub x: f64,
    pub y: f64,
}
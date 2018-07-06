#![warn(missing_docs)]
//! A crate for creating Visual Novels.
//!
//! See example_vn for a good example of how to use this crate.
//!
//! [`GameBuilder`] is recommended to make a [`Game`] with as you can use TOML files
//! to make the game without having to use any of the functions.
//!
//! Here is a quick example of how to quickly make a game.
//! ```rust, no_run
//! extern crate gobu;
//! use gobu::{Game, GameBuilder};
//! let mut game: Game = GameBuilder::new([WIDTH, HEIGHT]) // The width and height of the game/screen.
//!                 .gui("./path/to/gui.toml") // Config files are in the TOML format.
//!                 .characters("./path/to/characters.toml") // See the example_vn repo for more.
//!                 .backgrounds("./path/to/backgrounds.toml")
//!                 .build(&mut factory); // Requires a GfxFactory to build the game as it uses texture.
//! ```

extern crate toml;
extern crate serde;
extern crate rodio;
extern crate indexmap;
extern crate piston_window;
#[macro_use] extern crate nom;
#[macro_use] extern crate coord;
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
pub mod music;
pub mod animation;

pub use game::Game;
pub use util::GameBuilder;

#[cfg(test)]
mod tests {}

use coord::vec2::Vec2;

/// Represents a rectangle
#[derive(Copy, Clone, Debug, Deserialize)]
pub struct Rect {
    /// The position of the rectangle.
    pos: Pos,
    /// The size of the rectangle.
    size: Vec2<f64>,
}
impl Rect {
    /// Turns the rectangle into a slice.
    /// This is useful for draw functions that require a slice of values as a rectangle.
    /// [x, y, w, h]
    fn to_slice(self) -> [f64; 4] {
        [
            self.pos.x,
            self.pos.y,
            self.size.x,
            self.size.y
        ]
    }
}

/// Represents a position as a vector of [`f64`]s
pub type Pos = Vec2<f64>;
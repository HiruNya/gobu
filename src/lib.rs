#![warn(missing_docs)]
//! A crate for creating ~~Bad~~ Visual Novels.
//!
//! See example_vn for a good example of how to use this crate.
//!
//! [`GameBuilder`] is recommended to make a [`Game`] with as you can use TOML files
//! to make the game without having to use any of the functions.
//!
//! Here is a quick example of how to quickly make a game.
//! ```rust, no_run
//! use svnf::{Game, GameBuilder};
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

pub use game::Game;
pub use util::GameBuilder;

#[cfg(test)]
mod tests {}

/// Represents a rectangle
#[derive(Copy, Clone, Debug, Deserialize)]
pub struct Rect {
    /// The x - co-ordinate
    pub x: f64,
    /// The y - co-ordinate
    pub y: f64,
    /// The width of the rectangle
    pub w: f64,
    /// The height of the rectangle
    pub h: f64,
}


/// Represents a position
#[derive(Copy, Clone, Debug)]
pub struct Pos {
    /// The x - co-ordinate
    pub x: f64,
    /// The y - co-ordinate
    pub y: f64,
}
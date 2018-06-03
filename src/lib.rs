extern crate piston_window;
#[macro_use] extern crate nom;
extern crate toml;
extern crate serde;
#[macro_use] extern crate serde_derive;

pub mod game;
pub mod gui;
pub mod images;
pub mod character;
pub mod script;
pub mod util;

pub use game::*;
pub use character::Character;

#[cfg(test)]
mod tests {}

#[derive(Copy, Clone, Debug)]
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
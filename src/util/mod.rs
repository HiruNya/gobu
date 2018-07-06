//! A variety of utility functions that make life easier.
//!
//! Contains specific functions that deal with the ``gfx_glyph`` crate but will
//! only be compiled if the ``gfx_glyph_text`` feature is enabled.

pub mod load;
mod game_builder;
#[cfg(feature = "gfx_glyph_text")]
pub mod gfx_glyph;

pub use self::{
    load::{
        load_characters_from_file,
        load_input_from_file,
        load_gui_from_file,
        load_backgrounds_from_file,
        load_scripts_from_file,
        load_music_from_file,
        load_transitions_from_file,
    },
    game_builder::{
        GameBuilder,
        ExtFile,
    },
};

#[cfg(feature = "gfx_glyph_text")]
pub use self::gfx_glyph::GfxGlyph;
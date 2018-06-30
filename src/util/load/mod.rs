//! Helpful functions that load assets from a TOML file.

mod character;
mod load;
mod input;
mod gui;
mod background;
mod script;

pub use self::{
    character::{
        load_characters_from_file,
        load_characters_from_str,
    },
    input::{
        load_input_from_file,
        load_input_from_str,
    },
    gui::{
        load_gui_from_file,
        load_gui_from_str,
    },
    background::{
        load_backgrounds_from_file,
        load_backgrounds_from_str,
    },
    script::{
        load_scripts_from_file,
        load_scripts_from_str,
        ScriptsFromFile,
    }
};
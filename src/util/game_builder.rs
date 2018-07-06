use super::super::{
    game::Game,
    error::GameBuildError,
};
use piston_window::GfxFactory;

type MaybeFile = Option<ExtFile>;

/// The struct that is used to build a [`Game`] struct.
pub struct GameBuilder {
    /// The size of the window.
    /// [w, h]
    size: [f64; 2],
    /// The path or string to a GUI config file.
    gui: MaybeFile,
    /// The path or string to a Background config file.
    backgrounds: MaybeFile,
    /// The path or string to a Character config file.
    characters: MaybeFile,
    /// The path or string to a Script config file.
    scripts: MaybeFile,
    /// The path or string to a Grid config file.
    grid: Option<[u32; 2]>,
    /// The path or string to an Input config file.
    input: MaybeFile,
    /// The path or string to a Music config file.
    music: MaybeFile,
    /// The path or string to a Transition config file.
    transitions: MaybeFile,
}
impl GameBuilder {
    /// Create a new [`GameBuilder`] struct by providing the size of the window.
    pub fn new(size: [f64; 2]) -> Self {
        GameBuilder {
            size,
            gui: None,
            backgrounds: None,
            characters: None,
            scripts: None,
            grid: None,
            input: None,
            music: None,
            transitions: None,
        }
    }
    /// Builds the [`Game`] or returns an error.
    pub fn build(self, factory: &mut GfxFactory) -> Result<Game, GameBuildError> {
        let mut g = Game::new(self.size);
        if let Some(grid) = self.grid {
            g.apply_grid(grid[0], grid[1]);
        }
        if let Some(gui) = self.gui {
            let result = match gui {
                ExtFile::Path(p) => {
                    g.load_gui_from_file(&p)
                },
                ExtFile::Str(s) => {
                    g.load_gui_from_str(&s)
                },
            };
            if let Err(e) = result {
                return Err( GameBuildError::Gui(e) )
            }
        }
        if let Some(chara) = self.characters {
            let result = match chara {
                ExtFile::Path(p) => {
                    g.load_characters_from_file(&p, factory)
                },
                ExtFile::Str(s) => {
                    g.load_characters_from_str(&s, factory)
                },
            };
            if let Err(e) = result {
                return Err( GameBuildError::Characters(e) )
            }
        }
        if let Some(bgs) = self.backgrounds {
            let result = match bgs {
                ExtFile::Path(p) => {
                    g.load_backgrounds_from_file(&p, factory)
                },
                ExtFile::Str(s) => {
                    g.load_backgrounds_from_str(&s, factory)
                },
            };
            if let Err(e) = result {
                return Err( GameBuildError::Backgrounds(e) )
            }
        }
        if let Some(input) = self.input {
            let result = match input {
                ExtFile::Path(p) => {
                    g.load_input_from_file(&p)
                },
                ExtFile::Str(s) => {
                    g.load_input_from_str(&s)
                },
            };
            if let Err(e) = result {
                return Err( GameBuildError::Input(e) )
            }
        }
        if let Some(st) = self.scripts {
            match st {
                ExtFile::Path(p) => {
                    g.load_scripts_from_file(&p)?
                },
                ExtFile::Str(s) => {
                    g.load_scripts_from_str(&s)?
                },
            }
        }
        if let Some(e) = self.music {
            if let Ok(m) = g.enable_music() {
                let result = match e {
                    ExtFile::Path(p) => {
                        m.load_from_config_file(&p)
                    },
                    ExtFile::Str(s) => {
                        m.load_from_config_str(&s)
                    },
                };
                if let Err(er) = result {
                    return Err( GameBuildError::Music(er) )
                }
            }
        }
        if let Some(t) = self.transitions {
            let result = {
                match t {
                    ExtFile::Path(p) => {
                        g.load_transitions_from_file(&p)
                    },
                    ExtFile::Str(s) => {
                        g.load_transitions_from_str(&s)
                    },
                }
            };
            if let Err(e) = result {
                return Err( GameBuildError::Transition(e) )
            }
        }
        Ok(g)
    }
    /// Create a GUI from either a path to an external file or a [`&str`].
    pub fn gui<F: Into<ExtFile>>(mut self, file: F) -> Self {
        self.gui = Some(file.into());
        self
    }
    /// Create Characters from either a path to an external file or a [`&str`].
    pub fn characters<F: Into<ExtFile>>(mut self, file: F) -> Self {
        self.characters = Some(file.into());
        self
    }
    /// Create Backgrounds from either a path to an external file or a [`&str`].
    pub fn backgrounds<F: Into<ExtFile>>(mut self, file: F) -> Self {
        self.backgrounds = Some(file.into());
        self
    }
    /// Create Inputs from either a path to an external file or a [`&str`].
    pub fn input<F: Into<ExtFile>>(mut self, file: F) -> Self {
        self.input = Some(file.into());
        self
    }
    /// Create a Grid from either a path to an external file or a [`&str`].
    pub fn grid(mut self, w: u32, h: u32) -> Self {
        self.grid = Some([w, h]);
        self
    }
    /// Load scripts from either a path to an external file or a [`&str`].
    pub fn scripts<F: Into<ExtFile>>(mut self, file: F) -> Self {
        self.scripts = Some(file.into());
        self
    }
    /// Load music from either a path to an external file or a [`&str`].
    pub fn music<F: Into<ExtFile>>(mut self, file: F) -> Self {
        self.music = Some(file.into());
        self
    }
    /// Load transitions from either a path to an external file or a [`&str`].
    pub fn transitions<F: Into<ExtFile>>(mut self, file: F) -> Self {
        self.transitions = Some(file.into());
        self
    }
}

/// Represents an external file OR a [`String`].
///
/// Note: when entering in a [`String`] or [`&str`] to functions that take in an [`Into<ExtFile>`]
/// it assumes that a Path was entered. If this was wrong and in fact a ``str`` was entered,
/// you have to write out the whole enum variant. E.g. ``ExtFile::Str("TOML File goes here")``
pub enum ExtFile {
    /// Represents a path to an external file that can be read.
    Path(String),
    /// A String that acts as a TOML file.
    Str(String),
}
impl From<String> for ExtFile {
    fn from(text: String) -> ExtFile {
        ExtFile::Path(text)
    }
}
impl From<&'static str> for ExtFile {
    fn from(text: &'static str) -> ExtFile {
        ExtFile::Path(text.to_string())
    }
}
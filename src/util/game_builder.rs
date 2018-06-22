use super::super::{
    game::Game,
    error::GameBuildError,
};
use piston_window::GfxFactory;

type MaybeFile = Option<ExtFile>;

pub struct GameBuilder {
    size: [f64; 2],
    gui: MaybeFile,
    backgrounds: MaybeFile,
    characters: MaybeFile,
    scripts: MaybeFile,
    grid: Option<[u32; 2]>,
    input: MaybeFile,
}
impl GameBuilder {
    pub fn new(size: [f64; 2]) -> Self {
        GameBuilder {
            size,
            gui: None,
            backgrounds: None,
            characters: None,
            scripts: None,
            grid: None,
            input: None,
        }
    }
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
        Ok(g)
    }
    pub fn gui<F: Into<ExtFile>>(mut self, file: F) -> Self {
        self.gui = Some(file.into());
        self
    }
    pub fn characters<F: Into<ExtFile>>(mut self, file: F) -> Self {
        self.characters = Some(file.into());
        self
    }
    pub fn backgrounds<F: Into<ExtFile>>(mut self, file: F) -> Self {
        self.backgrounds = Some(file.into());
        self
    }
    pub fn input<F: Into<ExtFile>>(mut self, file: F) -> Self {
        self.input = Some(file.into());
        self
    }
    pub fn grid(mut self, w: u32, h: u32) -> Self {
        self.grid = Some([w, h]);
        self
    }
    pub fn scripts<F: Into<ExtFile>>(mut self, file: F) -> Self {
        self.scripts = Some(file.into());
        self
    }
}

pub enum ExtFile {
    Path(String),
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
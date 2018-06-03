use piston_window::{
    clear,
    color,
    G2d,
    context::Context,
    Glyphs,
    G2dTexture,
    Event,
    PistonWindow,
};
use std::{
    collections::HashMap,
    sync::Arc,
    path::Path,
};

use super::{
    gui::{
        Ui,
        Grid,
        TextBox,
    },
    images::BackgroundImage,
    Rect, Pos,
    character::{
        Character,
        CharacterEntity,
    },
    script::Script,
    util::load_characters_from_file,
};

pub struct Game {
    pub size: Rect,
    pub ui: Ui,
    pub background: BackgroundImage,
    pub stage: HashMap<String, CharacterEntity>,
    pub characters: HashMap<String, Character>,
    pub story: Script,
    pub grid: Grid,
}

impl Game {
    pub fn new(size: [f64; 2]) -> Game {
        let size = Rect {
            x: 0.,
            y: 0.,
            w: size[0],
            h: size[1],
        };
        Game{
            size,
            ui: Ui::new(size),
            background: BackgroundImage::new(size),
            stage: HashMap::new(),
            characters: HashMap::new(),
            story: Script::new(),
            grid: Grid::new(1, 1, size),
        }
    }
    pub fn handle_event(&mut self, event: &Event) {
        use self::Event::Input;
        match *event {
            Input(ref i) => {
                use piston_window::{
                    Input::{Resize, Button},
                    ButtonState::Press,
                };
                match i {
                    Resize(ref w, ref h) => {
                        self.resize(*w, *h);
                    },
                    Button(args) if args.state == Press  => {
                        use piston_window::Button::Mouse;
                        match args.button {
                            Mouse(button) => {
                                use piston_window::MouseButton::Left;
                                match button {
                                    Left => {
                                        self.next_step();
                                        self.change_entity_state("cat entity".to_string(), "2".to_string());
                                    },
                                    _ => {},
                                }
                            },
                            _ => {},
                        }
                    },
                    _ => {},
                }
            },
            _ => {},
        }
    }
    pub fn draw(&mut self, c: Context, g: &mut G2d, glyph_cache: &mut Glyphs) {
        clear(color::BLACK, g);
        self.background.draw(c, g);
        for character in self.stage.values() {
            character.draw(c, g);
        };
        self.ui.draw(c, g, glyph_cache);
    }
    pub fn set_background(&mut self, texture: Arc<G2dTexture>) {
        self.background.set_texture(texture);
    }
    pub fn resize(&mut self, w: u32, h: u32) {
        let rect = Rect {x: 0., y: 0., w: w as f64, h: h as f64};
        self.background.resize(rect);
//        self.ui.resize(rect);
    }
    pub fn add_character(&mut self, name: String, character: Character) {
        self.characters.insert(name, character);
    }
    pub fn add_to_stage(&mut self, name: String, character: String) -> bool {
        if let Some(chara) = self.characters.get(&character) {
            if let Some(c) =  chara.spawn(character) {
                self.stage.insert(name, c);
                true
            } else {false}
        } else {false}
    }
    pub fn change_entity_state(&mut self, name: String, state: String) -> bool {
        if let Some(entity) = self.stage.get_mut(&name) {
            if let Some(chara) = self.characters.get(&entity.name) {
                if let Some(state) = chara.state_map.get(&state) {
                    entity.texture = state.clone();
                    true
                } else {false}
            } else {false}
        } else {false}
    }
    pub fn move_character(&mut self, name: &String, pos: Pos) {
        if let Some(entity) = self.stage.get_mut(name) {
            entity.set_pos(self.grid.get_pos(pos))
        }
    }
    pub fn apply_grid(&mut self, w: u32, h: u32) {
        self.grid = Grid::new(w, h, self.size)
    }
    pub fn set_textbox(&mut self, text_box: TextBox) {
        self.ui.textbox = text_box;
    }
    pub fn set_speaker_box(&mut self, speaker_box: TextBox) {
        self.ui.speaker_box = Some(speaker_box);
    }
    pub fn load_characters_from_file<P: AsRef<Path>>(&mut self, path: P, window: &mut PistonWindow) -> Result<(), String> {
        let map = load_characters_from_file(path, window)?;
        for (k, v) in map.iter() {
            self.characters.insert(k.to_string(), v.clone());
        }
        Ok(())
    }
}
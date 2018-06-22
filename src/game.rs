use piston_window::{
    clear,
    color,
    G2d,
    context::Context,
    G2dTexture,
    Event,
    GfxFactory,
};
#[cfg(not(feature = "gfx_glyph_text"))]
use ::piston_window::Glyphs;
#[cfg(feature = "gfx_glyph_text")]
use ::{
    gfx_glyph::GlyphBrush,
    gfx_device_gl::Resources,
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
    util::{
        load_characters_from_file,
        load_input_from_file,
        load_gui_from_file,
        load_backgrounds_from_file,
        load_scripts_from_file,
        load::{
            load_input_from_str,
            load_characters_from_str,
            load_gui_from_str,
            load_backgrounds_from_str,
            load_scripts_from_str,
            script::ScriptsFromFile,
        },
    },
    input::GameInput,
    error::{
        ConfigImportError,
        ScriptConfigImportError,
    },
};

pub struct Game {
    pub size: Rect,
    pub ui: Ui,
    pub background: BackgroundImage,
    pub backgrounds: HashMap<String, Arc<G2dTexture>>,
    pub stage: HashMap<String, CharacterEntity>,
    pub characters: HashMap<String, Character>,
    pub story: Script,
    pub grid: Grid,
    pub input: GameInput,
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
            backgrounds: HashMap::new(),
            stage: HashMap::new(),
            characters: HashMap::new(),
            story: Script::new(),
            grid: Grid::new(1, 1, size),
            input: GameInput::new(),
        }
    }
    pub fn handle_event(&mut self, event: &Event) {
        use self::Event::Input;
        match *event {
            Input(ref i) => {
                use piston_window::{
                    Input::Button,
                    ButtonState::Press,
                };
                match i {
                    Button(args) if args.state == Press => {
                        use input::GameEvent;
                        let event = self.input.handle_event(&args.button);
                        if let Some(e) = event {
                            match e {
                                GameEvent::Continue => {
                                    self.next_step();
                                },
                            }
                        }
                    },
                    _ => {},
                }
            },
            _ => {},
        }
    }
    #[cfg(not(feature = "gfx_glyph_text"))]
    pub fn draw(&mut self, c: Context, g: &mut G2d, glyph_cache: &mut Glyphs) {
        clear(color::BLACK, g);
        self.background.draw(c, g);
        for character in self.stage.values() {
            character.draw(c, g);
        };
        self.ui.draw(c, g, glyph_cache);
    }
    #[cfg(feature = "gfx_glyph_text")]
    pub fn draw(&mut self, c: Context, g: &mut G2d) {
        clear(color::BLACK, g);
        self.background.draw(c, g);
        for character in self.stage.values() {
            character.draw(c, g);
        };
        self.ui.draw(c, g);
    }
    #[cfg(feature = "gfx_glyph_text")]
    pub fn draw_text(&mut self, brush: &mut GlyphBrush<Resources, GfxFactory>) {
        self.ui.draw_text(brush)
    }
    pub fn resize(&mut self, w: u32, h: u32) {
        let rect = Rect {x: 0., y: 0., w: w as f64, h: h as f64};
        self.background.resize(rect);
//        self.ui.resize(rect);
    }
    pub fn add_character(&mut self, name: String, character: Character) {
        self.characters.insert(name, character);
    }
    pub fn add_background(&mut self, name: String, background: Arc<G2dTexture>) {
        self.backgrounds.insert(name, background);
    }
    pub fn add_to_stage(&mut self, name: String, character: String) -> bool {
        if let Some(chara) = self.characters.get(&character) {
            if let Some(c) =  chara.spawn(character) {
                self.stage.insert(name, c);
                true
            } else {false}
        } else {false}
    }
    pub fn set_background(&mut self, name: &String) {
        if let Some(bg) = self.backgrounds.get(name) {
            self.background.set_texture(bg.clone());
        }
    }
    pub fn change_entity_state(&mut self, name: &String, state: &String) -> bool {
        if let Some(entity) = self.stage.get_mut(name) {
            if let Some(chara) = self.characters.get(&entity.name) {
                if let Some(state) = chara.state_map.get(state) {
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
    pub fn load_characters_from_file<P: AsRef<Path>>(&mut self, path: P, factory: &mut GfxFactory)
        -> Result<(), ConfigImportError> {
        let map = load_characters_from_file(path, factory)?;
        Ok(self.load_characters(map))
    }
    pub fn load_characters_from_str(&mut self, text: &str, factory: &mut GfxFactory) -> Result<(), ConfigImportError> {
        let map = load_characters_from_str(text, factory)?;
        Ok(self.load_characters(map))
    }
    fn load_characters(&mut self, map: HashMap<String, Character>) {
        for (k, v) in map.iter() {
            self.characters.insert(k.to_string(), v.clone());
        }
    }
    pub fn load_input_from_file<P: AsRef<Path>>(&mut self, path: P)
        -> Result<(), ConfigImportError> {
        let input = load_input_from_file(path)?;
        self.input.add_input(input);
        Ok(())
    }
    pub fn load_input_from_str(&mut self, text: &str)
        -> Result<(), ConfigImportError> {
        let input = load_input_from_str(text)?;
        self.input.add_input(input);
        Ok(())
    }
    pub fn load_gui_from_file<P: AsRef<Path>>(&mut self, path: P)
        -> Result<(), ConfigImportError> {
        let gui = load_gui_from_file(path, self)?;
        self.load_gui(gui);
        Ok(())
    }
    pub fn load_gui_from_str(&mut self, text: &str)
        -> Result<(), ConfigImportError> {
        let gui = load_gui_from_str(text, self)?;
        self.load_gui(gui);
        Ok(())
    }
    fn load_gui(&mut self, gui: HashMap<String, TextBox>) {
        if let Some(e) = gui.get("textbox") {
            self.set_textbox(e.clone());
        }
        if let Some(e)= gui.get("speakerbox") {
            self.set_speaker_box(e.clone());
        }
    }
    pub fn load_backgrounds_from_file<P: AsRef<Path>>(&mut self, path: P, factory: &mut GfxFactory)
        -> Result<(), ConfigImportError> {
        let bgs = load_backgrounds_from_file(path, factory)?;
        Ok(self.load_backgrounds(bgs))
    }
    pub fn load_backgrounds_from_str(&mut self, text: &str, factory: &mut GfxFactory)
        -> Result<(), ConfigImportError> {
        let bgs = load_backgrounds_from_str(text, factory)?;
        Ok(self.load_backgrounds(bgs))
    }
    fn load_backgrounds(&mut self, map: HashMap<String, Arc<G2dTexture>>) {
        self.backgrounds.extend(map);
    }
    pub fn load_scripts_from_file<P: AsRef<Path>>(&mut self, path: P)
        -> Result<(), ScriptConfigImportError> {
        let scripts = load_scripts_from_file(path)?;
        Ok(self.load_scripts(scripts))
    }
    pub fn load_scripts_from_str(&mut self, text: &str)
        -> Result<(), ScriptConfigImportError> {
        let scripts = load_scripts_from_str(text)?;
        Ok(self.load_scripts(scripts))
    }
    fn load_scripts(&mut self, file: ScriptsFromFile) {
        self.story.load_scripts(file.map.clone());
        if let Some(e) = file.default {
            self.story.set_script(&e, None);
        }
    }
}
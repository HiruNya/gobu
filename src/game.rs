//! The module which uses all the other modules to make a game.

use piston_window::{
    clear,
    color,
    G2d,
    context::Context,
    G2dTexture,
    Event,
    GfxFactory,
};
use coord::vec2::Vec2;
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
            ScriptsFromFile,
        },
    },
    input::GameInput,
    error::{
        ConfigImportError,
        ScriptConfigImportError,
        MusicError,
    },
    music::{
        Music,
    },
    animation::Animation,
};

/// The game
pub struct Game {
    /// The rectangle that represents the game window
    pub size: Rect,
    /// The UI component which manages the textbox(s)
    pub ui: Ui,
    /// The background of the game
    pub background: BackgroundImage,
    /// A HashMap of all the backgrounds that the background can be set to
    pub backgrounds: HashMap<String, Arc<G2dTexture>>,
    /// A HashMap of all the characters on the screen / in the world
    pub stage: HashMap<String, CharacterEntity>,
    /// A HashMap of all the characters that are or can be put onto the stage
    pub characters: HashMap<String, Character>,
    /// Holds a collection of all the scripts and also the current script
    pub story: Script,
    /// A grid that allows you to set the position without having to bother with the actual height and width of the screen
    pub grid: Grid,
    /// Holds all the possible inputs for the game and what events they produce
    pub input: GameInput,
    /// The music that is played in the game. (This is optional)
    pub music: Option<Music>,
    /// The animations that are displayed in the game like CharacterTransitions.
    pub anims: Animation,
}

impl Game {
    /// Create a new Game struct by providing the [width, height] of the screen.
    ///
    /// It is recommended to use [GameBuilder](./util/GameBuilder) to create your Game struct instead.
    pub fn new(size: [f64; 2]) -> Game {
        let size = Rect {
            pos: Pos::new(0., 0.),
            size: Vec2::new(
                size[0],
                size[1]
            ),
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
            music: None,
            anims: Animation::new(),
        }
    }
    /// Handles a piston event. Currently only going forward in the story is supported.
    pub fn handle_event(&mut self, event: &Event) {
        use self::Event::{
            Input,
            Loop,
        };
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
                                    for i in self.stage.values_mut() {
                                        i.finish()
                                    }
                                    self.next_step();
                                },
                            }
                        }
                    },
                    _ => {},
                }
            },
            Loop(l) => {
                use piston_window::Loop;
                match l {
                    Loop::Update(args) => {
                        for i in self.stage.values_mut() {
                            i.update(args.dt)
                        }
                    },
                    _ => {},
                }
            },
            _ => {},
        }
    }
    /// Draws all the components of the game onto the screen
    #[cfg(not(feature = "gfx_glyph_text"))]
    pub fn draw(&mut self, c: Context, g: &mut G2d, glyph_cache: &mut Glyphs) {
        clear(color::BLACK, g);
        self.background.draw(c, g);
        for character in self.stage.values() {
            character.draw(c, g);
        };
        self.ui.draw(c, g, glyph_cache);
    }
    /// Draws all the components of the game EXCEPT the text.
    ///
    /// Use [`draw_text`] to set the text first.
    #[cfg(feature = "gfx_glyph_text")]
    pub fn draw(&mut self, c: Context, g: &mut G2d) {
        clear(color::BLACK, g);
        self.background.draw(c, g);
        for character in self.stage.values() {
            character.draw(c, g);
        };
        self.ui.draw(c, g);
    }
    /// Slightly misleading as it doesn't draw it just yet, it just queues all the glyphs ready to be drawn.
    ///
    /// To truly draw the text use the [`draw_2d_with_text`] function that is implemented on [`PistonWindow`]
    /// but remember to import the [`GfxGlyph`] trait first.
    #[cfg(feature = "gfx_glyph_text")]
    pub fn draw_text(&mut self, brush: &mut GlyphBrush<Resources, GfxFactory>) {
        self.ui.draw_text(brush)
    }
    /// Add a [`Character`] to the HashMap of characters
    pub fn add_character(&mut self, name: String, character: Character) {
        self.characters.insert(name, character);
    }
    /// Add a texture to the HashMap of backgrounds
    pub fn add_background(&mut self, name: String, background: Arc<G2dTexture>) {
        self.backgrounds.insert(name, background);
    }
    /// Add a character to the stage using the ``character`` argument as the key to the
    /// characters HashMap and the ``name`` argument is the name of the [`CharacterEntity`]
    /// That is spawned.
    ///
    /// Returns True if the character is spawned to the stage.
    pub fn add_to_stage(&mut self, name: String, character: String) -> bool {
        if let Some(chara) = self.characters.get(&character) {
            if let Some(c) =  chara.spawn(character) {
                self.stage.insert(name, c);
                true
            } else {false}
        } else {false}
    }
    /// Set the background using the ``name`` argument as the key to its texture found
    /// in the backgrounds HashMap.
    pub fn set_background(&mut self, name: &String) {
        if let Some(bg) = self.backgrounds.get(name) {
            self.background.set_texture(bg.clone());
        }
    }
    /// Change the state of a [`CharacterEntity`] on the stage.
    /// Returns True if the entity is changed.
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
    /// Move a [`CharacterEntity`]'s position on the screen.
    /// (The position is relative to the grid you've specified).
    pub fn move_character(&mut self, name: &String, pos: Pos) {
        if let Some(entity) = self.stage.get_mut(name) {
            entity.set_pos(self.grid.get_pos(pos))
        }
    }
    /// Apply a grid to the game to make it easier to position characters.
    /// W is how many cells make up the width of the screen and h is how many cells
    /// make up the height of the cell.
    pub fn apply_grid(&mut self, w: u32, h: u32) {
        self.grid = Grid::new(w, h, self.size)
    }
    /// Set the main textbox of the game.
    pub fn set_textbox(&mut self, text_box: TextBox) {
        self.ui.textbox = text_box;
    }
    /// Set the speaker box which is the textbox that holds the name of the current speaker.
    /// Is not shown by default.
    pub fn set_speaker_box(&mut self, speaker_box: TextBox) {
        self.ui.speaker_box = Some(speaker_box);
    }
    /// Enables playing music in the game and returns a mutable reference in a result
    /// to the newly created [`Music`] struct that can be used to play music.
    /// ```rust, no_run
    ///    if let Ok(music) = game.enable_music() {
    ///        music.add_music_from_file("gabriel", "C:\\Users\\Hiruna\\Music\\Songs\\01. ガヴリールドロップキック.flac")
    ///            .expect("Error in loading the music!");
    ///        music.set_music("gabriel");
    ///    }
    /// ```
    pub fn enable_music(&mut self) -> Result<&mut Music, MusicError> {
        self.music = Some(Music::new()?);
        if let Some(ref mut e) = self.music {
            Ok(e)
        } else {
            Err(MusicError::NoDefaultOutputDeviceFound)
        }
    }
    /// Applies a ``CharacterTransition`` from anime onto a [`CharacterEntity`]
    pub fn apply_character_transition(&mut self, character_name: &String, transition_name: &String) {
        if let Some(entity) = self.stage.get_mut(character_name) {
            if let Some(trans) = self.anims.char_trans.get(transition_name) {
                entity.apply_trans(trans.create());
            }
        }
    }
    /// Load characters from a TOML file.
    pub fn load_characters_from_file<P: AsRef<Path>>(&mut self, path: P, factory: &mut GfxFactory)
        -> Result<(), ConfigImportError> {
        let map = load_characters_from_file(path, factory)?;
        Ok(self.load_characters(map))
    }
    /// Load characters from a [`&str`].
    /// Use this if you intend on compiling the TOML file and not keep it externally.
    /// Also could be used to load characters after decrypting a file.
    pub fn load_characters_from_str(&mut self, text: &str, factory: &mut GfxFactory) -> Result<(), ConfigImportError> {
        let map = load_characters_from_str(text, factory)?;
        Ok(self.load_characters(map))
    }
    fn load_characters(&mut self, map: HashMap<String, Character>) {
        for (k, v) in map.iter() {
            self.characters.insert(k.to_string(), v.clone());
        }
    }
    /// Load input from a TOML file.
    pub fn load_input_from_file<P: AsRef<Path>>(&mut self, path: P)
        -> Result<(), ConfigImportError> {
        let input = load_input_from_file(path)?;
        self.input.add_input(input);
        Ok(())
    }
    /// Load input from a [`&str`].
    /// Use this if you intend on compiling the TOML file and not keep it externally.
    /// Also could be used to load characters after decrypting a file.
    pub fn load_input_from_str(&mut self, text: &str)
        -> Result<(), ConfigImportError> {
        let input = load_input_from_str(text)?;
        self.input.add_input(input);
        Ok(())
    }
    /// Load the GUI from a TOML file.
    pub fn load_gui_from_file<P: AsRef<Path>>(&mut self, path: P)
        -> Result<(), ConfigImportError> {
        let gui = load_gui_from_file(path, self)?;
        self.load_gui(gui);
        Ok(())
    }
    /// Load the GUI from a [`&str`].
    /// Use this if you intend on compiling the TOML file and not keep it externally.
    /// Also could be used to load characters after decrypting a file.
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
    /// Load the backgrounds from a TOML file.
    pub fn load_backgrounds_from_file<P: AsRef<Path>>(&mut self, path: P, factory: &mut GfxFactory)
        -> Result<(), ConfigImportError> {
        let bgs = load_backgrounds_from_file(path, factory)?;
        Ok(self.load_backgrounds(bgs))
    }
    /// Load the backgrounds from a [`&str`].
    /// Use this if you intend on compiling the TOML file and not keep it externally.
    /// Also could be used to load characters after decrypting a file.
    pub fn load_backgrounds_from_str(&mut self, text: &str, factory: &mut GfxFactory)
        -> Result<(), ConfigImportError> {
        let bgs = load_backgrounds_from_str(text, factory)?;
        Ok(self.load_backgrounds(bgs))
    }
    fn load_backgrounds(&mut self, map: HashMap<String, Arc<G2dTexture>>) {
        self.backgrounds.extend(map);
    }
    /// Load the scripts from a TOML file.
    pub fn load_scripts_from_file<P: AsRef<Path>>(&mut self, path: P)
        -> Result<(), ScriptConfigImportError> {
        let scripts = load_scripts_from_file(path)?;
        Ok(self.load_scripts(scripts))
    }
    /// Load scripts from a [`&str`].
    /// Use this if you intend on compiling the TOML file and not keep it externally.
    /// Also could be used to load characters after decrypting a file.
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
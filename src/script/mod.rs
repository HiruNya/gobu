//! Manages the story of the VN and also parsing script files.
//!
//! Script files a just text files that have commands which causes:
//! * Dialogue to change
//! * Characters to be spawned, killed, shown, and hidden
//! * The background to change
//!
//! See the example_vn repo's script.txt file for an example of what can be done
//! with this language.

pub mod parser;

use std::{
    path::Path,
    fs::File,
    io::Read,
};
use indexmap::IndexMap;

use super::{
    game::Game,
    error::ScriptImportError,
};
use self::parser::translate;

/// Represents an action dictated by the script
#[derive(Debug, Clone)]
pub enum ScriptStep {
    /// A dialogue that consists of the name of the speaker and the text.
    /// (Name, Text)
    Dialogue(String, String),
    /// Only changes the text in the textbox. Doesn't change the speaker.
    DialogueContinue(String),
    /// Shows an entity on the screen and if the state is not ``None`` will change that
    /// entity's state to the state named.
    /// (Entity, Some(State), Some(Transition)
    Show(String, Option<String>, Option<String>), // Show(Entity, State)
    /// Hides an entity using its name
    /// (Entity, Some(Transition)
    Hide(String, Option<String>),
    /// Spawns a character with a specific entity name and also possibly a position.
    /// The last element is a transition that can be used.
    /// (CharacterName, Some(EntityName), Some(Position), Some(TransitionName))
    Spawn(String, Option<String>, Option<(f64, f64)>, Option<String>),
    /// Kill an entity
    /// (Entity, Some(Transition))
    Kill(String, Option<String>),
    /// Move an entity to a specific position
    Move(String, (f64, f64)),
    /// Set the background
    Stage(String),
    /// Go to a specific script or part of a script
    /// (ScriptName, AnchorName)
    GoTo(Option<String>, Option<String>),
    /// Play music
    Play(String),
    /// End of the script/game
    End,
}

/// The struct in charge of the story in the VN and also in charge of parsing everything.
pub struct Script {
    /// The current position in the script.
    step: usize,
    /// The current script that is being used.
    pub script: Vec<ScriptStep>,
    /// A collection of all the scripts that can be used.
    pub scripts: IndexMap<String, IndexMap<String, Vec<ScriptStep>>>,
    /// The current index of what script is being used and what anchor is being used.
    pub index: (usize, usize),
}
impl Script {
    /// Create a new [`Script`] struct.
    pub fn new() -> Script {
        Script {
            step: 0,
            script: Vec::new(),
            scripts: IndexMap::new(),
            index: (0, 0)
        }
    }
    /// Set a script with its name and maybe its anchor.
    pub fn set_script(&mut self, name: &str, anchor: Option<String>) {
        let mut index = self.index;
        self.script = match self.scripts.get_full(name) {
            Some((i, _, map)) => {
                index.0 = i;
                if let Some(e) = anchor {
                    if let Some(f) = map.get_full(&e) {
                        index.1 = f.0;
                        f.2.clone()
                    } else {
                        index.1 = 0;
                        Vec::new()
                    }
                } else {
                    index.1 = 0;
                    if let Some(e) = map.get_index(0) {
                        e.1.clone()
                    } else {
                        Vec::new()
                    }
                }
            },
            None => {
                index = (0, 0);
                Vec::new()
            },
        };
        self.step = 0;
        self.index = index;
    }
    /// Go to the next script.
    pub fn next_script(&mut self) {
        if let Some((_, map)) = self.scripts.get_index(self.index.0) {
            self.index.1 += 1;
            if let Some((_, v)) = map.get_index(self.index.1) {
                self.script = v.clone();
                self.step = 0;
            } else {
                self.script = vec![ScriptStep::End];
            }
        }
    }
    /// Load a script into the collection of scripts.
    pub fn load_script(&mut self, name: String, scripts: IndexMap<String, Vec<ScriptStep>>) {
        self.scripts.insert(name, scripts);
    }
    /// Load a number of scripts into the collection of scripts.
    pub fn load_scripts(&mut self, map: IndexMap<String, IndexMap<String, Vec<ScriptStep>>>) {
        for (k, v) in map.iter() {
            self.load_script(k.to_string(), v.clone());
        }
    }
    /// Load scripts from a [`&str`].
    pub fn load_from_str(&mut self, name: String, content: &str) -> Result<(), ScriptImportError> {
        self.load_script(name, translate(content)?);
        Ok(())
    }
    /// Load scripts from an external file.
    pub fn load_from_file<P: AsRef<Path>>(&mut self, name: String, path: P) -> Result<(), ScriptImportError> {
        let mut buffer = String::new();
        File::open(path)?.read_to_string(&mut buffer)?;
        self.load_from_str(name,&buffer)
    }
}

impl Game {
    /// Load the next step of the script and execute the step.
    pub fn next_step(&mut self) {
        let length = self.story.script.len();
        let mut execute = true;
        while execute {
            if self.story.step < length {
                self.story.step += 1;
                let possible_step;
                {
                    possible_step = self.story.script.get(self.story.step - 1).cloned()
                }
                if let Some(step) = possible_step {
                    execute = self.execute_step(step);
                } else {
                    self.story.next_script();
                    self.next_step();
                    break
                }
            } else {
                self.story.next_script();
                self.next_step();
                break
            }
        }
    }
    fn execute_step(&mut self, step: ScriptStep) -> bool {
        // boolean value is whether to continue or not.
        // True => Continue
        // False => Stop
        use self::ScriptStep::*;
        match step {
            Dialogue(speaker, content) => {
                self.ui.textbox.set_text(content.clone());
                if let Some(ref mut e) = self.ui.speaker_box {
                    e.set_text(speaker.to_string());
                }
                false
            },
            DialogueContinue(content) => {
                self.ui.textbox.set_text(content.clone());
                false
            },
            Show(image, possible_state, trans) => {
                if let Some(e) = self.stage.get_mut(&image) {
                    e.set_visible(true);
                }
                if let Some(state) = possible_state {
                    self.change_entity_state(&image, &state);
                }
                if let Some(t) = trans {
                    self.apply_character_transition(&image, &t);
                }
                true
            },
            Hide(image, trans) => {
                let mut vis = false;
                if let Some(t) = trans {
                    self.apply_character_transition(&image, &t);
                    if let Some(e) = self.stage.get_mut(&image) {
                        e.to_be_hidden = true;
                    }
                    vis = true;
                }
                if let Some(e) = self.stage.get_mut(&image) {
                    e.visible = vis;
                }
                true
            },
            Spawn(character, entity, maybe_pos, maybe_trans) => {
                let name = entity.clone().unwrap_or(character.clone());
                self.add_to_stage(name.clone(), character.clone());
                if let Some(pos) = maybe_pos {
                    self.move_character(&name, pos.into());
                }
                if let Some(trans) = maybe_trans {
                    self.apply_character_transition(&name, &trans);
                }
                true
            },
            Kill(name, trans) => {
                let mut rem = true;
                if let Some(t) = trans {
                    self.apply_character_transition(&name, &t);
                    if let Some(e) = self.stage.get_mut(&name) {
                        e.to_be_killed = true;
                    }
                    rem = false;
                }
                if rem {
                    self.stage.remove(&name);
                }
                true
            },
            Move(name, pos) => {
                self.move_character(&name, pos.into());
                true
            },
            Stage(bg) => {
                self.set_background(&bg);
                true
            },
            GoTo(name, anchor) => {
                let name = {
                    if let Some(n) = name {
                        n
                    } else {
                        if let Some((k, _)) = self.story.scripts.get_index(self.story.index.0) {
                            k.to_string()
                        } else {
                            String::new()
                        }
                    }
                };
                self.story.set_script(&name, anchor);
                true
            },
            Play(name) => {
                if let Some(ref mut m) = self.music {
                    // Ignore the result.
                    // In the future, this might actually be used.
                    // Maybe...
                    let _ = m.set_music(&name);
                }
                true
            }
            End => {
                self.story.step -= 1;
                false
            }
        }
    }
}
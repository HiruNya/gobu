pub mod parser;

use std::{
    path::Path,
    fs::File,
    io::Read,
};
use indexmap::IndexMap;

use super::{
    game::Game,
    Pos,
    error::ScriptImportError,
};
use self::parser::translate;

#[derive(Debug, Clone)]
pub enum ScriptStep {
    Dialogue(String, String),
    DialogueContinue(String),
    Show(String, Option<String>), // Show(Entity, State)
    Hide(String),
    Spawn(String, Option<String>, Option<(f64, f64)>),
    Kill(String),
    Move(String, (f64, f64)),
    Stage(String),
    GoTo(Option<String>, Option<String>),
    End,
}

pub struct Script {
    step: usize,
    script: Vec<ScriptStep>,
    scripts: IndexMap<String, IndexMap<String, Vec<ScriptStep>>>,
    index: (usize, usize),
}
impl Script {
    pub fn new() -> Script {
        Script {
            step: 0,
            script: Vec::new(),
            scripts: IndexMap::new(),
            index: (0, 0)
        }
    }
//    pub fn add_step(&mut self, step: ScriptStep) {
//        self.script.push(step)
//    }
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
    pub fn load_script(&mut self, name: String, scripts: IndexMap<String, Vec<ScriptStep>>) {
        self.scripts.insert(name, scripts);
    }
    pub fn load_scripts(&mut self, name: String, map: IndexMap<String, Vec<ScriptStep>>) {
        self.load_script(name, map);
    }
    pub fn load_from_str(&mut self, name: String, content: &str) -> Result<(), ScriptImportError> {
        self.load_scripts(name, translate(content)?);
        Ok(())
    }
    pub fn load_from_file<P: AsRef<Path>>(&mut self, name: String, path: P) -> Result<(), ScriptImportError> {
        let mut buffer = String::new();
        File::open(path)?.read_to_string(&mut buffer)?;
        self.load_from_str(name,&buffer)
    }
}

impl Game {
    pub fn next_step(&mut self) {
        let length = self.story.script.len();
        loop {
            if self.story.step < length {
                self.story.step += 1;
                let possible_step;
                {
                    possible_step = self.story.script.get(self.story.step - 1).cloned()
                }
                if let Some(step) = possible_step {
                    use self::ScriptStep::*;
                    match step {
                        Dialogue(speaker, content) => {
                            self.ui.textbox.set_text(content.clone());
                            if let Some(ref mut e) = self.ui.speaker_box {
                                e.set_text(speaker.to_string());
                            }
                            break
                        },
                        DialogueContinue(content) => {
                            self.ui.textbox.set_text(content.clone());
                            break
                        },
                        Show(image, possible_state) => {
                            if let Some(e) = self.stage.get_mut(&image) {
                                e.visible = true;
                            }
                            if let Some(state) = possible_state {
                                self.change_entity_state(&image, &state);
                            }
                        },
                        Hide(image) => {
                            if let Some(e) = self.stage.get_mut(&image) {
                                e.visible = false;
                            };
                        },
                        Spawn(character, entity, maybe_pos) => {
                            let name = entity.clone().unwrap_or(character.clone());
                            self.add_to_stage(name.clone(), character.clone());
                            if let Some(pos) = maybe_pos {
                                self.move_character(&name, pos.into())
                            }
                        },
                        Kill(name) => {
                            self.stage.remove(&name);
                        },
                        Move(name, pos) => {
                            self.move_character(&name, pos.into());
                        },
                        Stage(bg) => {
                            self.set_background(&bg);
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
                            self.next_step()
                        },
                        End => {
                            self.story.step -= 1;
                            break;
                        }
                    }
                } else {
                    self.story.next_script();
                    break
                }
            } else {
                self.story.next_script();
                break
            }
        }
    }
}

impl From<(f64, f64)> for Pos {
    fn from(tuple: (f64, f64)) -> Pos {
        Pos {x: tuple.0, y: tuple.1}
    }
}
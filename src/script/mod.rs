pub mod parser;
use super::{
    game::Game,
    Pos,
};
use self::parser::translate;

#[derive(Debug, Clone)]
pub enum ScriptStep {
    Dialogue(String, String),
    DialogueContinue(String),
    Show(String),
    Hide(String),
    Spawn(String, Option<String>, Option<(f64, f64)>),
    Kill(String),
    Move(String, (f64, f64)),
}

pub struct Script {
    step: usize,
    script: Vec<ScriptStep>,
}
impl Script {
    pub fn new() -> Script {
        Script {
            step: 0,
            script: Vec::new(),
        }
    }
    pub fn add_step(&mut self, step: ScriptStep) {
        self.script.push(step)
    }
    pub fn load_script(&mut self, script: Vec<ScriptStep>) {
        self.script = script;
        self.step = 0;
    }
    pub fn load_from_str(&mut self, content: &str) -> Result<(), String> {
        self.load_script(translate(content)?);
        Ok(())
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
                        Show(image) => {
                            if let Some(e) = self.stage.get_mut(&image) {
                                e.visible = true;
                            };
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
                    }
                } else {break}
            } else {break}
        }
    }
}

impl From<(f64, f64)> for Pos {
    fn from(tuple: (f64, f64)) -> Pos {
        Pos {x: tuple.0, y: tuple.1}
    }
}
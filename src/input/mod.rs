mod event;

use std::collections::HashSet;
use piston_window::Button;
pub use self::event::GameEvent;

#[derive(Debug)]
pub struct GameInput {
    pub continue_: HashSet<Button>
}
impl GameInput {
    pub fn new() -> Self {
        GameInput {
            continue_: HashSet::new(),
        }
    }
    pub fn handle_event(&self, button: &Button) -> Option<GameEvent> {
        if self.continue_.contains(button) { return Some(GameEvent::Continue) };
        None
    }
    pub fn add_continue_event(&mut self, button: Button) {
        self.continue_.insert(button);
    }
    pub fn add_continue_events(&mut self, buttons: HashSet<Button>) {
        self.continue_.extend(buttons);
    }
    pub fn add_input(&mut self, input: GameInput) {
        self.add_continue_events(input.continue_);
    }
}

//pub struct Input {
//    pub state: ButtonState,
//    pub button: Button,
//}
//impl PartialEq for Input {
//    fn eq(&self, other: &ButtonArgs) -> bool {
//        if self.button == other.button {
//            if self.state == other.state { true }
//        }
//        false
//    }
//}
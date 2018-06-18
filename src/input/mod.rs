mod event;

use piston_window::Button;
pub use self::event::GameEvent;

#[derive(Debug)]
pub struct GameInput {
    pub continue_: Vec<Button>
}
impl GameInput {
    pub fn new() -> Self {
        GameInput {
            continue_: Vec::new(),
        }
    }
    pub fn handle_event(&self, button: &Button) -> Option<GameEvent> {
        if self.continue_.contains(button) { return Some(GameEvent::Continue) };
        None
    }
    pub fn add_continue_event(&mut self, button: Button) {
        self.continue_.push(button);
    }
    pub fn add_continue_events(&mut self, buttons: &mut Vec<Button>) {
        self.continue_.append(buttons);
    }
    pub fn add_input(&mut self, input: &mut GameInput) {
        self.add_continue_events(&mut input.continue_);
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
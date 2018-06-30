//! The Piston events are taken and turned into [`GameEvents`]
//!
//! The events are then used to dictate what happens in the game.
//! Currently on Continue is supported.
//!
//! This is done in this way so that multiple keys or buttons can be used to trigger
//! the same GameEvent.

mod event;

use std::collections::HashSet;
use piston_window::Button;
pub use self::event::GameEvent;

/// The struct in charge of handling the input of the game.
#[derive(Debug)]
pub struct GameInput {
    /// the buttons that continue the story of the game.
    pub continue_: HashSet<Button>
}
impl GameInput {
    /// Create a new ``GameInput`` struct
    pub fn new() -> Self {
        GameInput {
            continue_: HashSet::new(),
        }
    }
    /// Handle a Piston Event and return a ``GameEvent``.
    pub fn handle_event(&self, button: &Button) -> Option<GameEvent> {
        if self.continue_.contains(button) { return Some(GameEvent::Continue) };
        None
    }
    /// Add an event that will continue the story.
    pub fn add_continue_event(&mut self, button: Button) {
        self.continue_.insert(button);
    }
    /// Add events that will continue the story.
    pub fn add_continue_events(&mut self, buttons: HashSet<Button>) {
        self.continue_.extend(buttons);
    }
    /// Add input in the form of a ``GameInput`` struct.
    /// Just joins all of the data together.
    pub fn add_input(&mut self, input: GameInput) {
        self.add_continue_events(input.continue_);
    }
}
//! The module that manages all the animations and transitions.
//!
//! You can create your own Transition by implementing the [`CharacterTransition`] trait on a struct
//! and adding it to the HashMap of transitions.

pub mod premade;

use piston_window::Image;
use std::collections::HashMap;

/// The struct that is in charge of holding all the animations in the game.
pub struct Animation {
    /// A [`HashMap`] of all the different [`CharacterTransition`]s.
    pub char_trans: HashMap<String, Box<CharacterTransition>>
}
impl Animation {
    /// Create a new [`Animation`] struct.
    pub fn new() -> Self {
        Animation {
            char_trans: HashMap::new(),
        }
    }
    /// Take the transitions from one struct and put it into itself.
    pub fn extend(&mut self, other: Animation) {
        self.char_trans.extend(other.char_trans);
    }
    /// Insert a struct that implements [`CharacterTransition`] into the hashmap of [`CharacterTransition`]s
    pub fn insert_char_trans(&mut self, name: String, trans: Box<dyn CharacterTransition>) {
        self.char_trans.insert(name, trans);
    }
}

/// A trait for Character Transitions like FadeIn
pub trait CharacterTransition {
    /// Create copy of itself. This is different to ``new`` as it requires a reference to self.
    /// This is because the struct which implements this trait shall be stored in a HashMap and
    /// therefore when a transition is needed we take the object in the HashMap and call [`create`]
    /// on it.
    fn create(&self) -> Box<CharacterTransition>;
    /// Every time the game updates, the entity will call this method and provide it's image.
    /// ``delta_time`` is the amount of time that has passed since the last update event.
    /// This will return a TransResult. If ``Finished`` then the animation shall be removed.
    /// If ``Continue`` nothing will happen.
    fn update(&mut self, image: &mut Image, delta_time: f64) -> TransResult;
    /// If the user wanted to skip the animation then this method will be called to make the character
    /// go to its final destination. For example if this was FadeIn, the image be fully opaque.
    /// If this was SlideIn, the image would be at its final destination.
    fn finish(&mut self, image: &mut Image);
}

/// The result of a transition.
#[derive(PartialEq)]
pub enum TransResult {
    /// Do nothing else, this transition will be called next update.
    Continue,
    /// This transition has finished therefore remove it from whatever struct holds it.
    Finished,
}

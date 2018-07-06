use piston_window::image::Image;
use super::super::super::{
    TransResult,
    CharacterTransition,
};

/// Fades the character in from being fully transparent to being fully opaque.
#[derive(Clone, Copy)]
pub struct FadeIn {
    time_elapsed: f32,
    /// The time it takes for the FadeIn to finish.
    pub in_time: f32,
}
impl FadeIn {
    /// Create a new [`FadeIn`] struct giving in the amount of time you wish the transition to end by.
    pub fn new(time: f32) -> Box<dyn CharacterTransition> {
        Box::new(FadeIn {
            time_elapsed: 0.,
            in_time: time,
        })
    }
}
impl CharacterTransition for FadeIn {
    fn create(&self) -> Box<dyn CharacterTransition> {
        Box::new(*self)
    }
    fn update(&mut self, image: &mut Image, delta_time: f64) -> TransResult {
        self.time_elapsed += delta_time as f32;
        image.color = Some([1., 1., 1., self.time_elapsed as f32 / self.in_time]);
        if self.time_elapsed > self.in_time {
            return TransResult::Finished
        }
        TransResult::Continue
    }
    fn finish(&mut self, image: &mut Image) {
        image.color = Some([1.; 4]);
    }
}

/// Fades the character in from being fully transparent to being fully opaque.
#[derive(Clone, Copy)]
pub struct FadeOut {
    time_left: f32,
    /// The time it takes for the FadeOut to finish.
    pub in_time: f32,
}
impl FadeOut {
    /// Create a new [`FadeOut`] struct giving in the amount of time you wish the transition to end by.
    pub fn new(time: f32) -> Box<dyn CharacterTransition> {
        Box::new(FadeOut {
            time_left: time,
            in_time: time,
        })
    }
}
impl CharacterTransition for FadeOut {
    fn create(&self) -> Box<dyn CharacterTransition> {
        Box::new(*self)
    }
    fn update(&mut self, image: &mut Image, delta_time: f64) -> TransResult {
        self.time_left -= delta_time as f32;
        image.color = Some([1., 1., 1., self.time_left as f32 / self.in_time]);
        if self.time_left < 0. {
            return TransResult::Finished
        }
        TransResult::Continue
    }
    fn finish(&mut self, image: &mut Image) {
        image.color = Some([1., 1., 1., 0.]);
    }
}
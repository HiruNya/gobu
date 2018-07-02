//! Manages the UI, which consists of only the textbox for now.
//!
//! In the future it is planned to also be in control of on-screen buttons as well.

use piston_window::{
    context::Context,
    G2d,
};
use coord::vec2::Vec2;
#[cfg(not(feature = "gfx_glyph_text"))]
use ::piston_window::Glyphs;
#[cfg(feature = "gfx_glyph_text")]
use ::{
    gfx_glyph::GlyphBrush,
    gfx_device_gl::Resources,
    piston_window::GfxFactory,
};

pub mod textbox;
mod grid;

pub use self::{
    textbox::{
        TextBox,
        TextBoxBuilder,
        Padding,
    },
    grid::Grid,
};
use super::Rect;

/// A struct that manages all the textboxes.
pub struct Ui {
    /// The main textbox that displays the text.
    pub textbox: TextBox,
    /// The textbox that displays the name of the speaker.
    /// If ``None`` then nothing will be shown.
    pub speaker_box: Option<TextBox>,
}

impl Ui {
    /// Creates a new [`Ui`] struct.
    pub fn new(canvas: Rect) -> Ui {
        let tb = TextBox::new(
            Rect {
                pos: vec2![0.025, 0.675] * canvas.pos,
                size: vec2![0.95, 0.5] * canvas.size,
//                x: 0.025 * canvas.w,
//                y: 0.675 * canvas.h,
//                w: 0.95 * canvas.w,
//                h: 0.3 * canvas.h,
            }
        );
        Ui {
            textbox: tb,
            speaker_box: None,
        }
    }
    /// Draws the components that the Ui contains.
    #[cfg(not(feature = "gfx_glyph_text"))]
    pub fn draw(&mut self, c: Context, g: &mut G2d, glyph_cache: &mut Glyphs) {
        self.textbox.draw(c, g, glyph_cache);
        if let Some(ref mut e) = self.speaker_box {
            e.draw(c, g, glyph_cache)
        }
    }
    /// Draws the components that the Ui contains.
    #[cfg(feature = "gfx_glyph_text")]
    pub fn draw(&mut self, c: Context, g: &mut G2d) {
        self.textbox.draw(c, g);
        if let Some(ref mut e) = self.speaker_box {
            e.draw(c, g)
        }
    }
    /// Queues the text into the Brush that will be drawn with [`draw_2d_with_text`]
    #[cfg(feature = "gfx_glyph_text")]
    pub fn draw_text(&mut self, brush: &mut GlyphBrush<Resources, GfxFactory>) {
        self.textbox.draw_text(brush);
        if let Some(ref mut e) = self.speaker_box {
            e.draw_text(brush)
        }
    }
//    pub fn resize(&mut self, canvas: Rect) {
//        self.textbox.resize(canvas);
//    }
}

//trait Widget {
//    fn draw(&mut self, _: Context, _: &mut G2d, _: &mut Glyphs) {}
//}
use std::collections::HashMap;
use piston_window::{
    context::Context,
    G2d,
    Glyphs,
};

mod textbox;
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

pub struct Ui {
    widgets: HashMap<u16, TextBox>,
    pub textbox: TextBox,
    pub speaker_box: Option<TextBox>,
}

impl Ui {
    pub fn new(canvas: Rect) -> Ui {
        let map = HashMap::new();
        let tb = TextBox::new(
            Rect {
                x: 0.025 * canvas.w,
                y: 0.675 * canvas.h,
                w: 0.95 * canvas.w,
                h: 0.3 * canvas.h,
            }
        );
        Ui {
            widgets: map,
            textbox: tb,
            speaker_box: None,
        }
    }
    pub fn draw(&mut self, c: Context, g: &mut G2d, glyph_cache: &mut Glyphs) {
        for v in self.widgets.values_mut() {
            v.draw(c, g, glyph_cache);
        }
        self.textbox.draw(c, g, glyph_cache);
        if let Some(ref mut e) = self.speaker_box {
            e.draw(c, g, glyph_cache)
        }
    }
//    pub fn resize(&mut self, canvas: Rect) {
//        self.textbox.resize(canvas);
//    }
}

trait Widget {
    fn draw(&mut self, _: Context, _: &mut G2d, _: &mut Glyphs) {}
}
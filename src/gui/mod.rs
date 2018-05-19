use std::collections::HashMap;
use piston_window::{
    context::Context,
    G2d,
    Glyphs,
};

mod textbox;

pub use self::textbox::TextBox;
use super::Rect;

pub struct Ui {
    widgets: HashMap<u16, TextBox>,
    textbox: TextBox,
}

impl Ui {
    pub fn new(rect: Rect) -> Ui {
        let map = HashMap::new();
        let mut tb = TextBox::new(
            Rect {
                x: 0.025,
                y: 0.675,
                w: 0.95,
                h: 0.3,
            },
            Rect {
                x: rect.x,
                y: rect.y,
                w: rect.w,
                h: rect.h,
            }
        );
        tb.set_text("This is best girl.\nAqua is shit BTW.".to_string());
        Ui {
            widgets: map,
            textbox: tb,
        }
    }
    pub fn draw(&mut self, c: Context, g: &mut G2d, glyph_cache: &mut Glyphs) {
        for v in self.widgets.values_mut() {
            v.draw(c, g, glyph_cache);
        }
        self.textbox.draw(c, g, glyph_cache);
    }
    pub fn resize(&mut self, canvas: Rect) {
        self.textbox.resize(canvas);
    }
}

trait Widget {
    fn draw(&mut self, _: Context, _: &mut G2d, _: &mut Glyphs) {}
}
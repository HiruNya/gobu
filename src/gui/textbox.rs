use piston_window::{
    G2d,
    Rectangle,
    context::Context,
    draw_state::DrawState,
    Text,
    Glyphs,
    Transformed,
    character::CharacterCache,
};

use super::{
    Widget,
    Rect,
};
use game::Game;

pub struct TextBox {
    pub rect: Rectangle,
//    pub percent: Rect,
    pub inner: Rect,
    pub outer: Rect,
    pub text: Vec<String>,
    pub text_pos: f64,
    pub padding: Padding,
    pub changed: TextBoxChanged,
    pub text_primitive: Text,
}

impl TextBox {
    pub fn new(rect: Rect) -> TextBox {
        let padding = Padding::Hv(0.025, 0.1);
        TextBox {
//            percent: Rect {
//                x: rect.x,
//                y: rect.y,
//                w: rect.w,
//                h: rect.h,
//            },
            outer: rect,
            inner: padding.calculate_inner_rect(rect),
            rect: Rectangle::new([0., 0., 0., 1.]),
            text: vec![],
            text_pos: 0.,
            padding,
            changed: TextBoxChanged::new(),
            text_primitive: Text::new_color([1., 1., 1., 4.], 13),
        }
    }
    pub fn set_text(&mut self, text: String) {
        self.text = text.split('\n')
            .fold(vec![], |mut lines, l| {
                lines.push(l.to_string());
                lines
            });
        self.changed.text = true;
    }
    fn calculate_inner(&mut self) {
        self.inner = self.padding.calculate_inner_rect(self.outer);
    }
//    pub fn resize(&mut self, canvas: Rect) {
//        let outer_rect = Rect {
//            x: canvas.x + (canvas.w * self.percent.x),
//            y: canvas.y + (canvas.h * self.percent.y),
//            w: canvas.w * self.percent.w,
//            h: canvas.h * self.percent.h,
//        };
//        self.outer = outer_rect;
//        self.inner = self.padding.calculate_inner_rect(outer_rect);
//        self.position_text();
//    }
    fn position_text(&mut self) {
        let y = {
            let font_size = 13;
            self.inner.y + (font_size as f64)
        };
        self.text_pos = y;
    }
    // Pretty much copied from ggez
    fn wrap_text(&mut self, cache: &mut Glyphs) {
        let mut new_text = Vec::new();
        for line in self.text.iter() {
            let mut current_line = String::new();
            for word in line.split_whitespace() {
                let mut possible_line = current_line.clone();
                if !possible_line.is_empty() { possible_line.push(' ') };
                possible_line.push_str(word);
                let text_width = match cache.width(13, possible_line.as_str()) {
                    Ok(e) => e,
                    Err(_) => 100.,
                };
                if text_width > self.inner.w {
                    new_text.push(current_line);
                    current_line = word.to_string();
                } else {
                    current_line = possible_line;
                }
            }
            if !current_line.is_empty() {
                new_text.push(current_line);
            }
        }
        self.text = new_text;
    }
}

impl Widget for TextBox {
    fn draw(&mut self, c: Context, g: &mut G2d, glyph_cache: &mut Glyphs) {
        self.rect
            .draw(
                [
                    self.outer.x,
                    self.outer.y,
                    self.outer.w,
                    self.outer.h
                ],
                &DrawState::default(),
                c.transform,
                g,
            );
        if self.changed.text {
            self.wrap_text(glyph_cache);
            self.position_text();
            self.changed.text = false;
        }
        for (i, l) in self.text.iter().enumerate() {
            Text::new_color([1.; 4], 13)
                .draw(
                    l.as_str(),
                    glyph_cache,
                    &DrawState::default(),
                    c.transform.trans(self.inner.x, self.text_pos + (20 * i) as f64),
                    g,
                ).expect("Panicked when drawing text!");
        };
    }
}

pub struct TextBoxBuilder {
    rectangle_colour: Option<[f32; 4]>,
    text_colour: Option<[f32; 4]>,
    font_size: Option<u32>,
    rectangle: Option<Rect>,
    padding: Option<Padding>,
}
impl TextBoxBuilder {
    pub fn new() -> TextBoxBuilder {
        TextBoxBuilder {
            rectangle_colour: None,
            text_colour: None,
            font_size: None,
            rectangle: None,
            padding: None,
        }
    }
    pub fn build(self, game: &Game) -> TextBox {
        let rect = self.rectangle.unwrap_or( Rect{x: 0., y: 0., w: 0., h: 0.} );
        let mut text_box = TextBox::new(game.grid.get_abs_rect(rect));
        let text_prim = Text::new_color(
            self.text_colour.unwrap_or([1.; 4]),
            self.font_size.unwrap_or(13)
        );
        text_box.text_primitive = text_prim;
        if let Some(col) = self.rectangle_colour {
            text_box.rect = Rectangle::new(col);
        };
        if let Some(pad) = self.padding {
            text_box.padding = pad;
            text_box.calculate_inner();
        }
        text_box
    }
    pub fn with_colour(mut self, col: [f32; 4]) -> Self {
        self.rectangle_colour = Some(col);
        self
    }
    pub fn with_text_colour(mut self, col: [f32; 4]) -> Self {
        self.text_colour = Some(col);
        self
    }
    pub fn with_font_size(mut self, size: u32) -> Self {
        self.font_size = Some(size);
        self
    }
    pub fn with_rectangle(mut self, rect: Rect) -> Self {
        self.rectangle = Some(rect);
        self
    }
    pub fn with_padding(mut self, pad: Padding) -> Self {
        self.padding = Some(pad);
        self
    }

}

pub enum Padding {
    All(f64),
    Hv(f64, f64),
    Tblr(f64, f64, f64, f64),
    None,
}

impl Padding {
    fn calculate_inner_rect(&self, outer_rect: Rect) -> Rect {
        use self::Padding::*;
        match *self {
            All(pad) => {
                Rect {
                    x: outer_rect.x + (outer_rect.w * pad),
                    y: outer_rect.y + (outer_rect.h * pad),
                    w: outer_rect.w - (outer_rect.w * pad),
                    h: outer_rect.h - (outer_rect.h * pad),
                }
            }
            Hv(h, v) => {
                Rect {
                    x: outer_rect.x + (outer_rect.w * h),
                    y: outer_rect.y + (outer_rect.h * v),
                    w: outer_rect.w - (outer_rect.w * h),
                    h: outer_rect.h - (outer_rect.h * v),
                }
            }
            Tblr(t, b, l, r) => {
                Rect {
                    x: outer_rect.x + (outer_rect.w * l),
                    y: outer_rect.y + (outer_rect.h * t),
                    w: outer_rect.w - (outer_rect.w * r),
                    h: outer_rect.h - (outer_rect.h * b),
                }
            }
            None => outer_rect,
        }
    }
}

pub struct TextBoxChanged {
    pub pos: bool,
    pub size: bool,
    pub text: bool,
}

impl TextBoxChanged {
    fn new() -> TextBoxChanged {
        TextBoxChanged {
            pos: true,
            size: true,
            text: true,
        }
    }
}
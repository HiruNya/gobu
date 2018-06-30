//! A module that lets you build and manage the textboxes shown in the game.
//!
//! use [`TextBoxBuilder'] to build a [`TextBox`]

use piston_window::{
    G2d,
    Rectangle,
    context::Context,
    draw_state::DrawState,
};
#[cfg(not(feature = "gfx_glyph_text"))]
use ::piston_window::{
    Glyphs,
    Text,
    Transformed,
    character::CharacterCache,
};
#[cfg(feature = "gfx_glyph_text")]
use ::{
    gfx_glyph::{
        Section,
        GlyphBrush,
        Scale,
    },
    gfx_device_gl::Resources,
    piston_window::GfxFactory,
};
//use std::clone::Clone;
use super::Rect;
use game::Game;

/// A struct that represents and draws a textbox
#[derive(Clone)]
pub struct TextBox {
    /// The rectangle which holds the x and y co-ordinates and the width and the height.
    pub rect: Rectangle,
    /// The inner rectangle after applying padding. This is where the text will start.
    pub inner: Rect,
    /// The outer rectangle which is what the square is drawn using.
    pub outer: Rect,
    /// The y position of the text.
    pub text_pos: f64,
    /// The padding applied on the textbox.
    pub padding: Padding,
    /// Whether the text has been changed or not.
    pub text_changed: bool,
    // Only if we don't use the gfx_glyph crate to render text
    /// The Piston text primitive that is used.
    #[cfg(not(feature = "gfx_glyph_text"))]
    pub text_primitive: Text,
    /// The font size that is used with the Piston text primitive.
    #[cfg(not(feature = "gfx_glyph_text"))]
    pub font_size: u32,
    /// The text that is to be displayed. One item of the Vec is a line.
    #[cfg(not(feature = "gfx_glyph_text"))]
    pub text_v: Vec<String>,
    /// The gap between the lines.
    #[cfg(not(feature = "gfx_glyph_text"))]
    pub line_gap: u32,
    // If we are using the gfx_glyph crate to render text
    /// The string that is to be drawn. gfx_glyph takes care of the wrapping.
    #[cfg(feature = "gfx_glyph_text")]
    pub text: String,
    /// The colour of the text.
    #[cfg(feature = "gfx_glyph_text")]
    pub color: [f32; 4],
    /// The size of the font.
    #[cfg(feature = "gfx_glyph_text")]
    pub font_scale: Scale,
}
impl TextBox {
    /// Create a new ``TextBox`` struct.
    #[cfg(not(feature = "gfx_glyph_text"))]
    pub fn new(rect: Rect) -> TextBox {
        let padding = Padding::Hv(0.025, 0.1);
        TextBox {
            outer: rect,
            inner: padding.calculate_inner_rect(rect),
            rect: Rectangle::new([0., 0., 0., 1.]),
            text_pos: 0.,
            padding,
            text_changed: false,
            font_size: 13,
            text_v: vec![],
            line_gap: 7,
            text_primitive: Text::new_color([1., 1., 1., 4.], 13),
        }
    }
    /// Create a new ``TextBox`` struct.
    #[cfg(feature = "gfx_glyph_text")]
    pub fn new(rect: Rect) -> TextBox {
        let padding = Padding::Hv(0.025, 0.1);
        let inner = padding.calculate_inner_rect(rect);
        TextBox {
            outer: rect,
            inner,
            rect: Rectangle::new([0., 0., 0., 1.]),
            text_pos: 0.,
            padding,
            text_changed: false,
            font_scale: Scale::uniform(13.),
            text: String::new(),
            color: [1.; 4]
        }
    }
    /// Sets the text that is shown by the textbox.
    #[cfg(not(feature = "gfx_glyph_text"))]
    pub fn set_text(&mut self, text: String) {
        self.text_v = text.split('\n')
            .fold(vec![], |mut lines, l| {
                lines.push(l.to_string());
                lines
            });
        self.text_changed = true;
    }
    /// Sets the text that is shown by the textbox.
    #[cfg(feature = "gfx_glyph_text")]
    pub fn set_text(&mut self, text: String) {
        self.text = text;
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
    #[cfg(not(feature = "gfx_glyph_text"))]
    fn position_text(&mut self) {
        let y = self.inner.y + (self.font_size as f64);
        self.text_pos = y;
    }
    // Pretty much copied from ggez
    #[cfg(not(feature = "gfx_glyph_text"))]
    fn wrap_text(&mut self, cache: &mut Glyphs) {
        let mut new_text = Vec::new();
        for line in self.text_v.iter() {
            let mut current_line = String::new();
            for word in line.split_whitespace() {
                let mut possible_line = current_line.clone();
                if !possible_line.is_empty() { possible_line.push(' ') };
                possible_line.push_str(word);
                let text_width = match cache.width(self.font_size, possible_line.as_str()) {
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
        self.text_v = new_text;
    }
    /// Draw the textbox.
    #[cfg(not(feature = "gfx_glyph_text"))]
    pub fn draw(&mut self, c: Context, g: &mut G2d, glyph_cache: &mut Glyphs) {
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
        if self.text_changed {
            self.wrap_text(glyph_cache);
            self.position_text();
            self.text_changed = false;
        }
        for (i, l) in self.text_v.iter().enumerate() {
            self.text_primitive
                .draw(
                    l.as_str(),
                    glyph_cache,
                    &DrawState::default(),
                    c.transform.trans(self.inner.x, self.text_pos + ((self.font_size + self.line_gap) * i as u32) as f64),
                    g,
                ).expect("Panicked when drawing text!");
        };
    }
    /// Draw the textbox.
    #[cfg(feature = "gfx_glyph_text")]
    pub fn draw(&self, c: Context, g: &mut G2d) {
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
//        if self.text_changed {
//            self.wrap_text(glyph_cache);
//            self.position_text();
//            self.text_changed = false;
//        }
    }
    /// Queue the text of the textbox into the brush.
    #[cfg(feature = "gfx_glyph_text")]
    pub fn draw_text(&self, brush: &mut GlyphBrush<Resources, GfxFactory>) {
        let section = Section {
            text: &self.text,
            bounds: (self.inner.w as f32, self.inner.h as f32),
            color: self.color,
            screen_position: (self.inner.x as f32, self.inner.y as f32),
            scale: self.font_scale,
            ..Section::default()
        };
        brush.queue(section);
    }
}

/// A builder struct that helps build a ``TextBox``.
#[derive(Clone, Debug, Deserialize)]
pub struct TextBoxBuilder {
    rectangle_colour: Option<[f32; 4]>,
    text_colour: Option<[f32; 4]>,
    font_size: Option<u32>,
    rectangle: Option<Rect>,
    padding: Option<Padding>,
    #[cfg(not(feature = "gfx_glyph_text"))]
    line_gap: Option<u32>
}
impl TextBoxBuilder {
    /// Creates a new ``TextBoxBuilder``.
    pub fn new() -> TextBoxBuilder {
        TextBoxBuilder {
            rectangle_colour: None,
            text_colour: None,
            font_size: None,
            rectangle: None,
            padding: None,
            #[cfg(not(feature = "gfx_glyph_text"))]
            line_gap: None,
        }
    }
    /// Builds the ``TextBox``.
    #[cfg(not(feature = "gfx_glyph_text"))]
    pub fn build(self, game: &Game) -> TextBox {
        let rect = self.rectangle.unwrap_or( Rect{x: 0., y: 0., w: 0., h: 0.} );
        let mut text_box = TextBox::new(game.grid.get_abs_rect(rect));
        let text_prim = Text::new_color(
            self.text_colour.unwrap_or([1.; 4]),
            self.font_size.unwrap_or(13)
        );
        text_box.text_primitive = text_prim;
        if let Some(gap) = self.line_gap {
            text_box.line_gap = gap;
        }
        if let Some(col) = self.rectangle_colour {
            text_box.rect = Rectangle::new(col);
        };
        if let Some(pad) = self.padding {
            text_box.padding = pad;
            text_box.calculate_inner();
        }
        if let Some(size) = self.font_size {
            text_box.font_size = size;
        }
        text_box
    }
    /// Builds the ``TextBox``.
    #[cfg(feature = "gfx_glyph_text")]
    pub fn build(self, game: &Game) -> TextBox {
        let rect = self.rectangle.unwrap_or( Rect{x: 0., y: 0., w: 0., h: 0.} );
        let mut text_box = TextBox::new(game.grid.get_abs_rect(rect));
        if let Some(col) = self.rectangle_colour {
            text_box.rect = Rectangle::new(col);
        };
        if let Some(pad) = self.padding {
            text_box.padding = pad;
            text_box.calculate_inner();
        }
        if let Some(size) = self.font_size {
            text_box.font_scale = Scale::uniform(size as f32);
        }
        if let Some(col) = self.text_colour {
            text_box.color = col;
        }
        text_box
    }
    /// The colourr of the rectangle.
    pub fn with_colour(mut self, col: [f32; 4]) -> Self {
        self.rectangle_colour = Some(col);
        self
    }
    /// The colour of the text.
    pub fn with_text_colour(mut self, col: [f32; 4]) -> Self {
        self.text_colour = Some(col);
        self
    }
    /// The size of the text.
    pub fn with_font_size(mut self, size: u32) -> Self {
        self.font_size = Some(size);
        self
    }
    /// The rectangle dimensions of the textbox.
    pub fn with_rectangle(mut self, rect: Rect) -> Self {
        self.rectangle = Some(rect);
        self
    }
    /// The padding of the textbox.
    pub fn with_padding(mut self, pad: Padding) -> Self {
        self.padding = Some(pad);
        self
    }
    /// The gap between the lines of the text.
    #[cfg(not(feature = "gfx_glyph_text"))]
    pub fn with_line_gap(mut self, gap: u32) -> Self {
        self.line_gap = Some(gap);
        self
    }

}

/// The padding of the textbox.
#[derive(Clone, Debug, Deserialize)]
#[serde(untagged)]
pub enum Padding {
    /// Padding equally all around.
    All(f64),
    /// (Horizontal, Vertical)
    Hv(f64, f64),
    /// (Top, Bottom, Left, Right)
    Tblr(f64, f64, f64, f64),
    /// No padding
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
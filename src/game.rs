use piston_window::{
    clear,
    color,
    G2d,
    context::Context,
    Glyphs,
    G2dTexture,
    Event,
};

use super::{
    gui::Ui,
    images::BackgroundImage,
};

pub struct Game {
    pub size: Rect,
    pub ui: Ui,
    pub background: BackgroundImage,
}

impl Game {
    pub fn new(size: [f64; 2]) -> Game {
        let size = Rect {
            x: 0.,
            y: 0.,
            w: size[0],
            h: size[1],
        };
        Game{
            size,
            ui: Ui::new(size),
            background: BackgroundImage::new(size),
        }
    }
    pub fn handle_event(&mut self, event: &Event) {
        use self::Event::Input;
        match *event {
            Input(ref i) => {
                use piston_window::Input::{Resize};
                match i {
                    Resize(ref w, ref h) => {
                        self.resize(*w, *h);
                    },
                    _ => {},
                }
            },
            _ => {},
        }
    }
    pub fn draw(&mut self, c: Context, g: &mut G2d, glyph_cache: &mut Glyphs) {
        clear(color::BLACK, g);
        self.background.draw(c, g);
        self.ui.draw(c, g, glyph_cache);
    }
    pub fn set_background(&mut self, texture: G2dTexture) {
        self.background.set_texture(texture);
    }
    pub fn resize(&mut self, w: u32, h: u32) {
        let rect = Rect {x: 0., y: 0., w: w as f64, h: h as f64};
        self.background.resize(rect);
        self.ui.resize(rect);
    }
}

#[derive(Copy, Clone)]
pub struct Rect {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
}
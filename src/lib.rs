extern crate piston_window;

mod game;
mod gui;
mod images;

pub use game::*;

#[cfg(test)]
mod tests {
    #[test]
    fn try_start() {
        use game::*;
        use piston_window::{
            PistonWindow,
            WindowSettings,
            TextureSettings,
            glyph_cache::rusttype::GlyphCache,
            Texture,
            Flip,
            Event,
            Loop,
        };
        let mut window: PistonWindow = WindowSettings::new("Hello, World!", [640, 480])
            .exit_on_esc(true)
            .build()
            .unwrap();
        let mut game = Game::new([640., 480.]);
        let mut glyph_cache = GlyphCache::new(
            "../assets/Roboto-Regular.ttf",
            window.factory.clone(),
            TextureSettings::new()).unwrap();
        let background = Texture::from_path(
            &mut window.factory,
            "../assets/megumin.jpg",
            Flip::None,
            &TextureSettings::new()).unwrap();
        game.set_background(background);
        while let Some(event) = window.next() {
            game.handle_event(&event);
            if let Event::Loop(Loop::Render(_)) = event {
                window.draw_2d( & event, | context, graphics |{
                    &game.draw(context, graphics, &mut glyph_cache);
                });
            }
        }
    }
}

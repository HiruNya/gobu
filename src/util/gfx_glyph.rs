//! A trait that implements the draw_2d_with_text function onto PistonWindow.
//!
//! This is used only if you want to use the ``gfx_glyph`` crate. This trait should
//! be imported for the corresponding function implemented for PistonWindow to also
//! be imported.
//!
//! E.g.
//! ```rust, no_run
//! use svnf::util::GfxGlyph;
//! // ...
//! window.draw_2d_with_text(&event, &mut brush, ||{
//!     &game.draw(c, g);
//! });
//! ```
//!
//! This would draw the text over all the things draw in the closure.
//! This, however, might cause problems if things are overlapping.

use piston_window::{
    PistonWindow,
    G2d,
    GfxFactory,
    GenericEvent,
    Context,
    OpenGLWindow,
};
use gfx_glyph::GlyphBrush;
use gfx_device_gl::Resources;

pub trait GfxGlyph {
    fn draw_2d_with_text<E, F, U>(&mut self, e: &E, brush: &mut GlyphBrush<Resources, GfxFactory>, f: F)
        -> Result<Option<U>, String> where
        E: GenericEvent,
        F: FnOnce(Context, &mut G2d) -> U;
}

impl GfxGlyph for PistonWindow {
    // Mostly copied from piston_window however I added support for Gfx_glyph
    fn draw_2d_with_text<E, F, U>(&mut self, e: &E, brush: &mut GlyphBrush<Resources, GfxFactory>, f: F)
        -> Result<Option<U>, String> where
        E: GenericEvent,
        F: FnOnce(Context, &mut G2d) -> U
    {
        if let Some(args) = e.render_args() {
            self.window.make_current();
            let res = self.g2d.draw(
                &mut self.encoder,
                &self.output_color,
                &self.output_stencil,
                args.viewport(),
                f
            );
            brush.draw_queued(
                &mut self.encoder,
                &self.output_color,
                &self.output_stencil
            )?;
            self.encoder.flush(&mut self.device);
            Ok(Some(res))
        } else {
            Ok(None)
        }
    }
}
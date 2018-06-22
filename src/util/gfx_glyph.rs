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
//
//trait GfxGlyph2 {
//    fn draw_with_text<C, F, U>(
//        &mut self,
//        encoder: &mut gfx::Encoder<R, C>,
//        output_color: &gfx::handle::RenderTargetView<R, Srgba8>,
//        output_stencil: &gfx::handle::DepthStencilView<R, DepthStencil>,
//        viewport: Viewport,
//        f: F,
//        brush: &mut GlyphBrush<Resources, GfxFactory>
//    ) -> U
//        where C: gfx::CommandBuffer<R>,
//              F: FnOnce(Context, &mut GfxGraphics<R, C>, &mut GlyphBrush<Resources, GfxFactory>) -> U;
//}
//
//impl<R: gfx::Resources> GfxGlyph2 for Gfx2d<R> {
//    fn draw_with_text<C, F, U>(
//        &mut self,
//        encoder: &mut gfx::Encoder<R, C>,
//        output_color: &gfx::handle::RenderTargetView<R, Srgba8>,
//        output_stencil: &gfx::handle::DepthStencilView<R, DepthStencil>,
//        viewport: Viewport,
//        f: F,
//        brush: &mut GlyphBrush<Resources, GfxFactory>
//    ) -> U
//        where C: gfx::CommandBuffer<R>,
//              F: FnOnce(Context, &mut GfxGraphics<R, C>, &mut GlyphBrush<Resources, GfxFactory>) -> U
//    {
//        let ref mut g = GfxGraphics::new(
//            encoder,
//            output_color,
//            output_stencil,
//            self
//        );
//        let c = Context::new_viewport(viewport);
//        let res = f(c, g, brush);
//        if g.g2d.colored_offset > 0 {
//            g.flush_colored();
//        }
//        res
//    }
//}
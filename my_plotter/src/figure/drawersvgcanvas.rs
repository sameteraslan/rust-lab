use super::{canvas::Canvas, drawer::Drawer, svgcanvas::SvgCanvas};

impl Drawer for SvgCanvas {
    fn draw(&mut self, canvas: &mut Canvas) {
        // Optional: You can convert your Canvas-based drawing to SVG here if needed.
        // For simplicity, this method can be a no-op for SVGCanvas.
    }

    fn draw_legend(&self, canvas: &mut Canvas) {
        // Optional: Implement legend drawing in SVG
    }

    fn draw_svg(&mut self, svg_canvas: &mut SvgCanvas) {
        // For SVG, this could be a no-op or direct rendering logic
    }
}

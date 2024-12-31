use super::{canvas::Canvas, drawer::Drawer, svgcanvas::SvgCanvas};

impl Drawer for Canvas {
    fn draw(&mut self, canvas: &mut Canvas) {
        // If needed, provide logic to convert one Canvas to another or leave as no-op
    }

    fn draw_legend(&self, canvas: &mut Canvas) {
        // Implement legend drawing for pixel-based canvas
    }

    fn draw_svg(&mut self, svg_canvas: &mut SvgCanvas) {
        // Convert existing pixel-based drawing logic into SVG logic.
        // For example:
        svg_canvas.draw_line(50.0, 50.0, 100.0, 100.0, "red", 2.0);
    }
}

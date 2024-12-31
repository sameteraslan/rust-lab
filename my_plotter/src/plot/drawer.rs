use crate::plot::canvas::Canvas;

use super::svgcanvas::SvgCanvas;

pub trait Drawer {
    fn draw(&mut self, canvas: &mut Canvas);
    fn draw_legend(&self, canvas: &mut Canvas);
    fn draw_svg(&mut self, svg_canvas: &mut SvgCanvas);
}

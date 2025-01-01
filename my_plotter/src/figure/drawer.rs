use ab_glyph::{FontRef, PxScale};
use imageproc::drawing::text_size;

use crate::figure::canvas::Canvas;

use super::{
    axistype::AxisType, figureconfig::FigureConfig, linetype::LineType, svgcanvas::SvgCanvas,
};

pub trait Drawer {
    fn draw(&mut self, canvas: &mut Canvas);
    fn draw_legend(&self, canvas: &mut Canvas);
    fn draw_svg(&mut self, svg_canvas: &mut SvgCanvas);
    fn draw_grid(&self, canvas: &mut Canvas, config: &FigureConfig) {
        canvas.draw_grid(
            &[config.num_grid_horizontal, config.num_grid_vertical],
            config.color_grid,
        );
    }

    fn draw_axis(
        &self,
        canvas: &mut Canvas,
        config: &FigureConfig,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
    ) {
        canvas.draw_line(x1, y1, x2, y2, config.color_axis, LineType::Solid);
    }

    fn draw_label(&self, canvas: &mut Canvas, config: &FigureConfig, x: u32, y: u32, text: &str) {
        let font =
            ab_glyph::FontRef::try_from_slice(include_bytes!("../../resources/fonts/Arial.ttf"))
                .unwrap();
        let scale = ab_glyph::PxScale {
            x: config.font_size_label,
            y: config.font_size_label,
        };

        let (w, h) = text_size(scale, &font, &text);

        canvas.draw_text(
            x.saturating_sub(w / 2),
            y.saturating_sub(h / 2),
            text,
            config.color_axis,
            &font,
            scale,
        );
    }

    fn draw_title(&self, canvas: &mut Canvas, config: &FigureConfig, x: u32, y: u32, text: &str) {
        let font_bytes = std::fs::read(&config.font_title).expect("Failed to read font file");
        let font = FontRef::try_from_slice(&font_bytes).unwrap();
        let scale = PxScale {
            x: config.font_size_title,
            y: config.font_size_title,
        };

        let (w, h) = text_size(scale, &font, &text);

        canvas.draw_text(
            x.saturating_sub(w / 2),
            y.saturating_sub(h / 2),
            text,
            config.color_title,
            &font,
            scale,
        );
    }

    fn draw_axis_value(
        &self,
        canvas: &mut Canvas,
        config: &FigureConfig,
        x: u32,
        y: u32,
        text: &str,
        axis: AxisType,
    ) {
        let font =
            ab_glyph::FontRef::try_from_slice(include_bytes!("../../resources/fonts/Arial.ttf"))
                .unwrap();
        let scale = ab_glyph::PxScale {
            x: config.font_size_axis,
            y: config.font_size_axis,
        };

        let (w, h) = text_size(scale, &font, &text);
        let mut x = x;
        let mut y = y;
        match axis {
            AxisType::AxisX => {
                x = x.saturating_sub(w / 2);
                y = y.saturating_add(h);
            }
            AxisType::AxisY => {
                x = x.saturating_sub(w);
                y = y.saturating_sub(h / 2);
            }
        }

        canvas.draw_text(x, y, text, config.color_axis, &font, scale);
    }
}

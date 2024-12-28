use image::RgbImage;

use crate::{BarChartPlot, CartesianGraphPlot};

use super::{cartesian_graph_plot::CartesianGraphDrawing, drawings::Drawing};

// Unified Renderer trait for all plot types
pub trait Renderer {
    fn render_plot(&mut self, img: &mut RgbImage);
    fn render_legend(&mut self, img: &mut RgbImage);
    fn width(&self) -> u32;
    fn height(&self) -> u32;
    fn title(&self) -> &str;
    fn get_font(&self) -> ab_glyph::FontRef<'static> {
        ab_glyph::FontRef::try_from_slice(include_bytes!("../../resources/fonts/Arial.ttf"))
            .unwrap()
    }
}

// Example implementation of Renderer for CartesianGraphPlot
impl Renderer for CartesianGraphPlot {
    fn render_plot(&mut self, img: &mut RgbImage) {
        Drawing::draw_axes(&mut self.base, img);
        for dataset in &self.base.datasets {
            dataset.render(&self.base, img);
        
            if let Some(mouse_position) = optional_mouse_position {
                if let Some(closest_point) = dataset.find_closest_point(&self.base, mouse_position.0, mouse_position.1) {
                    // Highlight the closest point on the plot
                    println!("Closest point: {:?}", closest_point);
                }
            }
        }
    }

    fn render_legend(&mut self, img: &mut RgbImage) {
        // Drawing::draw_legend(&self.base, img);
    }

    fn width(&self) -> u32 {
        self.base.width
    }

    fn height(&self) -> u32 {
        self.base.height
    }

    fn title(&self) -> &str {
        self.base.title.as_str()
    }


}

// Example implementation of Renderer for BarChartPlot
impl Renderer for BarChartPlot {
    fn render_plot(&mut self, img: &mut RgbImage) {
        Drawing::draw_bars(&self.datasets, &mut self.base, img);
    }

    fn render_legend(&mut self, img: &mut RgbImage) {
        // Drawing::draw_legend(&self.datasets,&mut self.base, img);
    }

    fn width(&self) -> u32 {
        self.base.width
    }

    fn height(&self) -> u32 {
        self.base.height
    }

    fn title(&self) -> &str {
        self.base.title.as_str()
    }
}
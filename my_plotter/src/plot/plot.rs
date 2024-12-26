use crate::plot::styles::LineStyle;

use super::styles::PlotType;

#[derive(Clone)]
pub struct Plot {
    pub width: u32,
    pub height: u32,
    pub title: String,
    pub xlabel: String,
    pub ylabel: String,
    pub datasets: Vec<(Vec<(f64, f64)>, [u8; 3], String, LineStyle)>,
    pub background_color: [u8; 3],
    pub front_color: [u8; 3],
    pub grid_color: [u8; 3],
    pub font_color: [u8; 3],
    pub title_color: [u8; 3],
    pub line_thickness: u32,
    pub x_min: f64,
    pub x_max: f64,
    pub y_min: f64,
    pub y_max: f64,
    pub margin: u32,
    pub font_path: String,
    pub plot_type: PlotType,
    pub background_with_grid: bool,
}

impl Plot {
    pub fn new(plot_type: PlotType) -> Self {
        Self {
            width: 800,
            height: 600,
            title: String::new(),
            xlabel: String::new(),
            ylabel: String::new(),
            datasets: Vec::new(),
            background_color: [255, 255, 255],
            front_color: [0, 0, 0],
            grid_color: [220, 220, 220],
            font_color: [0, 0, 0],
            title_color: [0, 0, 0],
            line_thickness: 1,
            x_min: -100.0,
            x_max: 100.0,
            y_min: -100.0,
            y_max: 100.0,
            margin: 100,
            font_path: String::from("../../resources/fonts/Arial.ttf"),
            plot_type,
            background_with_grid: true,
        }
    }

    // pub fn add_bar_dataset(
    //     mut self,
    //     data: Vec<(f64, f64)>,
    //     bar_color: [u8; 3],
    //     label: &str,
    // ) -> Self {
    //     self.datasets
    //         .push((data, bar_color, label.to_string(), LineStyle::Solid));
    //     self
    // }

    pub fn font_path(mut self, font_path: &str) -> Self {
        self.font_path = font_path.to_string();
        self
    }

    pub fn background_with_grid(mut self, background_with_grid: bool) -> Self {
        self.background_with_grid = background_with_grid;
        self
    }

    // Sets the margin around the plot in pixels
    pub fn plot_type(mut self, plot_type: PlotType) -> Self {
        self.plot_type = plot_type;
        self
    }

    // Sets the margin around the plot in pixels
    pub fn margin(mut self, margin: u32) -> Self {
        self.margin = margin;
        self
    }

    // Sets the maximum y value and adjusts the minimum y value to maintain symmetry
    pub fn y_min(mut self, y_min: f64) -> Self {
        self.y_min = y_min;
        self
    }

    // Sets the maximum y value and adjusts the minimum y value to maintain symmetry
    pub fn y_max(mut self, y_max: f64) -> Self {
        self.y_max = y_max;
        self.y_min = -y_max;
        self
    }

    // Sets the maximum x value and adjusts the minimum x value to maintain symmetry
    pub fn x_max(mut self, x_max: f64) -> Self {
        self.x_max = x_max;
        self.x_min = -x_max;
        self
    }

    // Sets the line thickness for data lines
    pub fn line_thickness(mut self, line_thickness: u32) -> Self {
        self.line_thickness = line_thickness;
        self
    }

    // Sets the color of the front elements (axes, labels, etc.)
    pub fn front_color(mut self, front_color: &[u8; 3]) -> Self {
        self.front_color = *front_color;
        self
    }

    // Sets
    pub fn title_color(mut self, title_color: &[u8; 3]) -> Self {
        self.title_color = *title_color;
        self
    }

    // Sets
    pub fn grid_color(mut self, grid_color: &[u8; 3]) -> Self {
        self.grid_color = *grid_color;
        self
    }

    // Sets the color of the front elements (axes, labels, etc.)
    pub fn font_color(mut self, font_color: &[u8; 3]) -> Self {
        self.font_color = *font_color;
        self
    }

    // Sets the background color of the plot
    pub fn background_color(mut self, background_color: &[u8; 3]) -> Self {
        self.background_color = *background_color;
        self
    }

    // Sets the height of the plot in pixels
    pub fn height(mut self, height: u32) -> Self {
        self.height = height;
        self
    }

    // Sets the width of the plot in pixels
    pub fn width(mut self, width: u32) -> Self {
        self.width = width;
        self
    }

    // Sets the title of the plot
    pub fn title(mut self, title: &str) -> Self {
        self.title = title.to_string();
        self
    }

    // Sets the label for the x-axis
    pub fn xlabel(mut self, label: &str) -> Self {
        self.xlabel = label.to_string();
        self
    }

    // Sets the label for the y-axis
    pub fn ylabel(mut self, label: &str) -> Self {
        self.ylabel = label.to_string();
        self
    }

    // Displays the plot's title and axis labels in the console
    pub fn show(&self) {
        println!("Plot: {}", self.title);
        println!("X-Axis: {}", self.xlabel);
        println!("Y-Axis: {}", self.ylabel);
    }

    pub fn get_font(&self) -> ab_glyph::FontRef<'static> {
        ab_glyph::FontRef::try_from_slice(include_bytes!("../../resources/fonts/Arial.ttf"))
            .unwrap()
    }
}

#[derive(Clone)]
pub struct FigureConfig {
    pub num_axis_ticks: usize,
    pub num_grid_horizontal: usize,
    pub num_grid_vertical: usize,
    pub color_grid: [u8; 3],
    pub color_axis: [u8; 3],
    pub color_background: [u8; 3],
    pub color_title: [u8; 3],
    pub font_label: String,
    pub font_title: String,
    pub font_size_label: f32,
    pub font_size_title: f32,
    pub font_size_legend: f32,
    pub font_size_axis: f32,
}

impl Default for FigureConfig {
    fn default() -> Self {
        Self {
            num_axis_ticks: 10,
            num_grid_horizontal: 10,
            num_grid_vertical: 10,
            color_grid: [200, 200, 200],
            color_axis: [0, 0, 0],
            color_background: [255, 255, 255],
            color_title: [0, 0, 0],
            font_label: "../../resources/fonts/Arial.ttf".to_string(),
            font_title: "../../resources/fonts/Arial.ttf".to_string(),
            font_size_label: 12.0,
            font_size_title: 24.0,
            font_size_legend: 10.0,
            font_size_axis: 10.0,
        }
    }
}

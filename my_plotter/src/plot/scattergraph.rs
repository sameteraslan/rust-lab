use super::{
    canvas::Canvas, scatterdottype::ScatterDotType, scattergraphdataset::ScatterGraphDataset,
};

pub struct ScatterGraph {
    pub title: String,
    pub x_label: String,
    pub y_label: String,
    pub datasets: Vec<ScatterGraphDataset>,
}

impl ScatterGraph {
    pub fn new(title: &str, x_label: &str, y_label: &str) -> Self {
        Self {
            title: title.to_string(),
            x_label: x_label.to_string(),
            y_label: y_label.to_string(),
            datasets: Vec::new(),
        }
    }

    pub fn add_dataset(&mut self, dataset: ScatterGraphDataset) {
        self.datasets.push(dataset);
    }

    pub fn draw_dot(
        &self,
        canvas: &mut Canvas,
        x: i32,
        y: i32,
        dot_type: ScatterDotType,
        color: [u8; 3],
    ) {
        match dot_type {
            ScatterDotType::Circle(radius) => {
                for dy in -(radius as i32)..=radius as i32 {
                    for dx in -(radius as i32)..=radius as i32 {
                        if dx * dx + dy * dy <= (radius * radius) as i32 {
                            canvas.draw_pixel((x + dx) as u32, (y + dy) as u32, color);
                        }
                    }
                }
            }
            ScatterDotType::Square(size) => {
                for dy in -(size as i32) / 2..=(size as i32) / 2 {
                    for dx in -(size as i32) / 2..=(size as i32) / 2 {
                        canvas.draw_pixel((x + dx) as u32, (y + dy) as u32, color);
                    }
                }
            }
            ScatterDotType::Cross(thickness) => {
                for i in -(thickness as i32)..=(thickness as i32) {
                    canvas.draw_pixel((x + i) as u32, y as u32, color); // Horizontal line
                    canvas.draw_pixel(x as u32, (y + i) as u32, color); // Vertical line
                }
            }
            ScatterDotType::Triangle(base) => {
                for dy in 0..=base as i32 {
                    let dx = (base as f64 * (1.0 - dy as f64 / base as f64)) as i32;
                    for x_offset in -dx..=dx {
                        canvas.draw_pixel((x + x_offset) as u32, (y - dy) as u32, color);
                    }
                }
            }
        }
    }
}

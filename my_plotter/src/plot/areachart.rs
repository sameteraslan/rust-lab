use super::{areachartdataset::AreaChartDataset, canvas::Canvas};

pub struct AreaChart {
    pub title: String,
    pub x_label: String,
    pub y_label: String,
    pub datasets: Vec<AreaChartDataset>,
}

impl AreaChart {
    pub fn new(title: &str, x_label: &str, y_label: &str) -> Self {
        Self {
            title: title.to_string(),
            x_label: x_label.to_string(),
            y_label: y_label.to_string(),
            datasets: Vec::new(),
        }
    }

    pub fn add_dataset(&mut self, dataset: AreaChartDataset) {
        self.datasets.push(dataset);
    }

    pub fn draw_area(
        &self,
        canvas: &mut Canvas,
        dataset: &AreaChartDataset,
        origin_x: i32,
        origin_y: i32,
        scale_x: f64,
        scale_y: f64,
    ) {
        let mut points = dataset.points.clone();
        points.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));

        for window in points.windows(2) {
            if let [p1, p2] = window {
                let x1 = origin_x + ((p1.0) * scale_x) as i32;
                let y1 = origin_y - ((p1.1) * scale_y) as i32;
                let x2 = origin_x + ((p2.0) * scale_x) as i32;
                let y2 = origin_y - ((p2.1) * scale_y) as i32;

                // Fill the area under the line
                for x in x1.min(x2)..=x1.max(x2) {
                    let interpolated_y =
                        y1 + ((x - x1) as f64 * (y2 - y1) as f64 / (x2 - x1).abs() as f64) as i32;
                    for y in interpolated_y..=origin_y {
                        canvas.blend_pixel(x as u32, y as u32, dataset.color, dataset.alpha);
                    }
                }
            }
        }
    }
}

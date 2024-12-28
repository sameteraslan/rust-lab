use super::canvas::Canvas;

pub struct PieChart {
    pub title: String,
    pub datasets: Vec<(String, f64, [u8; 3])>, // Label, Value, Color
}

impl PieChart {
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
            datasets: Vec::new(),
        }
    }

    pub fn add_slice(&mut self, label: &str, value: f64, color: [u8; 3]) {
        self.datasets.push((label.to_string(), value, color));
    }

    pub fn draw_slice(
        &self,
        canvas: &mut Canvas,
        center_x: i32,
        center_y: i32,
        radius: i32,
        start_angle: f64,
        end_angle: f64,
        color: [u8; 3],
    ) {
        let start_angle_rad = start_angle;
        let end_angle_rad = end_angle;

        for y in -radius..=radius {
            for x in -radius..=radius {
                // Check if the point is within the circle
                let distance = (x * x + y * y) as f64;
                if distance <= (radius * radius) as f64 {
                    // Calculate the angle of the point
                    let angle = (y as f64).atan2(x as f64);
                    let normalized_angle = if angle < 0.0 {
                        angle + 2.0 * std::f64::consts::PI
                    } else {
                        angle
                    };

                    // Check if the angle is within the slice range
                    if normalized_angle >= start_angle_rad && normalized_angle < end_angle_rad {
                        canvas.draw_pixel((center_x + x) as u32, (center_y - y) as u32, color);
                    }
                }
            }
        }
    }
}

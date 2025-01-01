pub struct BarDataset {
    pub label: String,
    pub color: [u8; 3],
    pub data: Vec<(f64, f64)>,
}

impl BarDataset {
    pub fn new(label: &str, color: [u8; 3]) -> Self {
        Self {
            data: Vec::new(),
            label: label.to_string(),
            color,
        }
    }

    pub fn add_data(&mut self, x: f64, y: f64) {
        self.data.push((x, y));
    }
}

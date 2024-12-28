pub struct AreaChartDataset {
    pub points: Vec<(f64, f64)>,
    pub color: [u8; 3],
    pub label: String,
    pub alpha: f64,
}

impl AreaChartDataset {
    pub fn new(color: [u8; 3], label: &str, alpha: f64) -> Self {
        Self {
            points: Vec::new(),
            color,
            label: label.to_string(),
            alpha,
        }
    }
}

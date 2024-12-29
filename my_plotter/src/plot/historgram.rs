pub struct Histogram {
    pub title: String,
    pub x_label: String,
    pub y_label: String,
    pub bins: usize,
    pub data: Vec<f64>,
    pub color: [u8; 3], // RGB color
}

impl Histogram {
    pub fn new(title: &str, x_label: &str, y_label: &str, bins: usize, color: [u8; 3]) -> Self {
        Self {
            title: title.to_string(),
            x_label: x_label.to_string(),
            y_label: y_label.to_string(),
            bins,
            data: Vec::new(),
            color,
        }
    }

    pub fn add_data(&mut self, values: Vec<f64>) {
        self.data.extend(values);
    }

    pub fn calculate_bins(&self) -> Vec<(f64, f64)> {
        let min = self.data.iter().cloned().fold(f64::INFINITY, f64::min);
        let max = self.data.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let bin_width = (max - min) / self.bins as f64;

        let mut bins = vec![0.0; self.bins];
        for &value in &self.data {
            let bin_index = ((value - min) / bin_width).floor() as usize;
            if bin_index < self.bins {
                bins[bin_index] += 1.0;
            }
        }

        bins.into_iter()
            .enumerate()
            .map(|(i, freq)| (min + i as f64 * bin_width, freq))
            .collect()
    }
}

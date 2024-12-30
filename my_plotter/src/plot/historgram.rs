pub struct Histogram {
    pub title: String,
    pub x_label: String,
    pub y_label: String,
    pub bins: usize,
    pub data: Vec<f64>,
    pub color: [u8; 3],       // RGB color
    pub min: f64,             // Cached minimum value
    pub max: f64,             // Cached maximum value
    pub bin_counts: Vec<f64>, // Cached bin frequencies
    pub bin_width: f64,       // Cached bin width
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
            min: f64::INFINITY,
            max: f64::NEG_INFINITY,
            bin_counts: vec![0.0; bins],
            bin_width: 0.0,
        }
    }

    pub fn add_data_vec(&mut self, values: Vec<f64>) {
        for value in values {
            self.add_data(value);
        }
    }

    pub fn add_data(&mut self, value: f64) {
        self.data.push(value);

        // Update min and max
        if value < self.min {
            self.min = value;
        }
        if value > self.max {
            self.max = value;
        }

        // Recalculate bin width and update bin counts
        self.bin_width = (self.max - self.min) / self.bins as f64;
        if self.bin_width > 0.0 {
            let bin_index = ((value - self.min) / self.bin_width).floor() as usize;
            if bin_index < self.bins {
                self.bin_counts[bin_index] += 1.0;
            }
        }
    }

    pub fn calculate_bins(&self) -> Vec<(f64, f64)> {
        self.bin_counts
            .iter()
            .enumerate()
            .map(|(i, &freq)| (self.min + i as f64 * self.bin_width, freq))
            .collect()
    }
}

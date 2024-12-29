use crate::plot::cartesiangraphdataset::CartesianDataset;

pub struct CartesianGraph {
    pub datasets: Vec<CartesianDataset>,
    pub title: String,
    pub x_label: String,
    pub y_label: String,
    pub x_min: f64, // Minimum x-value
    pub x_max: f64, // Maximum x-value
}

impl CartesianGraph {
    pub fn new(title: &str, x_label: &str, y_label: &str) -> Self {
        Self {
            datasets: Vec::new(),
            title: title.to_string(),
            x_label: x_label.to_string(),
            y_label: y_label.to_string(),
            x_min: f64::INFINITY,     // Initialize to max range
            x_max: f64::NEG_INFINITY, // Initialize to min range
        }
    }

    pub fn add_dataset(&mut self, dataset: CartesianDataset) {
        self.datasets.push(dataset);
        self.update_range();
    }

    pub fn update_range(&mut self) {
        for dataset in &self.datasets {
            for &(x, _) in &dataset.points {
                if x < self.x_min {
                    self.x_min = x;
                }
                if x > self.x_max {
                    self.x_max = x;
                }
            }
        }
    }
}

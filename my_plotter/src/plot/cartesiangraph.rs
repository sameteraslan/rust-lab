use crate::plot::cartesiangraphdataset::CartesianDataset;

pub struct CartesianGraph {
    pub datasets: Vec<CartesianDataset>,
    pub title: String,
    pub x_label: String,
    pub y_label: String,
}

impl CartesianGraph {
    pub fn new(title: &str, x_label: &str, y_label: &str) -> Self {
        Self {
            datasets: Vec::new(),
            title: title.to_string(),
            x_label: x_label.to_string(),
            y_label: y_label.to_string(),
        }
    }

    pub fn add_dataset(&mut self, dataset: CartesianDataset) {
        self.datasets.push(dataset);
    }
}

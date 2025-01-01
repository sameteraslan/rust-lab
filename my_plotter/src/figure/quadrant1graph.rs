use super::cartesiangraphdataset::CartesianDataset;

pub struct Quadrant1Graph {
    pub datasets: Vec<CartesianDataset>,
    pub title: String,
    pub x_label: String,
    pub y_label: String,
}

impl Quadrant1Graph {
    pub fn new(title: &str, x_label: &str, y_label: &str) -> Self {
        Self {
            datasets: Vec::new(),
            title: title.to_string(),
            x_label: x_label.to_string(),
            y_label: y_label.to_string(),
        }
    }

    pub fn add_dataset(&mut self, dataset: CartesianDataset) {
        let filtered_dataset = CartesianDataset {
            points: dataset
                .points
                .into_iter()
                .filter(|&(x, y)| x >= 0.0 && y >= 0.0)
                .collect(),
            color: dataset.color,
            label: dataset.label.clone(),
            line_type: dataset.line_type,
        };
        self.datasets.push(filtered_dataset);
    }
}

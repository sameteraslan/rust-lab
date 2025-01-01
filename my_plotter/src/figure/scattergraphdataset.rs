use super::scatterdottype::ScatterDotType;

pub struct ScatterGraphDataset {
    pub points: Vec<(f64, f64)>,
    pub color: [u8; 3],
    pub label: String,
    pub dot_type: ScatterDotType, // Add dot type
}

impl ScatterGraphDataset {
    pub fn new(color: [u8; 3], label: &str, dot_type: ScatterDotType) -> Self {
        Self {
            points: Vec::new(),
            color,
            label: label.to_string(),
            dot_type,
        }
    }
}

use crate::figure::linetype::LineType;

pub struct CartesianDataset {
    pub points: Vec<(f64, f64)>,
    pub color: [u8; 3],
    pub label: String,
    pub line_type: LineType, // Add line type
}

impl CartesianDataset {
    pub fn new(color: [u8; 3], label: &str, line_type: LineType) -> Self {
        Self {
            points: Vec::new(),
            color,
            label: label.to_string(),
            line_type,
        }
    }
}

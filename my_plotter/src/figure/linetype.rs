#[derive(Clone)]
pub enum LineType {
    Solid,
    Dashed(u32), // Dash length
    Dotted(u32), // Dot spacing
}

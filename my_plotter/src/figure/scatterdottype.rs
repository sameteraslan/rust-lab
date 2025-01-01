#[derive(Clone)]
pub enum ScatterDotType {
    Circle(u32),   // Radius
    Square(u32),   // Side length
    Cross(u32),    // Line thickness
    Triangle(u32), // Base size
}

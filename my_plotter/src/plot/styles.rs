#[derive(Clone)]
pub enum LineStyle {
    Solid,
    Dotted,
    Dashed,
    DashDot,
}

#[derive(Clone, PartialEq)]
pub enum PlotType {
    CartesianGraph,
    BarChart,
    PieChart, // Future extension
}

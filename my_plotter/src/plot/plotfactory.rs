use crate::plot::barchart::BarChart;
use crate::plot::cartesiangraph::CartesianGraph;
use crate::plot::drawer::Drawer;

use crate::plot::piechart::PieChart;

use super::scattergraph::ScatterGraph;

pub enum PlotType {
    BarChart,
    CartesianGraph,
    PieChart,
    ScatterGraph,
}

pub struct PlotFactory;

impl PlotFactory {
    pub fn create_plot(plot_type: PlotType) -> Box<dyn Drawer> {
        match plot_type {
            PlotType::BarChart => Box::new(BarChart::new(
                "Bar Chart",
                "X Axis",
                "Y Axis",
                super::orientation::Orientation::Horizontal,
            )),
            PlotType::CartesianGraph => {
                Box::new(CartesianGraph::new("Cartesian Graph", "X Axis", "Y Axis"))
            }
            PlotType::PieChart => Box::new(PieChart::new("Pie Chart")),
            PlotType::ScatterGraph => {
                Box::new(ScatterGraph::new("Scatter Graph", "X Axis", "Y Axis"))
            }
        }
    }
}

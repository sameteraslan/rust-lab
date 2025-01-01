use crate::figure::barchart::BarChart;
use crate::figure::cartesiangraph::CartesianGraph;
use crate::figure::drawer::Drawer;

use crate::figure::piechart::PieChart;

use super::areachart::AreaChart;
use super::canvas::Canvas;
use super::figureconfig::FigureConfig;
use super::historgram::Histogram;
use super::scattergraph::ScatterGraph;
use super::svgcanvas::SvgCanvas;

pub enum PlotType {
    BarChart,
    CartesianGraph,
    PieChart,
    ScatterGraph,
    AreaChart,
    Histogram,
}

pub enum OutputFormat {
    PixelCanvas,
    Svg,
}

pub struct FigureFactory;

impl FigureFactory {
    pub fn create_canvas(format: OutputFormat, width: u32, height: u32) -> Box<dyn Drawer> {
        match format {
            OutputFormat::PixelCanvas => Box::new(Canvas::new(width, height, [255, 255, 255], 10)),
            OutputFormat::Svg => Box::new(SvgCanvas::new(width, height, "white", 10)),
        }
    }

    pub fn create_plot(plot_type: PlotType) -> Box<dyn Drawer> {
        match plot_type {
            PlotType::BarChart => Box::new(BarChart::new(
                "Bar Chart",
                "X Axis",
                "Y Axis",
                super::orientation::Orientation::Horizontal,
                FigureConfig::default(),
            )),
            PlotType::CartesianGraph => Box::new(CartesianGraph::new(
                "Cartesian Graph",
                "X Axis",
                "Y Axis",
                &FigureConfig::default(),
            )),
            PlotType::PieChart => Box::new(PieChart::new("Pie Chart")),
            PlotType::ScatterGraph => {
                Box::new(ScatterGraph::new("Scatter Graph", "X Axis", "Y Axis"))
            }
            PlotType::AreaChart => Box::new(AreaChart::new(
                "Area Chart",
                "X Axis",
                "Y Axis",
                FigureConfig::default(),
            )),
            PlotType::Histogram => Box::new(Histogram::new(
                "Histogram",
                "Bins",
                "Frequency",
                0,
                [0, 0, 255],
            )),
        }
    }
}

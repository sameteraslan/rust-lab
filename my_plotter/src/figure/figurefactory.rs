use crate::figure::barchart::BarChart;
use crate::figure::cartesiangraph::CartesianGraph;
use crate::figure::drawer::Drawer;

use crate::figure::piechart::PieChart;

use super::areachart::AreaChart;
use super::pixelcanvas::Canvas;
use super::figureconfig::FigureConfig;
use super::histogram::Histogram;
use super::scattergraph::ScatterGraph;
use super::svgcanvas::SvgCanvas;

/// Represents the types of plots that can be created.
pub enum PlotType {
    /// A bar chart, which uses rectangular bars to represent data.
    BarChart,
    /// A Cartesian graph, which plots points and lines on a coordinate grid.
    CartesianGraph,
    /// A pie chart, which represents data as slices of a circle.
    PieChart,
    /// A scatter graph, which plots individual data points.
    ScatterGraph,
    /// An area chart, which represents data with filled areas under lines.
    AreaChart,
    /// A histogram, which shows the frequency distribution of data.
    Histogram,
}

/// Represents the output format for the generated plots.
pub enum OutputFormat {
    /// Output as a `PixelCanvas`, which is a raster-based rendering format.
    PixelCanvas,
    /// Output as an `Svg`, which is a scalable vector graphics format.
    Svg,
}

/// A factory for creating various types of plots.
///
/// This factory simplifies the creation of plot instances by abstracting the
/// initialization process and providing default configurations.
pub struct FigureFactory;

impl FigureFactory {
    /// Creates a plot of the specified type with default settings.
    ///
    /// # Parameters
    /// - `plot_type`: The type of plot to create (`PlotType`).
    ///
    /// # Returns
    /// A boxed `Drawer` object representing the created plot.
    ///
    /// # Example
    /// ```rust
    /// use crate::figure::figurefactory::{FigureFactory, PlotType};
    ///
    /// let bar_chart = FigureFactory::create_plot(PlotType::BarChart);
    /// ```
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
            PlotType::PieChart => Box::new(PieChart::new("Pie Chart", FigureConfig::default())),
            PlotType::ScatterGraph => Box::new(ScatterGraph::new(
                "Scatter Graph",
                "X Axis",
                "Y Axis",
                FigureConfig::default(),
            )),
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
                FigureConfig::default(),
            )),
        }
    }
}

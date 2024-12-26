use crate::plot::bar_chart_plot::BarChartPlot;
use crate::plot::cartesian_graph_plot::CartesianGraphPlot;
use crate::Plot;

pub trait DisplayablePlot {
    fn as_base_plot(&self) -> &Plot;
    fn as_mut_base_plot(&mut self) -> &mut Plot;
}

impl DisplayablePlot for CartesianGraphPlot {
    fn as_base_plot(&self) -> &Plot {
        &self.base
    }

    fn as_mut_base_plot(&mut self) -> &mut Plot {
        &mut self.base
    }
}

impl DisplayablePlot for BarChartPlot {
    fn as_base_plot(&self) -> &Plot {
        &self.base
    }

    fn as_mut_base_plot(&mut self) -> &mut Plot {
        &mut self.base
    }
}

impl DisplayablePlot for Plot {
    fn as_base_plot(&self) -> &Plot {
        self
    }

    fn as_mut_base_plot(&mut self) -> &mut Plot {
        self
    }
}

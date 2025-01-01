use crate::figure::bardataset::BarDataset;
use crate::figure::cartesiangraphdataset::CartesianDataset;

use super::areachartdataset::AreaChartDataset;
use super::scattergraphdataset::ScatterGraphDataset;

pub trait Dataset {
    fn get_points(&self) -> Vec<(f64, f64)>;
    fn add_point(&mut self, point: (f64, f64));
}

impl Dataset for BarDataset {
    fn get_points(&self) -> Vec<(f64, f64)> {
        self.data.clone()
    }

    fn add_point(&mut self, point: (f64, f64)) {
        self.data.push(point);
    }
}

impl Dataset for CartesianDataset {
    fn get_points(&self) -> Vec<(f64, f64)> {
        self.points.clone()
    }

    fn add_point(&mut self, point: (f64, f64)) {
        self.points.push(point);
    }
}

impl Dataset for ScatterGraphDataset {
    fn get_points(&self) -> Vec<(f64, f64)> {
        self.points.clone()
    }

    fn add_point(&mut self, point: (f64, f64)) {
        self.points.push(point);
    }
}

impl Dataset for AreaChartDataset {
    fn get_points(&self) -> Vec<(f64, f64)> {
        self.points.clone()
    }

    fn add_point(&mut self, point: (f64, f64)) {
        self.points.push(point);
    }
}

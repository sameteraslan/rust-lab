use image::{Rgb, RgbImage};
use imageproc::drawing::draw_text_mut;

use super::drawings::Drawing;
use crate::plot::styles::{LineStyle, PlotType};
use crate::Plot;
use ab_glyph::PxScale;
use std::ops::{Deref, DerefMut};

pub struct BarChartPlot {
    pub base: Plot,
}

impl Deref for BarChartPlot {
    type Target = Plot;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl DerefMut for BarChartPlot {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl BarChartPlot {
    pub fn new() -> Self {
        Self {
            base: Plot::new(PlotType::BarChart),
        }
    }

    pub fn add_bar_dataset(&mut self, data: Vec<(f64, f64)>, bar_color: [u8; 3], label: &str) {
        self.base
            .datasets
            .push((data, bar_color, label.to_string(), LineStyle::Solid));
    }
}

pub trait BarChartMethods {
    fn add_bar_dataset(&mut self, data: Vec<(f64, f64)>, bar_color: [u8; 3], label: &str);
}

impl BarChartMethods for Plot {
    fn add_bar_dataset(&mut self, data: Vec<(f64, f64)>, bar_color: [u8; 3], label: &str) {
        if let PlotType::BarChart = self.plot_type {
            self.datasets
                .push((data, bar_color, label.to_string(), LineStyle::Solid));
        } else {
            panic!("This method is only available for BarChart plot type!");
        }
    }
}

pub trait BarChartDrawing {
    fn draw_bars(plot: &mut Plot, img: &mut RgbImage);
    fn draw_text(
        font: &ab_glyph::FontRef,
        text: &str,
        x: i32,
        y: i32,
        color: [u8; 3],
        img: &mut RgbImage,
    );
}

impl BarChartDrawing for Drawing {
    fn draw_bars(plot: &mut Plot, img: &mut RgbImage) {
        // Extract necessary values to avoid repeated borrowing
        let total_pairs = plot.datasets.len();
        let available_height = plot.height as f64 - 2.0 * plot.margin as f64;
        let bar_gap = available_height / (total_pairs as f64 + 1.0);
        let bar_height = bar_gap * 0.6; // Bars take 60% of the gap
        let total_width = plot.width as f64 - 2.0 * plot.margin as f64; // Available width for bars
        let margin = plot.margin;
        let background_color = plot.background_color;

        for (index, (data, _, _, _)) in plot.datasets.iter().enumerate() {
            for &(buy, sell) in data {
                let y_center = margin as f64 + (index as f64 + 1.0) * bar_gap;
                let y_start = (y_center - bar_height / 2.0) as i32;
                let y_end = (y_center + bar_height / 2.0) as i32;

                let total_amount = buy + sell;
                let buy_width = (buy / total_amount) * total_width;
                let sell_width = (sell / total_amount) * total_width;

                let buy_start = margin as i32;
                let buy_end = (buy_start as f64 + buy_width) as i32;
                let sell_start = buy_end;
                let sell_end = (sell_start as f64 + sell_width) as i32;

                // Draw the buy bar (green)
                for y in y_start..y_end {
                    for x in buy_start..buy_end {
                        if x >= 0 && x < plot.width as i32 && y >= 0 && y < plot.height as i32 {
                            img.put_pixel(x as u32, y as u32, Rgb([0, 255, 0]));
                            // Green
                        }
                    }
                }

                // Draw the sell bar (red)
                for y in y_start..y_end {
                    for x in sell_start..sell_end {
                        if x >= 0 && x < plot.width as i32 && y >= 0 && y < plot.height as i32 {
                            img.put_pixel(x as u32, y as u32, Rgb([255, 0, 0]));
                            // Red
                        }
                    }
                }

                // Add percentage text in the middle of each segment
                let font = plot.get_font(); // Get the font for text rendering
                let buy_percent = (buy / total_amount * 100.0).round() as u32;
                let sell_percent = (sell / total_amount * 100.0).round() as u32;

                // Draw percentage text
                let buy_text = format!("{}%", buy_percent);
                let buy_text_x = buy_start + (buy_end - buy_start) / 2;
                Drawing::draw_text(
                    &font,
                    &buy_text,
                    buy_text_x,
                    y_center as i32 - 8,
                    background_color,
                    img,
                );

                let sell_text = format!("{}%", sell_percent);
                let sell_text_x = sell_start + (sell_end - sell_start) / 2;
                Drawing::draw_text(
                    &font,
                    &sell_text,
                    sell_text_x,
                    y_center as i32 - 8,
                    background_color,
                    img,
                );
            }
        }
    }

    fn draw_text(
        font: &ab_glyph::FontRef,
        text: &str,
        x: i32,
        y: i32,
        color: [u8; 3],
        img: &mut RgbImage,
    ) {
        let scale = PxScale { x: 12.0, y: 12.0 }; // Font size

        draw_text_mut(img, Rgb(color), x, y, scale, font, text);
    }
}

use image::{Rgb, RgbImage};

use crate::plot::styles::{LineStyle, PlotType};
use crate::Plot;

use super::drawings::Drawing;
use super::plot;
use std::ops::{Deref, DerefMut};

pub struct CartesianGraphPlot {
    pub base: Plot,
}

impl Deref for CartesianGraphPlot {
    type Target = Plot;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl DerefMut for CartesianGraphPlot {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl CartesianGraphPlot {
    pub fn new() -> Self {
        Self {
            base: Plot::new(PlotType::CartesianGraph),
        }
    }

    pub fn add_dataset(
        &mut self,
        data: Vec<(f64, f64)>,
        line_color: [u8; 3],
        label: &str,
        line_style: LineStyle,
    ) {
        self.base
            .datasets
            .push((data, line_color, label.to_string(), line_style));
    }
}

pub trait CartesianGraphMethods {
    fn add_dataset(
        &mut self,
        data: Vec<(f64, f64)>,
        line_color: [u8; 3],
        label: &str,
        line_style: LineStyle,
    );
}

impl CartesianGraphMethods for Plot {
    fn add_dataset(
        &mut self,
        data: Vec<(f64, f64)>,
        line_color: [u8; 3],
        label: &str,
        line_style: LineStyle,
    ) {
        if let PlotType::CartesianGraph = self.plot_type {
            self.datasets
                .push((data, line_color, label.to_string(), line_style));
        } else {
            panic!("This method is only available for CartesianGraph plot type!");
        }
    }
}

pub trait CartesianGraphDrawing {
    fn draw_axes(plot: &mut Plot, img: &mut RgbImage);
    fn draw_lines_with_thickness(plot: &mut Plot, img: &mut RgbImage);
    fn draw_dotted_line(
        plot: &mut plot::Plot,
        img: &mut RgbImage,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        color: [u8; 3],
    );
    fn draw_dashed_line(
        plot: &mut plot::Plot,
        img: &mut RgbImage,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        color: [u8; 3],
    );
    fn draw_dash_dot_line(
        plot: &mut plot::Plot,
        img: &mut RgbImage,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        color: [u8; 3],
    );

    fn draw_thick_line(
        plot: &mut plot::Plot,
        img: &mut RgbImage,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        color: [u8; 3],
        thickness: u32,
    );
}

impl CartesianGraphDrawing for Drawing {
    /// Draws the axes on the provided image.
    ///
    /// # Arguments
    ///
    /// * `img` - A mutable reference to the image where the axes will be drawn.
    ///
    /// This function draws the X and Y axes on the image, including the axis lines,
    /// tick marks, and labels.
    fn draw_axes(plot: &mut Plot, img: &mut RgbImage) {
        // Calculate the center of the axes with respect to the margin
        let center_x = (plot.margin + (plot.width - 2 * plot.margin) / 2) as i32;
        let center_y = (plot.margin + (plot.height - 2 * plot.margin) / 2) as i32;

        // Draw the X-axis (horizontal line at center_y)
        if center_y >= 0 && center_y < plot.height as i32 {
            for x in plot.margin..(plot.width - plot.margin) {
                img.put_pixel(x, center_y as u32, Rgb(plot.front_color));
            }
        }

        // Draw the Y-axis (vertical line at center_x)
        if center_x >= 0 && center_x < plot.width as i32 {
            for y in plot.margin..(plot.height - plot.margin) {
                img.put_pixel(center_x as u32, y, Rgb(plot.front_color));
            }
        }
    }

    /// Draws lines with thickness on the provided image.
    ///
    /// # Arguments
    ///
    /// * `img` - A mutable reference to the image where the lines will be drawn.
    ///
    /// This function iterates over the datasets in the plot and draws lines
    /// connecting the points with the specified thickness and style.
    fn draw_lines_with_thickness(plot: &mut plot::Plot, img: &mut RgbImage) {
        for (data, color, _, line_style) in plot.datasets.clone() {
            if data.len() < 2 {
                continue; // Skip if there are fewer than 2 points
            }

            let scale_x =
                (plot.width as f64 - 2.0 * plot.margin as f64) / (plot.x_max - plot.x_min);
            let scale_y =
                (plot.height as f64 - 2.0 * plot.margin as f64) / (plot.y_max - plot.y_min);

            let mut previous_point: Option<(i32, i32)> = None;

            for &(x, y) in &data {
                // Clip the point to the bounds
                if x < plot.x_min || x > plot.x_max || y < plot.y_min || y > plot.y_max {
                    previous_point = None; // Skip rendering outside bounds
                    continue;
                }

                let x_pixel = ((x - plot.x_min) * scale_x + plot.margin as f64) as i32;
                let y_pixel =
                    (plot.height as f64 - plot.margin as f64 - (y - plot.y_min) * scale_y) as i32;

                match line_style {
                    LineStyle::Solid => {
                        if let Some((prev_x, prev_y)) = previous_point {
                            Drawing::draw_thick_line(
                                plot,
                                img,
                                prev_x,
                                prev_y,
                                x_pixel,
                                y_pixel,
                                color,
                                plot.line_thickness,
                            );
                        }
                    }
                    LineStyle::Dotted => {
                        if let Some((prev_x, prev_y)) = previous_point {
                            Drawing::draw_dotted_line(
                                plot, img, prev_x, prev_y, x_pixel, y_pixel, color,
                            );
                        }
                    }
                    LineStyle::Dashed => {
                        if let Some((prev_x, prev_y)) = previous_point {
                            Drawing::draw_dashed_line(
                                plot, img, prev_x, prev_y, x_pixel, y_pixel, color,
                            );
                        }
                    }
                    LineStyle::DashDot => {
                        if let Some((prev_x, prev_y)) = previous_point {
                            Drawing::draw_dash_dot_line(
                                plot, img, prev_x, prev_y, x_pixel, y_pixel, color,
                            );
                        }
                    }
                }

                previous_point = Some((x_pixel, y_pixel));
            }
        }
    }

    /// Draws a thick line on the provided image.
    ///
    /// # Arguments
    ///
    /// * `img` - A mutable reference to the image where the line will be drawn.
    /// * `x1` - The starting x-coordinate of the line.
    /// * `y1` - The starting y-coordinate of the line.
    /// * `x2` - The ending x-coordinate of the line.
    /// * `y2` - The ending y-coordinate of the line.
    /// * `color` - The color of the line.
    /// * `thickness` - The thickness of the line.
    fn draw_thick_line(
        plot: &mut plot::Plot,
        img: &mut RgbImage,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        color: [u8; 3],
        thickness: u32,
    ) {
        let dx = (x2 - x1).abs();
        let dy = -(y2 - y1).abs();
        let sx = if x1 < x2 { 1 } else { -1 };
        let sy = if y1 < y2 { 1 } else { -1 };
        let mut err = dx + dy;

        let mut x = x1;
        let mut y = y1;

        while x != x2 || y != y2 {
            for tx in -(thickness as i32 / 2)..=(thickness as i32 / 2) {
                for ty in -(thickness as i32 / 2)..=(thickness as i32 / 2) {
                    let px = x + tx;
                    let py = y + ty;
                    if px >= 0 && px < plot.width as i32 && py >= 0 && py < plot.height as i32 {
                        img.put_pixel(px as u32, py as u32, Rgb(color));
                    }
                }
            }

            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x += sx;
            }
            if e2 <= dx {
                err += dx;
                y += sy;
            }
        }
    }

    /// Draws a dotted line on the provided image.
    ///
    /// # Arguments
    ///
    /// * `img` - A mutable reference to the image where the line will be drawn.
    /// * `x1` - The starting x-coordinate of the line.
    /// * `y1` - The starting y-coordinate of the line.
    /// * `x2` - The ending x-coordinate of the line.
    /// * `y2` - The ending y-coordinate of the line.
    /// * `color` - The color of the line.
    fn draw_dotted_line(
        plot: &mut plot::Plot,
        img: &mut RgbImage,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        color: [u8; 3],
    ) {
        let dx = (x2 - x1).abs();
        let dy = -(y2 - y1).abs();
        let sx = if x1 < x2 { 1 } else { -1 };
        let sy = if y1 < y2 { 1 } else { -1 };
        let mut err = dx + dy;

        let mut x = x1;
        let mut y = y1;
        let mut draw_dot = true; // Toggle to draw dots intermittently

        while x != x2 || y != y2 {
            if draw_dot {
                for tx in -(plot.line_thickness as i32 / 2)..=(plot.line_thickness as i32 / 2) {
                    for ty in -(plot.line_thickness as i32 / 2)..=(plot.line_thickness as i32 / 2) {
                        let px = x + tx;
                        let py = y + ty;
                        if px >= 0 && px < plot.width as i32 && py >= 0 && py < plot.height as i32 {
                            img.put_pixel(px as u32, py as u32, Rgb(color));
                        }
                    }
                }
            }

            draw_dot = !draw_dot; // Toggle dot drawing

            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x += sx;
            }
            if e2 <= dx {
                err += dx;
                y += sy;
            }
        }
    }

    /// Draws a dashed line on the provided image.
    ///
    /// # Arguments
    ///
    /// * `img` - A mutable reference to the image where the line will be drawn.
    /// * `x1` - The starting x-coordinate of the line.
    /// * `y1` - The starting y-coordinate of the line.
    /// * `x2` - The ending x-coordinate of the line.
    /// * `y2` - The ending y-coordinate of the line.
    /// * `color` - The color of the line.
    fn draw_dashed_line(
        plot: &mut plot::Plot,
        img: &mut RgbImage,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        color: [u8; 3],
    ) {
        let dx = (x2 - x1).abs();
        let dy = -(y2 - y1).abs();
        let sx = if x1 < x2 { 1 } else { -1 };
        let sy = if y1 < y2 { 1 } else { -1 };
        let mut err = dx + dy;

        let mut x = x1;
        let mut y = y1;
        let segment_length = 10; // Length of each dash
        let gap_length = 5; // Length of each gap
        let mut counter = 0;
        let mut draw_dash = true; // Start with drawing a dash

        while x != x2 || y != y2 {
            if draw_dash {
                for tx in -(plot.line_thickness as i32 / 2)..=(plot.line_thickness as i32 / 2) {
                    for ty in -(plot.line_thickness as i32 / 2)..=(plot.line_thickness as i32 / 2) {
                        let px = x + tx;
                        let py = y + ty;
                        if px >= 0 && px < plot.width as i32 && py >= 0 && py < plot.height as i32 {
                            img.put_pixel(px as u32, py as u32, Rgb(color));
                        }
                    }
                }
            }

            counter += 1;

            if draw_dash && counter == segment_length {
                counter = 0;
                draw_dash = false; // Switch to gap
            } else if !draw_dash && counter == gap_length {
                counter = 0;
                draw_dash = true; // Switch to dash
            }

            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x += sx;
            }
            if e2 <= dx {
                err += dx;
                y += sy;
            }
        }
    }

    /// Draws a dash-dot line on the provided image.
    ///
    /// # Arguments
    ///
    /// * `img` - A mutable reference to the image where the line will be drawn.
    /// * `x1` - The starting x-coordinate of the line.
    /// * `y1` - The starting y-coordinate of the line.
    /// * `x2` - The ending x-coordinate of the line.
    /// * `y2` - The ending y-coordinate of the line.
    /// * `color` - The color of the line.
    fn draw_dash_dot_line(
        plot: &mut plot::Plot,
        img: &mut RgbImage,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        color: [u8; 3],
    ) {
        let dx = (x2 - x1).abs();
        let dy = -(y2 - y1).abs();
        let sx = if x1 < x2 { 1 } else { -1 };
        let sy = if y1 < y2 { 1 } else { -1 };
        let mut err = dx + dy;

        let mut x = x1;
        let mut y = y1;
        let segment_length = 10; // Length of each dash
        let gap_length = 5; // Length of each gap
        let dot_length = 2; // Length of each dot
        let mut counter = 0;
        let mut phase = 0; // 0 = dash, 1 = gap, 2 = dot

        while x != x2 || y != y2 {
            if phase == 0 || phase == 2 {
                // Draw for dash or dot
                for tx in -(plot.line_thickness as i32 / 2)..=(plot.line_thickness as i32 / 2) {
                    for ty in -(plot.line_thickness as i32 / 2)..=(plot.line_thickness as i32 / 2) {
                        let px = x + tx;
                        let py = y + ty;
                        if px >= 0 && px < plot.width as i32 && py >= 0 && py < plot.height as i32 {
                            img.put_pixel(px as u32, py as u32, Rgb(color));
                        }
                    }
                }
            }

            counter += 1;

            if phase == 0 && counter == segment_length {
                counter = 0;
                phase = 1; // Switch to gap
            } else if phase == 1 && counter == gap_length {
                counter = 0;
                phase = 2; // Switch to dot
            } else if phase == 2 && counter == dot_length {
                counter = 0;
                phase = 0; // Switch to dash
            }

            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x += sx;
            }
            if e2 <= dx {
                err += dx;
                y += sy;
            }
        }
    }
}

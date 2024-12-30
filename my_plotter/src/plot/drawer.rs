use crate::plot::barchart::BarChart;
use crate::plot::canvas::Canvas;
use crate::plot::cartesiangraph::CartesianGraph;
use ab_glyph::{FontRef, PxScale};
use imageproc::drawing::text_size;
use std::f64::consts::PI;

use super::{
    areachart::AreaChart, historgram::Histogram, orientation::Orientation, piechart::PieChart,
    quadrant1graph::Quadrant1Graph, scattergraph::ScatterGraph,
};

pub trait Drawer {
    fn draw(&mut self, canvas: &mut Canvas);
    fn draw_legend(&self, canvas: &mut Canvas);
}

impl Drawer for BarChart {
    fn draw(&mut self, canvas: &mut Canvas) {
        match self.orientation {
            Orientation::Vertical => self.draw_vertical(canvas),
            Orientation::Horizontal => self.draw_horizontal(canvas),
        }
    }

    fn draw_legend(&self, canvas: &mut Canvas) {
        let font =
            FontRef::try_from_slice(include_bytes!("../../resources/fonts/Arial.ttf")).unwrap(); // Font file
        let scale = PxScale { x: 10.0, y: 10.0 }; // Font size

        let square_size = 10; // Size of the colored square
        let padding = 5; // Space between the square and text
        let line_height = 20; // Vertical space for each legend entry
        let legend_margin = canvas.margin; // Margin from the bottom of the canvas

        let mut x = canvas.margin;
        let mut y = canvas.height - legend_margin; // Legend starts from the bottom

        for dataset in &self.datasets {
            let (w, h) = text_size(scale, &font, &dataset.label);
            // Draw the square
            for dy in 0..square_size {
                for dx in 0..square_size {
                    canvas.draw_pixel(
                        x + dx,
                        y + square_size * 2 + dy + h, // Adjust to align above baseline
                        dataset.color,
                    );
                }
            }

            // Draw the label text next to the square
            let text_x: u32 = x + square_size + padding;
            canvas.draw_text(
                text_x,
                y + 2 * square_size + h,
                &dataset.label,
                dataset.color,
                &font,
                scale,
            );

            // Move to the next legend entry
            x += square_size + padding + w + padding;
            if x > canvas.width - canvas.margin {
                // If the width exceeds, wrap to the next row
                x = canvas.margin;
                y -= line_height;
            }
        }
    }
}

impl Drawer for CartesianGraph {
    fn draw(&mut self, canvas: &mut Canvas) {
        canvas.clear();

        let font =
            FontRef::try_from_slice(include_bytes!("../../resources/fonts/Arial.ttf")).unwrap(); // Provide the path to your font file
        let scale_title = PxScale { x: 20.0, y: 20.0 }; // Font size
        let scale = PxScale { x: 15.0, y: 15.0 }; // Font size
        let (w_title, h_title) = text_size(scale_title, &font, &self.title);

        canvas.draw_text(
            (canvas.width - w_title) / 2,
            canvas.margin / 3 - h_title,
            &self.title,
            [0, 0, 0],
            &font,
            scale_title,
        );
        canvas.draw_grid(20, [200, 200, 200]);

        // Ensure x_min and x_max are symmetric
        let abs_x_min = self.x_min.abs();
        let abs_x_max = self.x_max.abs();

        if abs_x_min > abs_x_max {
            self.x_max = abs_x_min;
        } else {
            self.x_min = -abs_x_max;
        }

        // Draw X and Y axes
        let center_x = canvas.width / 2;
        let center_y = canvas.height / 2;
        canvas.draw_vertical_line(center_x, [0, 0, 0]);
        canvas.draw_horizontal_line(center_y, [0, 0, 0]);

        let scale_x = (canvas.width - 2 * canvas.margin) as f64 / (self.x_max - self.x_min);
        let scale_y = (canvas.height - 2 * canvas.margin) as f64 / (self.y_max - self.y_min); // Adjust y-range as needed

        for dataset in &self.datasets {
            for window in dataset.points.windows(2) {
                if let [p1, p2] = window {
                    let x1 = center_x as i32 + (p1.0 * scale_x) as i32;
                    let y1 = center_y as i32 - (p1.1 * scale_y) as i32;
                    let x2 = center_x as i32 + (p2.0 * scale_x) as i32;
                    let y2 = center_y as i32 - (p2.1 * scale_y) as i32;

                    // Simple line drawing algorithm (Bresenham)
                    let dx = (x2 - x1).abs();
                    let sx = if x1 < x2 { 1 } else { -1 };
                    let dy = -(y2 - y1).abs();
                    let sy = if y1 < y2 { 1 } else { -1 };
                    let mut err = dx + dy;

                    let mut x = x1;
                    let mut y = y1;

                    while x != x2 || y != y2 {
                        if x >= canvas.margin as i32
                            && x < (canvas.width - canvas.margin) as i32
                            && y >= canvas.margin as i32
                            && y < (canvas.height - canvas.margin) as i32
                        {
                            canvas.draw_pixel(x as u32, y as u32, dataset.color);
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
        }

        // X-axis label
        let (w, h) = text_size(scale, &font, &self.x_label);
        let x = canvas.width - (canvas.margin + w) / 2;
        let y = (canvas.height - h) / 2;
        canvas.draw_text(x, y, &self.x_label, [0, 0, 0], &font, scale);

        // Y-axis label
        let (w, h) = text_size(scale, &font, &self.y_label);
        let x = (canvas.width - w) / 2;
        let y = (h + canvas.margin) / 2;
        canvas.draw_text(x, y, &self.y_label, [0, 0, 0], &font, scale);

        // Draw X and Y axis tick values
        let num_ticks = 10;
        let x_tick_step = (canvas.width - 2 * canvas.margin) / num_ticks;
        let y_tick_step = (canvas.height - 2 * canvas.margin) / num_ticks;
        // Calculate the min and max for X and Y from datasets
        // let (x_min, x_max) = self
        //     .datasets
        //     .iter()
        //     .flat_map(|d| d.points.iter().map(|p| p.0))
        //     .fold((f64::INFINITY, f64::NEG_INFINITY), |(min, max), x| {
        //         (min.min(x), max.max(x))
        //     });
        // let (y_min, y_max) = self
        //     .datasets
        //     .iter()
        //     .flat_map(|d| d.points.iter().map(|p| p.1))
        //     .fold((f64::INFINITY, f64::NEG_INFINITY), |(min, max), y| {
        //         (min.min(y), max.max(y))
        //     });

        let (w, h) = text_size(scale, &font, format!("{:+.2}", self.x_min).as_str());

        for i in 0..=num_ticks {
            // X-axis ticks
            let x = canvas.margin + i * x_tick_step - w / 2;
            let value_x = self.x_min + ((self.x_max - self.x_min) / num_ticks as f64) * i as f64;
            let label_x = format!("{:+.2}", value_x);
            canvas.draw_text(
                x,
                canvas.height - canvas.margin + h,
                &label_x,
                [0, 0, 0],
                &font,
                scale,
            );

            // Y-axis ticks
            let y = canvas.margin + i * y_tick_step;
            let value_y = self.y_min + ((self.y_max - self.y_min) / num_ticks as f64) * i as f64;
            let label_y = format!("{:.2}", value_y);
            canvas.draw_text(
                canvas.margin - w - 5,
                canvas.height - y - h / 2,
                &label_y,
                [0, 0, 0],
                &font,
                scale,
            );
        }
    }

    fn draw_legend(&self, canvas: &mut Canvas) {
        let font =
            FontRef::try_from_slice(include_bytes!("../../resources/fonts/Arial.ttf")).unwrap(); // Font file
        let scale = PxScale { x: 10.0, y: 10.0 }; // Font size

        let square_size = 10; // Size of the colored square
        let padding = 5; // Space between the square and text
        let line_height = 20; // Vertical space for each legend entry
        let legend_margin = canvas.margin; // Margin from the bottom of the canvas

        let mut x = canvas.margin;
        let mut y = canvas.height - legend_margin; // Legend starts from the bottom

        for dataset in &self.datasets {
            let (w, h) = text_size(scale, &font, &dataset.label);
            // Draw the square
            for dy in 0..square_size {
                for dx in 0..square_size {
                    canvas.draw_pixel(
                        x + dx,
                        y + square_size * 2 + dy + h, // Adjust to align above baseline
                        dataset.color,
                    );
                }
            }

            // Draw the label text next to the square
            let text_x: u32 = x + square_size + padding;
            canvas.draw_text(
                text_x,
                y + 2 * square_size + h,
                &dataset.label,
                dataset.color,
                &font,
                scale,
            );

            // Move to the next legend entry
            x += square_size + padding + w + padding;
            if x > canvas.width - canvas.margin {
                // If the width exceeds, wrap to the next row
                x = canvas.margin;
                y -= line_height;
            }
        }
    }
}

impl Drawer for Quadrant1Graph {
    fn draw(&mut self, canvas: &mut Canvas) {
        canvas.clear();

        let font =
            FontRef::try_from_slice(include_bytes!("../../resources/fonts/Arial.ttf")).unwrap();
        let scale_title = PxScale { x: 20.0, y: 20.0 };
        let scale_labels = PxScale { x: 15.0, y: 15.0 };

        // Draw the title
        let (w_title, h_title) = text_size(scale_title, &font, &self.title);
        canvas.draw_text(
            (canvas.width - w_title) / 2,
            ((canvas.margin + h_title) / 2).max(0) as u32,
            &self.title,
            [0, 0, 0],
            &font,
            scale_title,
        );

        // Calculate dataset limits
        let (x_min, x_max) = self
            .datasets
            .iter()
            .flat_map(|dataset| dataset.points.iter().map(|&(x, _)| x))
            .fold((0.0_f64, 0.0_f64), |(min, max), x| (min.min(x), max.max(x)));

        let (y_min, y_max) = self
            .datasets
            .iter()
            .flat_map(|dataset| dataset.points.iter().map(|&(_, y)| y))
            .fold((0.0_f64, 0.0_f64), |(min, max), y| (min.min(y), max.max(y)));

        // Adjust limits to include (0, 0)
        let x_min = x_min.min(0.0);
        let y_min = y_min.min(0.0);
        // Calculate scales
        let scale_x = (canvas.width - 2 * canvas.margin) as f64 / (x_max - x_min);
        let scale_y = (canvas.height - 2 * canvas.margin) as f64 / (y_max - y_min);

        // Calculate scales based on dataset limits
        // let scale_x = (canvas.width - 2 * canvas.margin) as f64 / (x_max - x_min);
        // let scale_y = (canvas.height - 2 * canvas.margin) as f64 / (y_max - y_min);

        // Draw grids
        canvas.draw_grid(20, [200, 200, 200]);
        canvas.draw_vertical_line(canvas.margin, [0, 0, 0]);
        canvas.draw_vertical_line(canvas.width - canvas.margin, [0, 0, 0]);
        canvas.draw_horizontal_line(canvas.height - canvas.margin, [0, 0, 0]);
        canvas.draw_horizontal_line(canvas.margin, [0, 0, 0]);

        // Draw axes
        // let origin_x = canvas.margin as i32;
        // let origin_y = canvas.height as i32 - canvas.margin as i32;
        // Draw axes
        let origin_x = canvas.margin as i32 + ((0.0 - x_min) * scale_x) as i32;
        let origin_y =
            canvas.height as i32 - canvas.margin as i32 - ((0.0 - y_min) * scale_y) as i32;

        let (w, h) = text_size(scale_labels, &font, &self.x_label);
        // Draw axes labels
        canvas.draw_text(
            canvas.width - canvas.margin + w / 2,
            origin_y as u32 - h / 2,
            &self.x_label,
            [0, 0, 0],
            &font,
            scale_labels,
        );

        let (w, h) = text_size(scale_labels, &font, &self.y_label);
        canvas.draw_text(
            origin_x as u32 - w / 2,
            canvas.margin - h - 10,
            &self.y_label,
            [0, 0, 0],
            &font,
            scale_labels,
        );

        // Draw axis tick values
        let num_ticks = 10;

        // X-axis ticks
        let x_tick_step = (x_max - x_min) / num_ticks as f64;
        for i in 0..=num_ticks {
            let value_x = x_min + i as f64 * x_tick_step;
            let tick_x = origin_x + ((value_x - x_min) * scale_x) as i32;

            let value_label = format!("{:.2}", value_x);
            let (w, h) = text_size(scale_labels, &font, &value_label);

            canvas.draw_text(
                (tick_x - w as i32 / 2).max(0) as u32,
                (origin_y + h as i32).min(canvas.height as i32 - 1) as u32,
                &value_label,
                [0, 0, 0],
                &font,
                scale_labels,
            );
        }

        // Y-axis ticks
        let y_tick_step = (y_max - y_min) / num_ticks as f64;
        for i in 0..=num_ticks {
            let value_y = y_min + i as f64 * y_tick_step;
            let tick_y = origin_y - ((value_y - y_min) * scale_y) as i32;

            let value_label = format!("{:.2}", value_y);
            let (w, h) = text_size(scale_labels, &font, &value_label);

            canvas.draw_text(
                (origin_x - w as i32 - 5).max(0) as u32,
                (tick_y - h as i32 / 2).max(0) as u32,
                &value_label,
                [0, 0, 0],
                &font,
                scale_labels,
            );
        }

        // Draw datasets
        for dataset in &self.datasets {
            for window in dataset.points.windows(2) {
                if let [p1, p2] = window {
                    let x1 = origin_x + ((p1.0 - x_min) * scale_x) as i32;
                    let y1 = origin_y - ((p1.1 - y_min) * scale_y) as i32;
                    let x2 = origin_x + ((p2.0 - x_min) * scale_x) as i32;
                    let y2 = origin_y - ((p2.1 - y_min) * scale_y) as i32;

                    canvas.draw_line(x1, y1, x2, y2, dataset.color, dataset.line_type.clone());
                }
            }
        }
        // Draw legend
        self.draw_legend(canvas);
    }

    fn draw_legend(&self, canvas: &mut Canvas) {
        let font =
            FontRef::try_from_slice(include_bytes!("../../resources/fonts/Arial.ttf")).unwrap(); // Font file
        let scale = PxScale { x: 10.0, y: 10.0 }; // Font size

        let square_size = 10; // Size of the colored square
        let padding = 5; // Space between the square and text
        let line_height = 20; // Vertical space for each legend entry
        let legend_margin = canvas.margin; // Margin from the bottom of the canvas

        let mut x = canvas.margin;
        let mut y = canvas.height - legend_margin; // Legend starts from the bottom

        for dataset in &self.datasets {
            let (w, h) = text_size(scale, &font, &dataset.label);
            // Draw the square
            for dy in 0..square_size {
                for dx in 0..square_size {
                    canvas.draw_pixel(
                        x + dx,
                        y + square_size * 2 + dy + h, // Adjust to align above baseline
                        dataset.color,
                    );
                }
            }

            // Draw the label text next to the square
            let text_x: u32 = x + square_size + padding;
            canvas.draw_text(
                text_x,
                y + 2 * square_size + h,
                &dataset.label,
                dataset.color,
                &font,
                scale,
            );

            // Move to the next legend entry
            x += square_size + padding + w + padding;
            if x > canvas.width - canvas.margin {
                // If the width exceeds, wrap to the next row
                x = canvas.margin;
                y -= line_height;
            }
        }
    }
}

impl Drawer for PieChart {
    fn draw(&mut self, canvas: &mut Canvas) {
        canvas.clear();

        let font =
            FontRef::try_from_slice(include_bytes!("../../resources/fonts/Arial.ttf")).unwrap();
        let scale_title = PxScale { x: 20.0, y: 20.0 };

        // Draw the title
        let (w_title, h_title) = text_size(scale_title, &font, &self.title);
        let title_x = (canvas.width).saturating_sub(w_title) / 2;
        let title_y = (canvas.margin / 3).saturating_sub(h_title) as u32;
        canvas.draw_text(title_x, title_y, &self.title, [0, 0, 0], &font, scale_title);

        // Calculate total value
        let total: f64 = self.datasets.iter().map(|(_, value, _)| value).sum();
        if total == 0.0 {
            return;
        }

        // Center and radius of the pie chart
        let center_x = canvas.width / 2;
        let center_y = canvas.height / 2;
        let radius = (canvas.width.min(canvas.height) / 2 - canvas.margin) as i32;

        let mut start_angle = 0.0;
        for (_label, value, color) in &self.datasets {
            let percentage = value / total;
            let sweep_angle = 2.0 * PI * percentage;

            // Draw the slice
            self.draw_slice(
                canvas,
                center_x as i32,
                center_y as i32,
                radius,
                start_angle,
                start_angle + sweep_angle,
                *color,
            );

            // Calculate mid-angle for label placement
            let mid_angle = start_angle + sweep_angle / 2.0;
            let label_x = center_x as f64 + (radius as f64 * 0.6 * mid_angle.cos());
            let label_y = center_y as f64 - (radius as f64 * 0.6 * mid_angle.sin());
            canvas.draw_text(
                label_x as u32,
                label_y as u32,
                &format!("{:.1}%", percentage * 100.0),
                [0, 0, 0],
                &font,
                PxScale { x: 12.0, y: 12.0 },
            );

            start_angle += sweep_angle;
        }

        // Draw legend
        self.draw_legend(canvas);
    }

    fn draw_legend(&self, canvas: &mut Canvas) {
        let font =
            FontRef::try_from_slice(include_bytes!("../../resources/fonts/Arial.ttf")).unwrap();
        let scale = PxScale { x: 15.0, y: 15.0 };

        let square_size = 10;
        let padding = 5;
        let line_height = 20;
        let x = canvas.margin;
        let mut y = canvas.height - canvas.margin;

        for (label, _, color) in &self.datasets {
            // Draw the square
            for dy in 0..square_size {
                for dx in 0..square_size {
                    canvas.draw_pixel(x + dx, y + dy, *color);
                }
            }

            // Draw the label
            canvas.draw_text(x + square_size + padding, y, label, [0, 0, 0], &font, scale);

            y += line_height;
        }
    }
}

impl Drawer for ScatterGraph {
    fn draw(&mut self, canvas: &mut Canvas) {
        canvas.clear();

        let font =
            FontRef::try_from_slice(include_bytes!("../../resources/fonts/Arial.ttf")).unwrap();
        let scale_title = PxScale { x: 20.0, y: 20.0 };
        let scale_labels = PxScale { x: 15.0, y: 15.0 };

        // Draw the title
        let (w_title, h_title) = text_size(scale_title, &font, &self.title);
        let title_x = (canvas.width).saturating_sub(w_title) / 2;
        let title_y = (canvas.margin / 3).saturating_sub(h_title) as u32;
        canvas.draw_text(title_x, title_y, &self.title, [0, 0, 0], &font, scale_title);

        // Calculate dataset limits
        let (x_min, x_max) = self
            .datasets
            .iter()
            .flat_map(|dataset| dataset.points.iter().map(|&(x, _)| x))
            .fold((f64::INFINITY, f64::NEG_INFINITY), |(min, max), x| {
                (min.min(x), max.max(x))
            });

        let (y_min, y_max) = self
            .datasets
            .iter()
            .flat_map(|dataset| dataset.points.iter().map(|&(_, y)| y))
            .fold((f64::INFINITY, f64::NEG_INFINITY), |(min, max), y| {
                (min.min(y), max.max(y))
            });

        // Adjust limits to include (0, 0)
        let x_min = x_min.min(0.0);
        let y_min = y_min.min(0.0);

        // Calculate scales
        let scale_x = (canvas.width - 2 * canvas.margin) as f64 / (x_max - x_min);
        let scale_y = (canvas.height - 2 * canvas.margin) as f64 / (y_max - y_min);

        // Draw grids
        canvas.draw_grid(20, [200, 200, 200]);

        canvas.draw_vertical_line(canvas.margin, [0, 0, 0]);
        canvas.draw_vertical_line(canvas.width - canvas.margin, [0, 0, 0]);
        canvas.draw_horizontal_line(canvas.height - canvas.margin, [0, 0, 0]);
        canvas.draw_horizontal_line(canvas.margin, [0, 0, 0]);

        let origin_x = canvas.margin as i32 + ((0.0 - x_min) * scale_x) as i32;
        let origin_y =
            canvas.height as i32 - canvas.margin as i32 - ((0.0 - y_min) * scale_y) as i32;

        let (w, h) = text_size(scale_labels, &font, &self.x_label);
        // Draw axes labels
        canvas.draw_text(
            canvas.width - canvas.margin + w / 2,
            origin_y as u32 - h / 2,
            &self.x_label,
            [0, 0, 0],
            &font,
            scale_labels,
        );

        let (w, h) = text_size(scale_labels, &font, &self.y_label);
        canvas.draw_text(
            origin_x as u32 - w / 2,
            canvas.margin - h - 10,
            &self.y_label,
            [0, 0, 0],
            &font,
            scale_labels,
        );

        // Draw axis tick values
        let num_ticks = 10;

        // X-axis ticks
        let x_tick_step = (x_max - x_min) / num_ticks as f64;
        for i in 0..=num_ticks {
            let value_x = x_min + i as f64 * x_tick_step;
            let tick_x = origin_x + ((value_x - x_min) * scale_x) as i32;

            let value_label = format!("{:.2}", value_x);
            let (w, h) = text_size(scale_labels, &font, &value_label);

            canvas.draw_text(
                (tick_x - w as i32 / 2).max(0) as u32,
                (origin_y + h as i32).min(canvas.height as i32 - 1) as u32,
                &value_label,
                [0, 0, 0],
                &font,
                scale_labels,
            );
        }

        // Y-axis ticks
        let y_tick_step = (y_max - y_min) / num_ticks as f64;
        for i in 0..=num_ticks {
            let value_y = y_min + i as f64 * y_tick_step;
            let tick_y = origin_y - ((value_y - y_min) * scale_y) as i32;

            let value_label = format!("{:.2}", value_y);
            let (w, h) = text_size(scale_labels, &font, &value_label);

            canvas.draw_text(
                (origin_x - w as i32 - 5).max(0) as u32,
                (tick_y - h as i32 / 2).max(0) as u32,
                &value_label,
                [0, 0, 0],
                &font,
                scale_labels,
            );
        }

        // Draw scatter points
        for dataset in &self.datasets {
            for &(_x, _y) in &dataset.points {
                // Draw a small square or circle to represent the point
                for dataset in &self.datasets {
                    for &(x, y) in &dataset.points {
                        let px = origin_x + ((x - x_min) * scale_x) as i32;
                        let py = origin_y - ((y - y_min) * scale_y) as i32;

                        self.draw_dot(canvas, px, py, dataset.dot_type.clone(), dataset.color);
                    }
                }
            }
        }

        // Draw legend
        self.draw_legend(canvas);
    }

    fn draw_legend(&self, canvas: &mut Canvas) {
        let font =
            FontRef::try_from_slice(include_bytes!("../../resources/fonts/Arial.ttf")).unwrap(); // Font file
        let scale = PxScale { x: 10.0, y: 10.0 }; // Font size

        let square_size = 10; // Size of the colored square
        let padding = 5; // Space between the square and text
        let line_height = 20; // Vertical space for each legend entry
        let legend_margin = canvas.margin; // Margin from the bottom of the canvas

        let mut x = canvas.margin;
        let mut y = canvas.height - legend_margin; // Legend starts from the bottom

        for dataset in &self.datasets {
            let (w, h) = text_size(scale, &font, &dataset.label);
            // Draw the square
            for dy in 0..square_size {
                for dx in 0..square_size {
                    canvas.draw_pixel(
                        x + dx,
                        y + square_size * 2 + dy + h, // Adjust to align above baseline
                        dataset.color,
                    );
                }
            }

            // Draw the label text next to the square
            let text_x: u32 = x + square_size + padding;
            canvas.draw_text(
                text_x,
                y + 2 * square_size + h,
                &dataset.label,
                dataset.color,
                &font,
                scale,
            );

            // Move to the next legend entry
            x += square_size + padding + w + padding;
            if x > canvas.width - canvas.margin {
                // If the width exceeds, wrap to the next row
                x = canvas.margin;
                y -= line_height;
            }
        }
    }
}

impl Drawer for AreaChart {
    fn draw(&mut self, canvas: &mut Canvas) {
        canvas.clear();

        let font =
            FontRef::try_from_slice(include_bytes!("../../resources/fonts/Arial.ttf")).unwrap();
        let scale_title = PxScale { x: 20.0, y: 20.0 };
        let scale_labels = PxScale { x: 15.0, y: 15.0 };

        // Draw the title
        let (w_title, h_title) = text_size(scale_title, &font, &self.title);
        let title_x = (canvas.width).saturating_sub(w_title) / 2;
        let title_y = (canvas.margin / 3).saturating_sub(h_title) as u32;
        canvas.draw_text(title_x, title_y, &self.title, [0, 0, 0], &font, scale_title);

        // Calculate dataset limits
        let (x_min, x_max) = self
            .datasets
            .iter()
            .flat_map(|dataset| dataset.points.iter().map(|&(x, _)| x))
            .fold((f64::INFINITY, f64::NEG_INFINITY), |(min, max), x| {
                (min.min(x), max.max(x))
            });

        let (y_min, y_max) = self
            .datasets
            .iter()
            .flat_map(|dataset| dataset.points.iter().map(|&(_, y)| y))
            .fold((f64::INFINITY, f64::NEG_INFINITY), |(min, max), y| {
                (min.min(y), max.max(y))
            });

        // Adjust limits to include (0, 0)
        let x_min = x_min.min(0.0);
        let y_min = y_min.min(0.0);

        // Calculate scales
        let scale_x = (canvas.width - 2 * canvas.margin) as f64 / (x_max - x_min);
        let scale_y = (canvas.height - 2 * canvas.margin) as f64 / (y_max - y_min);

        // Draw grids
        canvas.draw_grid(20, [200, 200, 200]);

        // Draw axes
        // let origin_x = canvas.margin as i32;
        // let origin_y = canvas.height as i32 - canvas.margin as i32;
        // Draw axes
        let origin_x = canvas.margin as i32 + ((0.0 - x_min) * scale_x) as i32;
        let origin_y =
            canvas.height as i32 - canvas.margin as i32 - ((0.0 - y_min) * scale_y) as i32;

        let (w, h) = text_size(scale_labels, &font, &self.x_label);
        // Draw axes labels
        canvas.draw_text(
            canvas.width - canvas.margin + w / 2,
            origin_y as u32 - h / 2,
            &self.x_label,
            [0, 0, 0],
            &font,
            scale_labels,
        );

        let (w, h) = text_size(scale_labels, &font, &self.y_label);
        canvas.draw_text(
            origin_x as u32 - w / 2,
            canvas.margin - h - 10,
            &self.y_label,
            [0, 0, 0],
            &font,
            scale_labels,
        );

        // Draw axis tick values
        let num_ticks = 10;

        // X-axis ticks
        let x_tick_step = (x_max - x_min) / num_ticks as f64;
        for i in 0..=num_ticks {
            let value_x = x_min + i as f64 * x_tick_step;
            let tick_x = origin_x + ((value_x - x_min) * scale_x) as i32;

            let value_label = format!("{:.2}", value_x);
            let (w, h) = text_size(scale_labels, &font, &value_label);

            canvas.draw_text(
                (tick_x - w as i32 / 2).max(0) as u32,
                (origin_y + h as i32).min(canvas.height as i32 - 1) as u32,
                &value_label,
                [0, 0, 0],
                &font,
                scale_labels,
            );
        }

        // Y-axis ticks
        let y_tick_step = (y_max - y_min) / num_ticks as f64;
        for i in 0..=num_ticks {
            let value_y = y_min + i as f64 * y_tick_step;
            let tick_y = origin_y - ((value_y - y_min) * scale_y) as i32;

            let value_label = format!("{:.2}", value_y);
            let (w, h) = text_size(scale_labels, &font, &value_label);

            canvas.draw_text(
                (origin_x - w as i32 - 5).max(0) as u32,
                (tick_y - h as i32 / 2).max(0) as u32,
                &value_label,
                [0, 0, 0],
                &font,
                scale_labels,
            );
        }

        // Draw areas under the curves
        for dataset in &self.datasets {
            self.draw_area(canvas, dataset, origin_x, origin_y, scale_x, scale_y);
        }

        canvas.draw_vertical_line(canvas.margin, [0, 0, 0]);
        canvas.draw_vertical_line(canvas.width - canvas.margin, [0, 0, 0]);
        canvas.draw_horizontal_line(canvas.height - canvas.margin, [0, 0, 0]);
        canvas.draw_horizontal_line(canvas.margin, [0, 0, 0]);

        // Draw legend
        self.draw_legend(canvas);
    }

    fn draw_legend(&self, canvas: &mut Canvas) {
        let font =
            FontRef::try_from_slice(include_bytes!("../../resources/fonts/Arial.ttf")).unwrap(); // Font file
        let scale = PxScale { x: 10.0, y: 10.0 }; // Font size

        let square_size = 10; // Size of the colored square
        let padding = 5; // Space between the square and text
        let line_height = 20; // Vertical space for each legend entry
        let legend_margin = canvas.margin; // Margin from the bottom of the canvas

        let mut x = canvas.margin;
        let mut y = canvas.height - legend_margin; // Legend starts from the bottom

        for dataset in &self.datasets {
            let (w, h) = text_size(scale, &font, &dataset.label);
            // Draw the square
            for dy in 0..square_size {
                for dx in 0..square_size {
                    canvas.draw_pixel(
                        x + dx,
                        y + square_size * 2 + dy + h, // Adjust to align above baseline
                        dataset.color,
                    );
                }
            }

            // Draw the label text next to the square
            let text_x: u32 = x + square_size + padding;
            canvas.draw_text(
                text_x,
                y + 2 * square_size + h,
                &dataset.label,
                dataset.color,
                &font,
                scale,
            );

            // Move to the next legend entry
            x += square_size + padding + w + padding;
            if x > canvas.width - canvas.margin {
                // If the width exceeds, wrap to the next row
                x = canvas.margin;
                y -= line_height;
            }
        }
    }
}

impl Drawer for Histogram {
    fn draw(&mut self, canvas: &mut Canvas) {
        let bin_data = self.calculate_bins();
        let font =
            FontRef::try_from_slice(include_bytes!("../../resources/fonts/Arial.ttf")).unwrap();
        let scale_title = PxScale { x: 20.0, y: 20.0 };
        let scale_labels = PxScale { x: 15.0, y: 15.0 };
        let scale_axis_values = PxScale { x: 10.0, y: 10.0 };

        let y_max = bin_data.iter().map(|&(_, freq)| freq).fold(0.0, f64::max);

        let scale_x = (canvas.width - 2 * canvas.margin) as f64 / self.bins as f64;
        let scale_y = (canvas.height - 2 * canvas.margin) as f64 / y_max;

        canvas.clear();
        canvas.draw_grid(50, [200, 200, 200]);

        // Draw axes
        let origin_x = canvas.margin as i32;
        let origin_y = canvas.height as i32 - canvas.margin as i32;

        // Draw bars with edges
        let bin_start = bin_data[0].0; // First bin's start
        let bin_width = (bin_data[1].0 - bin_start).abs(); // Width of each bin

        for (i, &(_, freq)) in bin_data.iter().enumerate() {
            let bar_height = (freq * scale_y) as i32;
            let bar_left = origin_x + (i as f64 * scale_x) as i32;
            let bar_right = bar_left + scale_x as i32;

            // Fill the bar
            for x in bar_left..=bar_right {
                for y in (origin_y - bar_height)..origin_y {
                    canvas.draw_pixel(x as u32, y as u32, self.color);
                }
            }

            // Draw the edges (outline)
            let edge_color = [0, 0, 0]; // Black color for edges
                                        // Left edge
            for y in (origin_y - bar_height)..origin_y {
                canvas.draw_pixel(bar_left as u32, y as u32, edge_color);
            }
            // Right edge
            for y in (origin_y - bar_height)..origin_y {
                canvas.draw_pixel(bar_right as u32, y as u32, edge_color);
            }
            // Top edge
            for x in bar_left..=bar_right {
                canvas.draw_pixel(x as u32, (origin_y - bar_height) as u32, edge_color);
            }
        }

        // Add x-axis ticks and labels at bin edges
        for i in 0..=self.bins {
            let edge_x = origin_x + (i as f64 * scale_x) as i32;
            let edge_value = bin_start + i as f64 * bin_width;

            canvas.draw_pixel(edge_x as u32, origin_y as u32, [0, 0, 0]); // Tick mark
            let edge_label = format!("{:.1}", edge_value);
            let (w, _h) = text_size(scale_axis_values, &font, &edge_label);
            canvas.draw_text(
                edge_x as u32 - w / 2,
                (origin_y + 10) as u32,
                &edge_label,
                [0, 0, 0],
                &font,
                scale_axis_values,
            );
        }

        // Add y-axis ticks and labels
        let num_y_ticks = 10;
        for i in 0..=num_y_ticks {
            let tick_value = y_max * i as f64 / num_y_ticks as f64;
            let tick_y = origin_y - (tick_value * scale_y) as i32;

            canvas.draw_pixel(origin_x as u32, tick_y as u32, [0, 0, 0]); // Tick mark
            let tick_label = format!("{:.1}", tick_value);
            canvas.draw_text(
                (origin_x - 30) as u32,
                tick_y as u32,
                &tick_label,
                [0, 0, 0],
                &font,
                scale_axis_values,
            );
        }

        let (w, h) = text_size(scale_title, &font, &self.title);
        // Draw title and axis labels
        canvas.draw_text(
            (canvas.width - w) / 2,
            canvas.margin / 2 - h / 2,
            &self.title,
            [0, 0, 0],
            &font,
            scale_title,
        );

        let (w, h) = text_size(scale_title, &font, &self.x_label);
        canvas.draw_text(
            canvas.width / 2 - w / 2,
            (canvas.height - canvas.margin / 2 - h / 2) as u32,
            &self.x_label,
            [0, 0, 0],
            &font,
            scale_labels,
        );

        let (w, h) = text_size(scale_title, &font, &self.y_label);
        canvas.draw_text(
            canvas.margin - w / 2,
            canvas.margin - h - 10,
            &self.y_label,
            [0, 0, 0],
            &font,
            scale_labels,
        );

        canvas.draw_vertical_line(canvas.margin, [0, 0, 0]);
        canvas.draw_vertical_line(canvas.width - canvas.margin, [0, 0, 0]);
        canvas.draw_horizontal_line(canvas.height - canvas.margin, [0, 0, 0]);
        canvas.draw_horizontal_line(canvas.margin, [0, 0, 0]);
    }

    fn draw_legend(&self, _canvas: &mut Canvas) {
        // Histogram does not have a legend
    }
}

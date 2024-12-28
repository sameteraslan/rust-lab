use ab_glyph::{FontRef, PxScale};
use imageproc::drawing::text_size;

use crate::{drawer::Drawer, plot::bardataset::BarDataset};

use super::{canvas::Canvas, orientation::Orientation};

pub struct BarChart {
    pub datasets: Vec<BarDataset>,
    pub title: String,
    pub x_label: String,
    pub y_label: String,
    pub orientation: Orientation,
}

impl BarChart {
    pub fn new(title: &str, x_label: &str, y_label: &str, orientation: Orientation) -> Self {
        Self {
            datasets: Vec::new(),
            title: title.to_string(),
            x_label: x_label.to_string(),
            y_label: y_label.to_string(),
            orientation,
        }
    }

    pub fn add_dataset(&mut self, dataset: BarDataset) {
        self.datasets.push(dataset);
    }

    // Horizontal grouped bar chart logic
    pub fn draw_horizontal(&self, canvas: &mut Canvas) {
        canvas.clear();
        println!("Drawing Bar Chart: {}", self.title);

        let font =
            FontRef::try_from_slice(include_bytes!("../../resources/fonts/Arial.ttf")).unwrap();
        let scale_title = PxScale { x: 20.0, y: 20.0 };
        let scale_labels = PxScale { x: 15.0, y: 15.0 };

        // Draw the title
        let (w_title, h_title) = text_size(scale_title, &font, &self.title);
        let title_x = (canvas.width).saturating_sub(w_title) / 2;
        let title_y = (canvas.margin / 3).saturating_sub(h_title) as u32;
        canvas.draw_text(title_x, title_y, &self.title, [0, 0, 0], &font, scale_title);

        // Get unique y-axis values
        let unique_y_values: Vec<u32> = self
            .datasets
            .iter()
            .flat_map(|dataset| dataset.data.iter().map(|(y, _)| *y as u32))
            .collect::<std::collections::BTreeSet<_>>()
            .into_iter()
            .collect();

        let y_count = unique_y_values.len();

        let (x_min, x_max) = self
            .datasets
            .iter()
            .flat_map(|dataset| dataset.data.iter().map(|&(_, y)| y))
            .fold((0.0_f64, 0.0_f64), |(min, max), y| (min.min(y), max.max(y)));

        // Adjust limits to include (0, 0)
        let x_min = x_min.min(0.0);

        // Calculate scales
        let scale_y = (canvas.height - 2 * canvas.margin) as f64 / y_count as f64;
        let scale_x = (canvas.width - 2 * canvas.margin) as f64 / x_max;

        // Draw grids
        canvas.draw_grid(20, [200, 200, 200]);

        // Draw axes
        let origin_x = canvas.margin as i32;
        let origin_y = canvas.height as i32 - canvas.margin as i32;

        // Draw axis labels
        canvas.draw_text(
            canvas.width - canvas.margin + 5,
            origin_y as u32,
            &self.y_label,
            [0, 0, 0],
            &font,
            scale_labels,
        );

        canvas.draw_text(
            origin_x as u32 - 20,
            canvas.margin / 2,
            &self.x_label,
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

        // Draw grouped horizontal bars
        let group_height = scale_y * 0.8; // Height of each group
        let bar_height = group_height / self.datasets.len() as f64; // Height of each bar

        for (group_index, y_label) in unique_y_values.iter().enumerate() {
            let group_center_y = origin_y - ((group_index as f64 + 0.5) * scale_y) as i32;

            // Draw y-axis label
            let (w, h) = text_size(scale_labels, &font, &y_label.to_string());
            canvas.draw_text(
                (origin_x - w as i32 - 10).max(0) as u32,
                (group_center_y - h as i32 / 2).max(0) as u32,
                &y_label.to_string(),
                [0, 0, 0],
                &font,
                scale_labels,
            );

            // Draw bars for each company in the group
            for (company_index, dataset) in self.datasets.iter().enumerate() {
                if let Some(&(_, value)) = dataset
                    .data
                    .iter()
                    .find(|(y, _)| &(*y as u32).to_string() == &y_label.to_string())
                {
                    let bar_length = (value * scale_x) as i32;
                    let bar_top = group_center_y - (group_height / 2.0) as i32
                        + (company_index as f64 * bar_height) as i32;
                    let bar_bottom = bar_top + bar_height as i32;

                    for x in origin_x..(origin_x + bar_length) {
                        for y in bar_top..bar_bottom {
                            canvas.draw_pixel(x as u32, y as u32, dataset.color);
                        }
                    }
                }
            }
        }

        canvas.draw_vertical_line(canvas.margin, [0, 0, 0]);
        canvas.draw_horizontal_line(canvas.height - canvas.margin, [0, 0, 0]);
        // Draw legend
        self.draw_legend(canvas);
    }

    pub fn draw_vertical(&self, canvas: &mut Canvas) {
        canvas.clear();
        println!("Drawing Bar Chart: {}", self.title);

        let font =
            FontRef::try_from_slice(include_bytes!("../../resources/fonts/Arial.ttf")).unwrap();
        let scale_title = PxScale { x: 20.0, y: 20.0 };
        let scale_labels = PxScale { x: 15.0, y: 15.0 };

        // Draw the title
        let (w_title, h_title) = text_size(scale_title, &font, &self.title);
        let title_x = (canvas.width).saturating_sub(w_title) / 2;
        let title_y = (canvas.margin / 3).saturating_sub(h_title) as u32;
        canvas.draw_text(title_x, title_y, &self.title, [0, 0, 0], &font, scale_title);

        // Get unique x-axis values
        let unique_x_values: Vec<u32> = self
            .datasets
            .iter()
            .flat_map(|dataset| dataset.data.iter().map(|(x, _)| *x as u32))
            .collect::<std::collections::BTreeSet<_>>()
            .into_iter()
            .collect();

        let x_count = unique_x_values.len();
        let y_max = self
            .datasets
            .iter()
            .flat_map(|dataset| dataset.data.iter().map(|(_, y)| *y))
            .fold(0.0_f64, |max, y| max.max(y));

        // Calculate scales
        let scale_x = (canvas.width - 2 * canvas.margin) as f64 / x_count as f64;
        let scale_y = (canvas.height - 2 * canvas.margin) as f64 / y_max;

        // Draw grids
        canvas.draw_grid(50, [200, 200, 200]);

        // Draw axes
        let origin_x = canvas.margin as i32;
        let origin_y = canvas.height as i32 - canvas.margin as i32;

        canvas.draw_vertical_line(canvas.margin, [0, 0, 0]);
        canvas.draw_vertical_line(canvas.width - canvas.margin, [0, 0, 0]);
        canvas.draw_horizontal_line(canvas.height - canvas.margin, [0, 0, 0]);
        canvas.draw_horizontal_line(canvas.margin, [0, 0, 0]);

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

        // Y-axis ticks
        let y_tick_step = y_max / num_ticks as f64;
        for i in 0..=num_ticks {
            let value_y = i as f64 * y_tick_step;
            let tick_y = origin_y - (value_y * scale_y) as i32;

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

        // Draw x-axis labels and grouped bars
        let group_width = scale_x * 0.8; // Width of each group of bars
        let bar_width = group_width / self.datasets.len() as f64; // Width of each bar

        for (group_index, x_label) in unique_x_values.iter().enumerate() {
            let group_center_x = origin_x + ((group_index as f64 + 0.5) * scale_x) as i32;

            // Draw x-axis label
            let (w, h) = text_size(scale_labels, &font, &x_label.to_string());

            canvas.draw_text(
                (group_center_x - w as i32 / 2).max(0) as u32,
                (origin_y + h as i32).min(canvas.height as i32 - 1) as u32,
                &x_label.to_string(),
                [0, 0, 0],
                &font,
                scale_labels,
            );

            // Draw bars for each company in the group
            for (company_index, dataset) in self.datasets.iter().enumerate() {
                if let Some(&(_, income)) = dataset
                    .data
                    .iter()
                    .find(|(x, _)| &(*x as u32).to_string() == &x_label.to_string())
                {
                    let bar_height = (income * scale_y) as i32;
                    let bar_left = group_center_x - (group_width / 2.0) as i32
                        + (company_index as f64 * bar_width) as i32;
                    let bar_right = bar_left + bar_width as i32;

                    for x in bar_left..=bar_right {
                        for y in (origin_y - bar_height)..origin_y {
                            canvas.draw_pixel(x as u32, y as u32, dataset.color);
                        }
                    }
                }
            }
        }

        // Draw legend
        self.draw_legend(canvas);
    }
}

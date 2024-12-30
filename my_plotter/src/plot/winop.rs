use std::{
    thread,
    time::{Duration, Instant},
};

use crate::plot::canvas::Canvas;
use image::ImageBuffer;
use imageproc::drawing::{draw_line_segment_mut, draw_text_mut, text_size};
use minifb::{Key, MouseMode, Window, WindowOptions};

use super::{
    barchart::BarChart, cartesiangraph::CartesianGraph, drawer::Drawer, historgram::Histogram,
    linetype::LineType, piechart::PieChart, scattergraph::ScatterGraph,
};

pub struct Winop;

impl Winop {
    pub fn new() -> Self {
        Self
    }

    /// Displays the plot in real-time with continuous updates
    pub fn display_real_time<T: HoverablePlot + Drawer>(
        canvas: &mut Canvas,
        plot: &mut T,
        title: &str,
        mut update_data: impl FnMut(&mut T) + 'static,
        fps: u32,
    ) {
        let width = canvas.width as usize;
        let height = canvas.height as usize;

        let mut window = Window::new(
            title,
            width,
            height,
            WindowOptions {
                resize: true,
                scale: minifb::Scale::X1,
                ..WindowOptions::default()
            },
        )
        .unwrap_or_else(|e| panic!("Unable to open Window: {}", e));

        let frame_duration = Duration::from_secs_f32(1.0 / fps as f32);
        let mut last_frame_time = Instant::now();

        let mut hover_enabled = false;
        let mut show_hints = false;
        while window.is_open() && !window.is_key_pressed(Key::Escape, minifb::KeyRepeat::No) {
            // Update data for real-time rendering
            if last_frame_time.elapsed() >= frame_duration {
                update_data(plot);
                plot.draw(canvas);
                last_frame_time = Instant::now();
            }

            // Render the canvas to a buffer
            let mut buffer: Vec<u32> = Self::canvas_to_buffer(canvas);

            if hover_enabled {
                if let Some(mouse_pos) = window.get_mouse_pos(MouseMode::Pass) {
                    let (mouse_x, mouse_y) = (mouse_pos.0 as u32, mouse_pos.1 as u32);

                    if let Some(updated_buffer) = plot.handle_hover(mouse_x, mouse_y, canvas) {
                        buffer = updated_buffer;
                    }
                }
            }

            if show_hints {
                Self::render_hints(canvas);
            }

            if window.is_key_pressed(Key::C, minifb::KeyRepeat::No) {
                hover_enabled = !hover_enabled;
            }

            if window.is_key_pressed(Key::H, minifb::KeyRepeat::No) {
                show_hints = !show_hints;
            }

            window.update_with_buffer(&buffer, width, height).unwrap();
        }
    }

    fn canvas_to_buffer(canvas: &Canvas) -> Vec<u32> {
        canvas
            .buffer
            .chunks_exact(3)
            .map(|rgb| {
                let r = rgb[0] as u32;
                let g = rgb[1] as u32;
                let b = rgb[2] as u32;
                (r << 16) | (g << 8) | b
            })
            .collect()
    }

    /// Displays the plot in an interactive window with hover functionality
    pub fn display_interactive<T: HoverablePlot>(canvas: &mut Canvas, plot: &T, title: &str) {
        let width = canvas.width as usize;
        let height = canvas.height as usize;

        let mut window = Window::new(
            title,
            width,
            height,
            WindowOptions {
                resize: true,
                scale: minifb::Scale::X1,
                ..WindowOptions::default()
            },
        )
        .unwrap_or_else(|e| panic!("Unable to open Window: {}", e));

        let mut hover_enabled = false;
        let mut show_hints = false;

        while window.is_open() && !window.is_key_pressed(Key::Escape, minifb::KeyRepeat::No) {
            // Render the canvas to a buffer
            let mut buffer: Vec<u32> = Self::canvas_to_buffer(canvas);

            if show_hints {
                Self::render_hints(canvas);
            }

            // Handle hover functionality
            if hover_enabled {
                if let Some(mouse_pos) = window.get_mouse_pos(MouseMode::Pass) {
                    let (mouse_x, mouse_y) = (mouse_pos.0 as u32, mouse_pos.1 as u32);

                    if let Some(updated_buffer) = plot.handle_hover(mouse_x, mouse_y, canvas) {
                        buffer = updated_buffer;
                    }
                }
            }

            // Handle key events
            if window.is_key_pressed(Key::H, minifb::KeyRepeat::No) {
                show_hints = !show_hints;
            }

            if window.is_key_pressed(Key::C, minifb::KeyRepeat::No) {
                hover_enabled = !hover_enabled;
            }

            // Update the window with the buffer
            window.update_with_buffer(&buffer, width, height).unwrap();
        }
    }

    /// Renders hints on the canvas
    fn render_hints(_canvas: &mut Canvas) {

        // let hints = vec![
        //     "H: Toggle Hints",
        //     "C: Toggle Hover Mode",
        //     "NumPad +: Zoom In",
        //     "NumPad -: Zoom Out",
        //     "Escape: Exit",
        // ];

        // let font = include_bytes!("../../resources/fonts/Arial.ttf"); // Path to your font
        // let font_ref = ab_glyph::FontRef::try_from_slice(font).unwrap();
        // let scale = ab_glyph::PxScale { x: 12.0, y: 12.0 };

        // let mut y_offset = 20; // Start slightly down from the top margin
        // let x_offset = 10;     // Fixed X position for all hints

        // // Draw each hint on the canvas
        // for hint in hints {
        //     draw_text_mut(
        //         &mut ImageBuffer::from_raw(canvas.width, canvas.height, canvas.buffer.clone())
        //             .unwrap(),
        //         image::Rgb([0, 0, 0]), // Text color
        //         x_offset,
        //         y_offset,
        //         scale,
        //         &font_ref,
        //         hint,
        //     );

        //     y_offset += 20; // Increment Y position for the next line
        // }
    }
}

/// A trait for plots that support hover functionality
pub trait HoverablePlot {
    fn find_closest_point(
        &self,
        mouse_x: u32,
        mouse_y: u32,
        canvas: &Canvas,
    ) -> Option<((f64, f64), f64)>;

    fn to_canvas_coordinates(&self, x: f64, y: f64, canvas: &Canvas) -> (u32, u32);

    fn get_font(&self) -> ab_glyph::FontRef;

    /// Handle hover functionality and return updated buffer if applicable
    fn handle_hover(&self, mouse_x: u32, mouse_y: u32, canvas: &Canvas) -> Option<Vec<u32>>;
}

impl HoverablePlot for Histogram {
    fn find_closest_point(
        &self,
        mouse_x: u32,
        _mouse_y: u32,
        canvas: &Canvas,
    ) -> Option<((f64, f64), f64)> {
        let bin_data = self.calculate_bins();

        let bin_width = (bin_data[1].0 - bin_data[0].0).abs();
        let scale_x = (canvas.width - 2 * canvas.margin) as f64 / self.bins as f64;

        let mut closest_bin = None;
        let mut min_distance = f64::MAX;

        for (i, &(bin_start, freq)) in bin_data.iter().enumerate() {
            let bin_end = bin_start + bin_width; // End of the bin
            let bin_x = canvas.margin as f64 + i as f64 * scale_x;

            let distance = (mouse_x as f64 - bin_x).abs(); // Distance to mouse x
            if distance < min_distance {
                min_distance = distance;
                closest_bin = Some(((bin_start, bin_end), freq)); // Bin range and frequency
            }
        }

        closest_bin
    }

    fn to_canvas_coordinates(&self, x: f64, y: f64, canvas: &Canvas) -> (u32, u32) {
        let bin_data = self.calculate_bins();
        let bin_width = (bin_data[1].0 - bin_data[0].0).abs();
        let x_min = bin_data[0].0; // Start of the first bin
        let x_max = x_min + bin_width * self.bins as f64;

        let scale_x = (canvas.width - 2 * canvas.margin) as f64 / (x_max - x_min);
        let scale_y = (canvas.height - 2 * canvas.margin) as f64
            / bin_data.iter().map(|(_, f)| *f).fold(0.0, f64::max);

        let px = ((x - x_min) * scale_x + canvas.margin as f64) as u32;
        let py = (canvas.height as f64 - canvas.margin as f64 - y * scale_y) as u32;

        (px, py)
    }

    fn get_font(&self) -> ab_glyph::FontRef {
        ab_glyph::FontRef::try_from_slice(include_bytes!("../../resources/fonts/Arial.ttf"))
            .unwrap()
    }

    /// Handle hover functionality and return updated buffer if applicable
    fn handle_hover(&self, mouse_x: u32, mouse_y: u32, canvas: &Canvas) -> Option<Vec<u32>> {
        // Find the closest bin's range and total value
        if let Some(((bin_start, bin_end), freq)) =
            self.find_closest_point(mouse_x, mouse_y, canvas)
        {
            let mut img =
                ImageBuffer::from_raw(canvas.width, canvas.height, canvas.buffer.clone()).unwrap();

            // Format the text as: ([x-start, x-end], total: y-value)
            let bin_info = format!("([{:.2}, {:.2}], total: {:.2})", bin_start, bin_end, freq);

            // Calculate text size for background rectangle
            let font = self.get_font();
            let scale = ab_glyph::PxScale { x: 12.0, y: 12.0 };
            let text_size = text_size(scale, &font, &bin_info).0 as i32;

            let rect_x = mouse_x as i32 + 15; // Rectangle start X, slightly offset from mouse
            let rect_y = mouse_y as i32 - 20; // Rectangle start Y
            let rect_width = text_size + 20; // Add padding for rectangle width
            let rect_height = 25; // Height of the rectangle

            // Adjust rectangle coordinates to stay within canvas bounds
            let rect_x = rect_x.max(0).min((canvas.width as i32 - rect_width) as i32);
            let rect_y = rect_y
                .max(0)
                .min((canvas.height as i32 - rect_height) as i32);

            // Draw white rectangle as background
            for y in rect_y..(rect_y + rect_height) {
                for x in rect_x..(rect_x + rect_width) {
                    if x >= 0 && y >= 0 && (x as u32) < canvas.width && (y as u32) < canvas.height {
                        img.put_pixel(x as u32, y as u32, image::Rgb([255, 255, 255]));
                        // White
                    }
                }
            }

            // Draw edges for the rectangle
            for x in rect_x..(rect_x + rect_width) {
                if rect_y >= 0 && (x as u32) < canvas.width {
                    img.put_pixel(x as u32, rect_y as u32, image::Rgb([0, 0, 0])); // Top edge
                    img.put_pixel(
                        x as u32,
                        (rect_y + rect_height - 1) as u32,
                        image::Rgb([0, 0, 0]),
                    ); // Bottom edge
                }
            }
            for y in rect_y..(rect_y + rect_height) {
                if rect_x >= 0 && (y as u32) < canvas.height {
                    img.put_pixel(rect_x as u32, y as u32, image::Rgb([0, 0, 0])); // Left edge
                    img.put_pixel(
                        (rect_x + rect_width - 1) as u32,
                        y as u32,
                        image::Rgb([0, 0, 0]),
                    ); // Right edge
                }
            }

            // Draw the formatted text on top of the rectangle
            draw_text_mut(
                &mut img,
                image::Rgb([0, 0, 0]), // Text color
                rect_x + 10,           // Add padding to position text
                rect_y + 5,
                scale,
                &font,
                &bin_info,
            );

            // Draw a line from the bin's top point to the mouse location
            if let Some((bin_px, bin_py)) = self
                .to_canvas_coordinates((bin_start + bin_end) / 2.0, freq, canvas)
                .into()
            {
                draw_line_segment_mut(
                    &mut img,
                    (bin_px as f32, bin_py as f32),
                    (mouse_x as f32, mouse_y as f32),
                    image::Rgb([255, 0, 0]), // Line color
                );
            }

            return Some(
                img.pixels()
                    .map(|pixel: &image::Rgb<u8>| {
                        let [r, g, b] = pixel.0;
                        (r as u32) << 16 | (g as u32) << 8 | b as u32
                    })
                    .collect(),
            );
        }
        None
    }
}

impl HoverablePlot for CartesianGraph {
    fn handle_hover(&self, mouse_x: u32, mouse_y: u32, canvas: &Canvas) -> Option<Vec<u32>> {
        if let Some(((x, y), value)) = self.find_closest_point(mouse_x, mouse_y, canvas) {
            let mut img =
                ImageBuffer::from_raw(canvas.width, canvas.height, canvas.buffer.clone()).unwrap();

            let font = self.get_font();
            let scale = ab_glyph::PxScale { x: 12.0, y: 12.0 };
            let coord_text = format!("({:.2}, {:.2}) = {:.2}", x, y, value);
            let text_size = text_size(scale, &font, &coord_text).0 as i32;

            let rect_x = mouse_x as i32 + 10;
            let rect_y = mouse_y as i32 - 30;
            let rect_width = text_size + 20;
            let rect_height = 25;

            let rect_x = rect_x.max(0).min((canvas.width as i32 - rect_width) as i32);
            let rect_y = rect_y
                .max(0)
                .min((canvas.height as i32 - rect_height) as i32);

            for y in rect_y..(rect_y + rect_height) {
                for x in rect_x..(rect_x + rect_width) {
                    if x >= 0 && y >= 0 && (x as u32) < canvas.width && (y as u32) < canvas.height {
                        img.put_pixel(x as u32, y as u32, image::Rgb([255, 255, 255]));
                    }
                }
            }

            for x in rect_x..(rect_x + rect_width) {
                if rect_y >= 0 && (x as u32) < canvas.width {
                    img.put_pixel(x as u32, rect_y as u32, image::Rgb([0, 0, 0]));
                    img.put_pixel(
                        x as u32,
                        (rect_y + rect_height - 1) as u32,
                        image::Rgb([0, 0, 0]),
                    );
                }
            }

            for y in rect_y..(rect_y + rect_height) {
                if rect_x >= 0 && (y as u32) < canvas.height {
                    img.put_pixel(rect_x as u32, y as u32, image::Rgb([0, 0, 0]));
                    img.put_pixel(
                        (rect_x + rect_width - 1) as u32,
                        y as u32,
                        image::Rgb([0, 0, 0]),
                    );
                }
            }

            draw_text_mut(
                &mut img,
                image::Rgb([0, 0, 0]),
                rect_x + 10,
                rect_y + 5,
                scale,
                &font,
                &coord_text,
            );

            let (px, py) = self.to_canvas_coordinates(x, y, canvas);
            draw_line_segment_mut(
                &mut img,
                (px as f32, py as f32),
                (mouse_x as f32, mouse_y as f32),
                image::Rgb([255, 0, 0]), // Line color
            );

            return Some(
                img.pixels()
                    .map(|pixel: &image::Rgb<u8>| {
                        let [r, g, b] = pixel.0;
                        (r as u32) << 16 | (g as u32) << 8 | b as u32
                    })
                    .collect(),
            );
        }
        None
    }

    fn find_closest_point(
        &self,
        mouse_x: u32,
        mouse_y: u32,
        canvas: &Canvas,
    ) -> Option<((f64, f64), f64)> {
        let scale_x = (canvas.width - 2 * canvas.margin) as f64 / (self.x_max - self.x_min);
        let scale_y = (canvas.height - 2 * canvas.margin) as f64 / (self.y_max - self.y_min);

        self.datasets
            .iter()
            .flat_map(|dataset| {
                dataset.points.iter().map(move |&(x, y)| {
                    let px = canvas.margin as f64 + (x - self.x_min) * scale_x;
                    let py =
                        canvas.height as f64 - canvas.margin as f64 - (y - self.y_min) * scale_y;
                    let dist =
                        ((mouse_x as f64 - px).powi(2) + (mouse_y as f64 - py).powi(2)).sqrt();
                    ((x, y), dist)
                })
            })
            .min_by(|&(_, d1), &(_, d2)| d1.partial_cmp(&d2).unwrap())
            .map(|((x, y), _)| ((x, y), y))
    }

    fn to_canvas_coordinates(&self, x: f64, y: f64, canvas: &Canvas) -> (u32, u32) {
        let scale_x = (canvas.width - 2 * canvas.margin) as f64 / (self.x_max - self.x_min);
        let scale_y = (canvas.height - 2 * canvas.margin) as f64 / (self.y_max - self.y_min);

        let px = ((x - self.x_min) * scale_x + canvas.margin as f64) as u32;
        let py = (canvas.height as f64 - canvas.margin as f64 - (y - self.y_min) * scale_y) as u32;

        (px, py)
    }

    fn get_font(&self) -> ab_glyph::FontRef {
        ab_glyph::FontRef::try_from_slice(include_bytes!("../../resources/fonts/Arial.ttf"))
            .unwrap()
    }
}

impl HoverablePlot for BarChart {
    fn handle_hover(&self, mouse_x: u32, mouse_y: u32, canvas: &Canvas) -> Option<Vec<u32>> {
        let scale_x = (canvas.width - 2 * canvas.margin) as f64 / self.datasets.len() as f64;
        let y_max = self
            .datasets
            .iter()
            .flat_map(|dataset| dataset.data.iter().map(|&(_, y)| y))
            .fold(0.0, f64::max);
        let scale_y = (canvas.height - 2 * canvas.margin) as f64 / y_max;

        let mut img =
            ImageBuffer::from_raw(canvas.width, canvas.height, canvas.buffer.clone()).unwrap();

        let mut closest_bar_group: Option<(f64, Vec<f64>)> = None;
        let mut min_distance = f64::MAX;

        // Find the closest bar group
        for (group_index, _) in self.datasets.iter().enumerate() {
            let group_center_x = canvas.margin as f64 + (group_index as f64 + 0.5) * scale_x;
            let distance = (mouse_x as f64 - group_center_x).abs();

            if distance < min_distance {
                min_distance = distance;

                // Collect values for this group
                let values = self
                    .datasets
                    .iter()
                    .filter_map(|dataset| dataset.data.get(group_index).map(|&(_, value)| value))
                    .collect::<Vec<f64>>();

                closest_bar_group = Some((group_center_x, values));
            }
        }

        if let Some((group_center_x, group_values)) = closest_bar_group {
            // Draw a red line from the center of the bar group to the cursor
            draw_line_segment_mut(
                &mut img,
                (
                    group_center_x as f32,
                    (canvas.height - canvas.margin) as f32,
                ),
                (mouse_x as f32, mouse_y as f32),
                image::Rgb([255, 0, 0]), // Red line
            );

            // Format the tooltip text
            let mut tooltip_text = String::from("X Value: ");

            // Calculate tooltip dimensions
            let font = self.get_font();
            let scale = ab_glyph::PxScale { x: 12.0, y: 12.0 };
            let text_size = text_size(scale, &font, &tooltip_text).0 as i32;

            let rect_x = mouse_x as i32 + 10;
            let rect_y = mouse_y as i32 - 30;
            let rect_width = text_size + 20;
            let rect_height = 25 * (group_values.len() as i32 + 1);

            // Ensure tooltip stays within bounds
            let rect_x = rect_x.max(0).min((canvas.width as i32 - rect_width) as i32);
            let mut rect_y = rect_y
                .max(0)
                .min((canvas.height as i32 - rect_height) as i32);

            // Draw tooltip background
            for y in rect_y..(rect_y + rect_height) {
                for x in rect_x..(rect_x + rect_width) {
                    if x >= 0 && y >= 0 && (x as u32) < canvas.width && (y as u32) < canvas.height {
                        img.put_pixel(x as u32, y as u32, image::Rgb([255, 255, 255]));
                    }
                }
            }

            // Draw tooltip border
            for x in rect_x..(rect_x + rect_width) {
                img.put_pixel(x as u32, rect_y as u32, image::Rgb([0, 0, 0])); // Top edge
                img.put_pixel(
                    x as u32,
                    (rect_y + rect_height - 1) as u32,
                    image::Rgb([0, 0, 0]),
                ); // Bottom edge
            }
            for y in rect_y..(rect_y + rect_height) {
                img.put_pixel(rect_x as u32, y as u32, image::Rgb([0, 0, 0])); // Left edge
                img.put_pixel(
                    (rect_x + rect_width - 1) as u32,
                    y as u32,
                    image::Rgb([0, 0, 0]),
                ); // Right edge
            }

            for (dataset_index, value) in group_values.iter().enumerate() {
                if dataset_index == 0 {
                    tooltip_text.push_str(&format!("{:.2}", value));
                } else {
                    let dataset_label = &self.datasets[dataset_index].label;
                    tooltip_text = format!("{} : {:.2}", dataset_label, value);
                }

                // Draw the tooltip text
                draw_text_mut(
                    &mut img,
                    image::Rgb([0, 0, 0]), // Text color
                    rect_x + 10,
                    rect_y + 5,
                    scale,
                    &font,
                    &tooltip_text,
                );

                rect_y += 20;
            }

            Some(
                img.pixels()
                    .map(|pixel: &image::Rgb<u8>| {
                        let [r, g, b] = pixel.0;
                        (r as u32) << 16 | (g as u32) << 8 | b as u32
                    })
                    .collect(),
            )
        } else {
            None
        }
    }

    fn find_closest_point(
        &self,
        mouse_x: u32,
        mouse_y: u32,
        canvas: &Canvas,
    ) -> Option<((f64, f64), f64)> {
        let scale_x = (canvas.width - 2 * canvas.margin) as f64 / self.datasets.len() as f64;
        let scale_y = (canvas.height - 2 * canvas.margin) as f64 / 10.0; // Assume a fixed y-range for now

        let mut closest_bar = None;
        let mut min_distance = f64::MAX;

        for dataset in &self.datasets {
            for &(x, y) in &dataset.data {
                let px = ((x) * scale_x + canvas.margin as f64) as f64;
                let py = (canvas.height as f64 - canvas.margin as f64 - y * scale_y) as f64;

                let distance =
                    ((mouse_x as f64 - px).powi(2) + (mouse_y as f64 - py).powi(2)).sqrt();
                if distance < min_distance {
                    min_distance = distance;
                    closest_bar = Some(((x, y), y));
                }
            }
        }

        closest_bar
    }

    fn to_canvas_coordinates(&self, x: f64, y: f64, canvas: &Canvas) -> (u32, u32) {
        let scale_x = (canvas.width - 2 * canvas.margin) as f64 / self.datasets.len() as f64;
        let scale_y = (canvas.height - 2 * canvas.margin) as f64 / 10.0; // Assume a fixed y-range for now

        let px = ((x) * scale_x + canvas.margin as f64) as u32;
        let py = (canvas.height as f64 - canvas.margin as f64 - y * scale_y) as u32;

        (px, py)
    }

    fn get_font(&self) -> ab_glyph::FontRef {
        ab_glyph::FontRef::try_from_slice(include_bytes!("../../resources/fonts/Arial.ttf"))
            .unwrap()
    }
}

impl HoverablePlot for PieChart {
    fn find_closest_point(
        &self,
        mouse_x: u32,
        mouse_y: u32,
        canvas: &Canvas,
    ) -> Option<((f64, f64), f64)> {
        let center_x = canvas.width as f64 / 2.0;
        let center_y = canvas.height as f64 / 2.0;
        let total_value: f64 = self.datasets.iter().map(|(_, value, _)| *value).sum();
        let mut start_angle = 0.0;

        let dx = mouse_x as f64 - center_x;
        let dy = mouse_y as f64 - center_y;
        let mut angle = dy.atan2(dx);
        if angle < 0.0 {
            angle += 2.0 * std::f64::consts::PI;
        }

        for (label, value, _) in &self.datasets {
            let sweep_angle = (value / total_value) * 2.0 * std::f64::consts::PI;
            let mid_angle = start_angle + sweep_angle / 2.0;

            if angle >= start_angle && angle < start_angle + sweep_angle {
                let x = center_x + mid_angle.cos() * (canvas.width as f64 / 4.0);
                let y = center_y + mid_angle.sin() * (canvas.height as f64 / 4.0);
                return Some(((x, y), *value));
            }
            start_angle += sweep_angle;
        }

        None
    }

    fn to_canvas_coordinates(&self, x: f64, y: f64, _canvas: &Canvas) -> (u32, u32) {
        (x as u32, y as u32)
    }

    fn get_font(&self) -> ab_glyph::FontRef {
        ab_glyph::FontRef::try_from_slice(include_bytes!("../../resources/fonts/Arial.ttf"))
            .unwrap()
    }

    fn handle_hover(&self, mouse_x: u32, mouse_y: u32, canvas: &Canvas) -> Option<Vec<u32>> {
        let mut img =
            ImageBuffer::from_raw(canvas.width, canvas.height, canvas.buffer.clone()).unwrap();

        if let Some(((x, y), value)) = self.find_closest_point(mouse_x, mouse_y, canvas) {
            // Draw the line from slice center to cursor
            draw_line_segment_mut(
                &mut img,
                (x as f32, y as f32),
                (mouse_x as f32, mouse_y as f32),
                image::Rgb([255, 0, 0]),
            );

            // Draw an edged rectangle for the hover information
            let font = self.get_font();
            let scale = ab_glyph::PxScale { x: 12.0, y: 12.0 };
            let coord_text = format!("{}: {:.2}", self.title, value);
            let text_size = text_size(scale, &font, &coord_text).0 as i32;

            let rect_x = mouse_x as i32 + 10;
            let rect_y = mouse_y as i32 - 30;
            let rect_width = text_size + 20;
            let rect_height = 25;

            let rect_x = rect_x.max(0).min((canvas.width as i32 - rect_width) as i32);
            let rect_y = rect_y
                .max(0)
                .min((canvas.height as i32 - rect_height) as i32);

            for y in rect_y..(rect_y + rect_height) {
                for x in rect_x..(rect_x + rect_width) {
                    if x >= 0 && y >= 0 && (x as u32) < canvas.width && (y as u32) < canvas.height {
                        img.put_pixel(x as u32, y as u32, image::Rgb([255, 255, 255]));
                    }
                }
            }

            for x in rect_x..(rect_x + rect_width) {
                if rect_y >= 0 && (x as u32) < canvas.width {
                    img.put_pixel(x as u32, rect_y as u32, image::Rgb([0, 0, 0]));
                    img.put_pixel(
                        x as u32,
                        (rect_y + rect_height - 1) as u32,
                        image::Rgb([0, 0, 0]),
                    );
                }
            }

            for y in rect_y..(rect_y + rect_height) {
                if rect_x >= 0 && (y as u32) < canvas.height {
                    img.put_pixel(rect_x as u32, y as u32, image::Rgb([0, 0, 0]));
                    img.put_pixel(
                        (rect_x + rect_width - 1) as u32,
                        y as u32,
                        image::Rgb([0, 0, 0]),
                    );
                }
            }

            draw_text_mut(
                &mut img,
                image::Rgb([0, 0, 0]),
                rect_x + 10,
                rect_y + 5,
                scale,
                &font,
                &coord_text,
            );
        }

        Some(
            img.pixels()
                .map(|pixel: &image::Rgb<u8>| {
                    let [r, g, b] = pixel.0;
                    (r as u32) << 16 | (g as u32) << 8 | b as u32
                })
                .collect(),
        )
    }
}

impl HoverablePlot for ScatterGraph {
    fn find_closest_point(
        &self,
        mouse_x: u32,
        mouse_y: u32,
        canvas: &Canvas,
    ) -> Option<((f64, f64), f64)> {
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

        self.datasets
            .iter()
            .flat_map(|dataset| {
                dataset.points.iter().map(|&(x, y)| {
                    let px = canvas.margin as f64 + (x - x_min) * scale_x;
                    let py = canvas.height as f64 - canvas.margin as f64 - (y - y_min) * scale_y;
                    let dist =
                        ((mouse_x as f64 - px).powi(2) + (mouse_y as f64 - py).powi(2)).sqrt();
                    ((x, y), dist)
                })
            })
            .min_by(|&(_, d1), &(_, d2)| d1.partial_cmp(&d2).unwrap())
    }

    fn to_canvas_coordinates(&self, x: f64, y: f64, canvas: &Canvas) -> (u32, u32) {
        // todo!(); add max min values to scatter graph
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

        let px = ((x - x_min) * scale_x + canvas.margin as f64) as u32;
        let py = (canvas.height as f64 - canvas.margin as f64 - (y - y_min) * scale_y) as u32;

        (px, py)
    }

    fn get_font(&self) -> ab_glyph::FontRef {
        ab_glyph::FontRef::try_from_slice(include_bytes!("../../resources/fonts/Arial.ttf"))
            .unwrap()
    }

    fn handle_hover(&self, mouse_x: u32, mouse_y: u32, canvas: &Canvas) -> Option<Vec<u32>> {
        let mut img =
            ImageBuffer::from_raw(canvas.width, canvas.height, canvas.buffer.clone()).unwrap();

        if let Some(((x, y), _)) = self.find_closest_point(mouse_x, mouse_y, canvas) {
            let (px, py) = self.to_canvas_coordinates(x, y, canvas);

            // Draw the line from point to cursor
            draw_line_segment_mut(
                &mut img,
                (px as f32, py as f32),
                (mouse_x as f32, mouse_y as f32),
                image::Rgb([255, 0, 0]),
            );

            // Draw an edged rectangle for the hover information
            let font = self.get_font();
            let scale = ab_glyph::PxScale { x: 12.0, y: 12.0 };
            let coord_text = format!("({:.2}, {:.2})", x, y);
            let text_size = text_size(scale, &font, &coord_text).0 as i32;

            let rect_x = mouse_x as i32 + 10;
            let rect_y = mouse_y as i32 - 30;
            let rect_width = text_size + 20;
            let rect_height = 25;

            let rect_x = rect_x.max(0).min((canvas.width as i32 - rect_width) as i32);
            let rect_y = rect_y
                .max(0)
                .min((canvas.height as i32 - rect_height) as i32);

            for y in rect_y..(rect_y + rect_height) {
                for x in rect_x..(rect_x + rect_width) {
                    if x >= 0 && y >= 0 && (x as u32) < canvas.width && (y as u32) < canvas.height {
                        img.put_pixel(x as u32, y as u32, image::Rgb([255, 255, 255]));
                    }
                }
            }

            for x in rect_x..(rect_x + rect_width) {
                if rect_y >= 0 && (x as u32) < canvas.width {
                    img.put_pixel(x as u32, rect_y as u32, image::Rgb([0, 0, 0]));
                    img.put_pixel(
                        x as u32,
                        (rect_y + rect_height - 1) as u32,
                        image::Rgb([0, 0, 0]),
                    );
                }
            }

            for y in rect_y..(rect_y + rect_height) {
                if rect_x >= 0 && (y as u32) < canvas.height {
                    img.put_pixel(rect_x as u32, y as u32, image::Rgb([0, 0, 0]));
                    img.put_pixel(
                        (rect_x + rect_width - 1) as u32,
                        y as u32,
                        image::Rgb([0, 0, 0]),
                    );
                }
            }

            draw_text_mut(
                &mut img,
                image::Rgb([0, 0, 0]),
                rect_x + 10,
                rect_y + 5,
                scale,
                &font,
                &coord_text,
            );
        }

        Some(
            img.pixels()
                .map(|pixel: &image::Rgb<u8>| {
                    let [r, g, b] = pixel.0;
                    (r as u32) << 16 | (g as u32) << 8 | b as u32
                })
                .collect(),
        )
    }
}

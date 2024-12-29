use std::{thread, time::Duration};

use crate::plot::canvas::Canvas;
use image::ImageBuffer;
use imageproc::drawing::{draw_line_segment_mut, draw_text_mut, text_size};
use minifb::{Key, MouseMode, Window, WindowOptions};

use super::{
    barchart::BarChart, cartesiangraph::CartesianGraph, drawer::Drawer, historgram::Histogram, linetype::LineType, piechart::PieChart, scattergraph::ScatterGraph
};

pub struct Winop;

impl Winop {
    pub fn new() -> Self {
        Self
    }

    /// Displays the plot in real-time with continuous updates
    pub fn display_real_time<F, T>(
        canvas: &mut Canvas,
        plot: &mut T,
        title: &str,
        mut update_data: F,
        fps: u64,
    ) where
        F: FnMut(&mut T),
        T: Drawer + HoverablePlot,
    {
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
            // Update plot data
            update_data(plot);
    
            // Clear the canvas and redraw the plot
            canvas.clear();
            plot.draw(canvas);
    
            // Handle hover functionality
            if hover_enabled {
                if let Some(mouse_pos) = window.get_mouse_pos(MouseMode::Pass) {
                    let (mouse_x, mouse_y) = (mouse_pos.0 as u32, mouse_pos.1 as u32);

                    // Find the closest bin's range and total value
                    if let Some(((bin_start, bin_end), freq)) =
                        plot.find_closest_point(mouse_x, mouse_y, canvas)
                    {
                        let mut img = ImageBuffer::from_raw(
                            canvas.width,
                            canvas.height,
                            canvas.buffer.clone(),
                        )
                        .unwrap();

                        // Format the text as: ([x-start, x-end], total: y-value)
                        let bin_info =
                            format!("([{:.2}, {:.2}], total: {:.2})", bin_start, bin_end, freq);

                        // Calculate text size for background rectangle
                        let font = plot.get_font();
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
                                if x >= 0
                                    && y >= 0
                                    && (x as u32) < canvas.width
                                    && (y as u32) < canvas.height
                                {
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
                        if let Some((bin_px, bin_py)) = plot
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
    
            // Convert canvas buffer to the window's buffer format
            let buffer: Vec<u32> = canvas
                .buffer
                .chunks_exact(3)
                .map(|rgb| {
                    let r = rgb[0] as u32;
                    let g = rgb[1] as u32;
                    let b = rgb[2] as u32;
                    (r << 16) | (g << 8) | b // Convert RGB to u32
                })
                .collect();
    
            // Update the window with the new buffer
            window.update_with_buffer(&buffer, width, height).unwrap();
    
            // Control frame rate
            thread::sleep(Duration::from_millis(1000 / fps));
        }
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

                    // Find the closest bin's range and total value
                    if let Some(((bin_start, bin_end), freq)) =
                        plot.find_closest_point(mouse_x, mouse_y, canvas)
                    {
                        let mut img = ImageBuffer::from_raw(
                            canvas.width,
                            canvas.height,
                            canvas.buffer.clone(),
                        )
                        .unwrap();

                        // Format the text as: ([x-start, x-end], total: y-value)
                        let bin_info =
                            format!("([{:.2}, {:.2}], total: {:.2})", bin_start, bin_end, freq);

                        // Calculate text size for background rectangle
                        let font = plot.get_font();
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
                                if x >= 0
                                    && y >= 0
                                    && (x as u32) < canvas.width
                                    && (y as u32) < canvas.height
                                {
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
                        if let Some((bin_px, bin_py)) = plot
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

                        // Update buffer with the modified image
                        buffer = img
                            .pixels()
                            .map(|pixel: &image::Rgb<u8>| {
                                let [r, g, b] = pixel.0;
                                (r as u32) << 16 | (g as u32) << 8 | (b as u32)
                            })
                            .collect();
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

    /// Converts Canvas buffer to a format usable by minifb
    fn canvas_to_buffer(canvas: &Canvas) -> Vec<u32> {
        canvas
            .buffer
            .chunks_exact(3)
            .map(|rgb| {
                let r = rgb[0] as u32;
                let g = rgb[1] as u32;
                let b = rgb[2] as u32;
                (r << 16) | (g << 8) | b // Convert RGB to u32
            })
            .collect()
    }

    /// Renders hints on the canvas
    fn render_hints(_canvas: &mut Canvas) {
        // todo!();

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
}

impl HoverablePlot for CartesianGraph {
    fn find_closest_point(
        &self,
        mouse_x: u32,
        mouse_y: u32,
        canvas: &Canvas,
    ) -> Option<((f64, f64), f64)> {
        let scale_x = (canvas.width - 2 * canvas.margin) as f64 / (self.x_max - self.x_min);
        let scale_y = (canvas.height - 2 * canvas.margin) as f64 / 10.0; // Assume a fixed y-range for now

        let mut closest_point = None;
        let mut min_distance = f64::MAX;

        for dataset in &self.datasets {
            for &(x, y) in &dataset.points {
                let px = ((x - self.x_min) * scale_x + canvas.margin as f64) as f64;
                let py = (canvas.height as f64 - canvas.margin as f64 - y * scale_y) as f64;

                let distance =
                    ((mouse_x as f64 - px).powi(2) + (mouse_y as f64 - py).powi(2)).sqrt();
                if distance < min_distance {
                    min_distance = distance;
                    closest_point = Some(((x, y), y));
                }
            }
        }

        closest_point
    }

    fn to_canvas_coordinates(&self, x: f64, y: f64, canvas: &Canvas) -> (u32, u32) {
        let scale_x = (canvas.width - 2 * canvas.margin) as f64 / (self.x_max - self.x_min);
        let scale_y = (canvas.height - 2 * canvas.margin) as f64 / 10.0; // Assume a fixed y-range for now

        let px = ((x - self.x_min) * scale_x + canvas.margin as f64) as u32;
        let py = (canvas.height as f64 - canvas.margin as f64 - y * scale_y) as u32;

        (px, py)
    }

    fn get_font(&self) -> ab_glyph::FontRef {
        ab_glyph::FontRef::try_from_slice(include_bytes!("../../resources/fonts/Arial.ttf"))
            .unwrap()
    }
}

impl HoverablePlot for BarChart {
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
        // Center and radius of the pie chart
        let center_x = canvas.width as f64 / 2.0;
        let center_y = canvas.height as f64 / 2.0;
        let radius = (canvas.width.min(canvas.height) / 2 - canvas.margin) as f64;

        // Calculate relative mouse position to the center
        let dx = mouse_x as f64 - center_x;
        let dy = center_y - mouse_y as f64; // Y-axis is inverted for drawing

        // Check if the mouse is within the circle's radius
        let distance = (dx.powi(2) + dy.powi(2)).sqrt();
        if distance > radius {
            return None;
        }

        // Calculate the angle of the mouse position
        let angle = dy.atan2(dx).to_degrees();
        let normalized_angle = if angle < 0.0 { angle + 360.0 } else { angle };

        // Identify the slice containing this angle
        let mut start_angle = 0.0;
        let total: f64 = self.datasets.iter().map(|(_, value, _)| value).sum();

        for (label, value, _color) in &self.datasets {
            let sweep_angle = 360.0 * (value / total);
            if normalized_angle >= start_angle && normalized_angle < start_angle + sweep_angle {
                return Some(((normalized_angle, *value), *value)); // Return angle and value
            }
            start_angle += sweep_angle;
        }

        None
    }

    fn to_canvas_coordinates(&self, x: f64, y: f64, canvas: &Canvas) -> (u32, u32) {
        // Map x and y values to canvas coordinates
        let center_x = canvas.width as f64 / 2.0;
        let center_y = canvas.height as f64 / 2.0;
        let radius = (canvas.width.min(canvas.height) / 2 - canvas.margin) as f64;

        let px = center_x + x / 360.0 * radius * 0.8; // Adjusted radius for hover text
        let py = center_y - y / 360.0 * radius * 0.8; // Y-axis inverted for drawing

        (px as u32, py as u32)
    }

    fn get_font(&self) -> ab_glyph::FontRef {
        ab_glyph::FontRef::try_from_slice(include_bytes!("../../resources/fonts/Arial.ttf"))
            .unwrap()
    }
}


impl HoverablePlot for ScatterGraph {
    fn find_closest_point(
        &self,
        mouse_x: u32,
        mouse_y: u32,
        canvas: &Canvas,
    ) -> Option<((f64, f64), f64)> {
        // Calculate scaling factors
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

        let scale_x = (canvas.width - 2 * canvas.margin) as f64 / (x_max - x_min);
        let scale_y = (canvas.height - 2 * canvas.margin) as f64 / (y_max - y_min);

        let mut closest_point = None;
        let mut min_distance = f64::MAX;

        for dataset in &self.datasets {
            for &(x, y) in &dataset.points {
                let px = ((x - x_min) * scale_x + canvas.margin as f64) as f64;
                let py = (canvas.height as f64 - canvas.margin as f64 - (y - y_min) * scale_y) as f64;

                let distance = ((mouse_x as f64 - px).powi(2) + (mouse_y as f64 - py).powi(2)).sqrt();
                if distance < min_distance {
                    min_distance = distance;
                    closest_point = Some(((x, y), y));
                }
            }
        }

        closest_point
    }

    fn to_canvas_coordinates(&self, x: f64, y: f64, canvas: &Canvas) -> (u32, u32) {
        // Calculate scaling factors
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
}

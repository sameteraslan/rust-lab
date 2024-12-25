use crate::plot::styles::LineStyle;
use ab_glyph::FontRef;
use imageproc::drawing::{draw_line_segment_mut, draw_text_mut};
use minifb::{Key, MouseMode, Window, WindowOptions};
use std::thread;
use std::time::Duration;

pub struct Plot {
    pub width: u32,
    pub height: u32,
    pub title: String,
    pub xlabel: String,
    pub ylabel: String,
    pub datasets: Vec<(Vec<(f64, f64)>, [u8; 3], String, LineStyle)>,
    pub background_color: [u8; 3],
    pub front_color: [u8; 3],
    pub grid_color: [u8; 3],
    pub font_color: [u8; 3],
    pub title_color: [u8; 3],
    pub line_thickness: u32,
    pub x_min: f64,
    pub x_max: f64,
    pub y_min: f64,
    pub y_max: f64,
    pub margin: u32,
    pub font_path: String,
}

impl Plot {
    pub fn new() -> Self {
        Self {
            width: 800,
            height: 600,
            title: String::new(),
            xlabel: String::new(),
            ylabel: String::new(),
            datasets: Vec::new(),
            background_color: [255, 255, 255],
            front_color: [0, 0, 0],
            grid_color: [220, 220, 220],
            font_color: [0, 0, 0],
            title_color: [0, 0, 0],
            line_thickness: 1,
            x_min: -100.0,
            x_max: 100.0,
            y_min: -100.0,
            y_max: 100.0,
            margin: 100,
            font_path: String::from("../../resources/fonts/Arial.ttf"),
        }
    }

    // Adds a dataset to the plot with a custom line color
    pub fn add_dataset(
        mut self,
        data: Vec<(f64, f64)>,
        line_color: [u8; 3],
        label: &str,
        line_style: LineStyle,
    ) -> Self {
        self.datasets
            .push((data, line_color, label.to_string(), line_style));
        self
    }

    
    pub fn font_path(mut self, font_path: &str) -> Self {
        self.font_path = font_path.to_string();
        self
    }

    // Sets the margin around the plot in pixels
    pub fn margin(mut self, margin: u32) -> Self {
        self.margin = margin;
        self
    }

    // Sets the maximum y value and adjusts the minimum y value to maintain symmetry
    pub fn y_max(mut self, y_max: f64) -> Self {
        self.y_max = y_max;
        self.y_min = -y_max;
        self
    }

    // Sets the maximum x value and adjusts the minimum x value to maintain symmetry
    pub fn x_max(mut self, x_max: f64) -> Self {
        self.x_max = x_max;
        self.x_min = -x_max;
        self
    }

    // Sets the line thickness for data lines
    pub fn line_thickness(mut self, line_thickness: u32) -> Self {
        self.line_thickness = line_thickness;
        self
    }

    // Sets the color of the front elements (axes, labels, etc.)
    pub fn front_color(mut self, front_color: &[u8; 3]) -> Self {
        self.front_color = *front_color;
        self
    }

    // Sets
    pub fn title_color(mut self, title_color: &[u8; 3]) -> Self {
        self.title_color = *title_color;
        self
    }

    // Sets
    pub fn grid_color(mut self, grid_color: &[u8; 3]) -> Self {
        self.grid_color = *grid_color;
        self
    }

    // Sets the color of the front elements (axes, labels, etc.)
    pub fn font_color(mut self, font_color: &[u8; 3]) -> Self {
        self.font_color = *font_color;
        self
    }

    // Sets the background color of the plot
    pub fn background_color(mut self, background_color: &[u8; 3]) -> Self {
        self.background_color = *background_color;
        self
    }

    // Sets the height of the plot in pixels
    pub fn height(mut self, height: u32) -> Self {
        self.height = height;
        self
    }

    // Sets the width of the plot in pixels
    pub fn width(mut self, width: u32) -> Self {
        self.width = width;
        self
    }

    // Sets the title of the plot
    pub fn title(mut self, title: &str) -> Self {
        self.title = title.to_string();
        self
    }

    // Sets the label for the x-axis
    pub fn xlabel(mut self, label: &str) -> Self {
        self.xlabel = label.to_string();
        self
    }

    // Sets the label for the y-axis
    pub fn ylabel(mut self, label: &str) -> Self {
        self.ylabel = label.to_string();
        self
    }

    // Displays the plot's title and axis labels in the console
    pub fn show(&self) {
        println!("Plot: {}", self.title);
        println!("X-Axis: {}", self.xlabel);
        println!("Y-Axis: {}", self.ylabel);
    }

    // Displays the plot in a separate window
    pub fn display_with_window(&self) {
        let img = self.render_with_grid(); // Render the plot with grid
        let width = self.width as usize;
        let height = self.height as usize;

        // Convert the RgbImage to a Vec<u32> suitable for minifb
        let buffer: Vec<u32> = img
            .pixels()
            .map(|pixel| {
                let [r, g, b] = pixel.0;
                (r as u32) << 16 | (g as u32) << 8 | (b as u32) // Convert RGB to u32
            })
            .collect();

        // Create a window
        let mut window = Window::new(
            &self.title,
            width,
            height,
            WindowOptions {
                resize: false,
                scale: minifb::Scale::X1,
                ..WindowOptions::default()
            },
        )
        .unwrap_or_else(|e| {
            panic!("Unable to open Window: {}", e);
        });

        // Display the image
        while window.is_open() && !window.is_key_pressed(Key::Escape, minifb::KeyRepeat::No) {
            window.update_with_buffer(&buffer, width, height).unwrap();
        }
    }

    pub fn display(&mut self) {
        let mut window = Window::new(
            &self.title,
            self.width as usize,
            self.height as usize,
            WindowOptions {
                resize: true, // Enable resizing
                scale: minifb::Scale::X1,
                ..WindowOptions::default()
            },
        )
        .unwrap_or_else(|e| panic!("Unable to open Window: {}", e));

        let mut hover_enabled = false; // Track whether hover is enabled
        let mut show_hints = false; // Track whether hints are enabled

        while window.is_open() && !window.is_key_pressed(Key::Escape, minifb::KeyRepeat::No) {
            // Check if the window size has changed and update plot dimensions
            let current_size = window.get_size();
            if current_size != (self.width as usize, self.height as usize) {
                self.width = current_size.0 as u32;
                self.height = current_size.1 as u32;
            }

            let mut img = self.render_with_grid(); // Render the plot with current settings

            // Show hints if enabled
            if show_hints {
                let hints = vec![
                    "H: Toggle Hints",
                    "C: Toggle hover mode",
                    "NumPad +: Zoom in",
                    "NumPad -: Zoom out",
                    "Escape: Exit",
                ];

                let font = self.get_font();
                let mut y_offset = 20; // Start drawing hints slightly down from the top

                for hint in hints {
                    draw_text_mut(
                        &mut img,
                        image::Rgb([0, 0, 0]), // Text color
                        10,                    // X position
                        y_offset,              // Y position
                        ab_glyph::PxScale { x: 12.0, y: 12.0 },
                        &font,
                        hint,
                    );
                    y_offset += 20; // Move to the next line
                }
            }

            // Check if hover is enabled and process mouse events
            if hover_enabled {
                if let Some(mouse_pos) = window.get_mouse_pos(MouseMode::Pass) {
                    let (mouse_x, mouse_y) = (mouse_pos.0 as u32, mouse_pos.1 as u32);

                    // Find the closest point to the mouse
                    if let Some((closest_x, closest_y)) = self.find_closest_point(mouse_x, mouse_y)
                    {
                        // Display the closest point coordinates on the image
                        draw_text_mut(
                            &mut img,
                            image::Rgb([0, 0, 0]), // Text color
                            mouse_x as i32,
                            mouse_y as i32 - 15,
                            ab_glyph::PxScale { x: 12.0, y: 12.0 },
                            &self.get_font(),
                            &format!("({:.2}, {:.2})", closest_x, closest_y),
                        );

                        // Draw an arrow from the closest point to the mouse position
                        let scale_x =
                            (self.x_max - self.x_min) / (self.width - 2 * self.margin) as f64;
                        let scale_y =
                            (self.y_max - self.y_min) / (self.height - 2 * self.margin) as f64;

                        let px = ((closest_x - self.x_min) / scale_x + self.margin as f64) as f32;
                        let py = (self.height as f64
                            - self.margin as f64
                            - (closest_y - self.y_min) / scale_y)
                            as f32;

                        draw_line_segment_mut(
                            &mut img,
                            (px, py),
                            (mouse_x as f32, mouse_y as f32),
                            image::Rgb([255, 0, 0]), // Arrow color
                        );
                    }
                }
            }

            let buffer: Vec<u32> = img
                .pixels()
                .map(|pixel| {
                    let [r, g, b] = pixel.0;
                    (r as u32) << 16 | (g as u32) << 8 | (b as u32) // Convert RGB to u32
                })
                .collect();

            window
                .update_with_buffer(&buffer, self.width as usize, self.height as usize)
                .unwrap();

            // Handle hover toggle with the "H" key
            if window.is_key_pressed(Key::C, minifb::KeyRepeat::No) {
                hover_enabled = !hover_enabled; // Toggle hover mode
            }

            // Handle hints toggle with the "I" key
            if window.is_key_pressed(Key::H, minifb::KeyRepeat::No) {
                show_hints = !show_hints; // Toggle hints display
            }

            // Handle zooming
            if window.is_key_pressed(Key::NumPadPlus, minifb::KeyRepeat::No) {
                self.zoom(0.9); // Zoom in
            }
            if window.is_key_pressed(Key::NumPadMinus, minifb::KeyRepeat::No) {
                self.zoom(1.1); // Zoom out
            }
        }
    }

    // Finds the closest data point to the given pixel position
    fn find_closest_point(&self, mouse_x: u32, mouse_y: u32) -> Option<(f64, f64)> {
        let scale_x = (self.x_max - self.x_min) / (self.width - 2 * self.margin) as f64;
        let scale_y = (self.y_max - self.y_min) / (self.height - 2 * self.margin) as f64;

        let mut closest_point = None;
        let mut min_distance = f64::MAX;

        for (dataset, _, _, _) in &self.datasets {
            for &(x, y) in dataset {
                let px = ((x - self.x_min) / scale_x + self.margin as f64) as u32;
                let py =
                    (self.height as f64 - self.margin as f64 - (y - self.y_min) / scale_y) as u32;

                let distance = ((px as i32 - mouse_x as i32).pow(2)
                    + (py as i32 - mouse_y as i32).pow(2)) as f64;

                if distance < min_distance {
                    min_distance = distance;
                    closest_point = Some((x, y));
                }
            }
        }

        closest_point
    }

    // Retrieves a font for rendering text
    fn get_font(&self) -> ab_glyph::FontRef<'static> {
        FontRef::try_from_slice(include_bytes!("../../resources/fonts/Arial.ttf")).unwrap()
    }

    // Adjusts the zoom level
    fn zoom(&mut self, factor: f64) {
        let x_center = (self.x_min + self.x_max) / 2.0;
        let y_center = (self.y_min + self.y_max) / 2.0;

        let x_range = (self.x_max - self.x_min) * factor;
        let y_range = (self.y_max - self.y_min) * factor;

        self.x_min = x_center - x_range / 2.0;
        self.x_max = x_center + x_range / 2.0;
        self.y_min = y_center - y_range / 2.0;
        self.y_max = y_center + y_range / 2.0;
    }

    pub fn display_real_time<F>(&mut self, mut data_generator: F)
    where
        F: FnMut() -> (f64, f64),
    {
        let mut window = Window::new(
            &self.title,
            self.width as usize,
            self.height as usize,
            WindowOptions {
                resize: true,
                scale: minifb::Scale::X1,
                ..WindowOptions::default()
            },
        )
        .unwrap_or_else(|e| panic!("Unable to open Window: {}", e));

        let mut hover_enabled = false; // Track whether hover is enabled
        let mut show_hints = false; // Track whether hints are enabled

        while window.is_open() && !window.is_key_pressed(Key::Escape, minifb::KeyRepeat::No) {
            // Check if the window size has changed and update plot dimensions
            let current_size = window.get_size();
            if current_size != (self.width as usize, self.height as usize) {
                self.width = current_size.0 as u32;
                self.height = current_size.1 as u32;
            }

            // Generate new data and update the dataset
            let (new_x, new_y) = data_generator();
            if let Some(dataset) = self.datasets.first_mut() {
                dataset.0.push((new_x, new_y));

                // Remove old data points to keep the graph within bounds
                if dataset.0.len() > 500 {
                    dataset.0.remove(0);
                }
            }

            let mut img = self.render_with_grid(); // Render the plot with current settings

            // Show hints if enabled
            if show_hints {
                let hints = vec![
                    "H: Toggle hover mode",
                    "NumPad +: Zoom in",
                    "NumPad -: Zoom out",
                    "Escape: Exit",
                ];

                let font = self.get_font();
                let mut y_offset = 20; // Start drawing hints slightly down from the top

                for hint in hints {
                    draw_text_mut(
                        &mut img,
                        image::Rgb([0, 0, 0]), // Text color
                        10,                    // X position
                        y_offset,              // Y position
                        ab_glyph::PxScale { x: 12.0, y: 12.0 },
                        &font,
                        hint,
                    );
                    y_offset += 20; // Move to the next line
                }
            }

            // Check if hover is enabled and process mouse events
            if hover_enabled {
                if let Some(mouse_pos) = window.get_mouse_pos(MouseMode::Pass) {
                    let (mouse_x, mouse_y) = (mouse_pos.0 as u32, mouse_pos.1 as u32);

                    // Find the closest point to the mouse
                    if let Some((closest_x, closest_y)) = self.find_closest_point(mouse_x, mouse_y)
                    {
                        // Display the closest point coordinates on the image
                        draw_text_mut(
                            &mut img,
                            image::Rgb([0, 0, 0]), // Text color
                            mouse_x as i32,
                            mouse_y as i32 - 15,
                            ab_glyph::PxScale { x: 12.0, y: 12.0 },
                            &self.get_font(),
                            &format!("({:.2}, {:.2})", closest_x, closest_y),
                        );

                        // Draw an arrow from the closest point to the mouse position
                        let scale_x =
                            (self.x_max - self.x_min) / (self.width - 2 * self.margin) as f64;
                        let scale_y =
                            (self.y_max - self.y_min) / (self.height - 2 * self.margin) as f64;

                        let px = ((closest_x - self.x_min) / scale_x + self.margin as f64) as f32;
                        let py = (self.height as f64
                            - self.margin as f64
                            - (closest_y - self.y_min) / scale_y)
                            as f32;

                        draw_line_segment_mut(
                            &mut img,
                            (px, py),
                            (mouse_x as f32, mouse_y as f32),
                            image::Rgb([255, 0, 0]), // Arrow color
                        );
                    }
                }
            }

            let buffer: Vec<u32> = img
                .pixels()
                .map(|pixel| {
                    let [r, g, b] = pixel.0;
                    (r as u32) << 16 | (g as u32) << 8 | (b as u32) // Convert RGB to u32
                })
                .collect();

            window
                .update_with_buffer(&buffer, self.width as usize, self.height as usize)
                .unwrap();

            // Handle hover toggle with the "H" key
            if window.is_key_pressed(Key::C, minifb::KeyRepeat::No) {
                hover_enabled = !hover_enabled; // Toggle hover mode
            }

            // Handle hints toggle with the "I" key
            if window.is_key_pressed(Key::H, minifb::KeyRepeat::No) {
                show_hints = !show_hints; // Toggle hints display
            }

            // Handle zooming
            if window.is_key_pressed(Key::NumPadPlus, minifb::KeyRepeat::No) {
                self.zoom(0.9); // Zoom in
            }
            if window.is_key_pressed(Key::NumPadMinus, minifb::KeyRepeat::No) {
                self.zoom(1.1); // Zoom out
            }

            // Limit update rate to control real-time rendering speed
            thread::sleep(Duration::from_millis(16)); // Approx. 60 FPS
        }
    }
}

use image::{Rgb, RgbImage};

// Represents a 2D plot with customizable properties
pub struct Plot {
    pub width: u32,            // Width of the plot in pixels
    pub height: u32,           // Height of the plot in pixels
    pub title: String,         // Title of the plot
    pub xlabel: String,        // Label for the x-axis
    pub ylabel: String,        // Label for the y-axis
    pub data: Vec<(f64, f64)>, // Data points to plot, stored as (x, y) pairs
    pub bg_color: [u8; 3],     // Background color in RGB
    pub axis_color: [u8; 3],   // Color of the axes in RGB
    pub line_color: [u8; 3],   // Color of the data lines in RGB
    pub line_thickness: u32,   // Thickness of the plotted lines
    x_min: f64,                // Minimum x value for the plot
    pub x_max: f64,            // Maximum x value for the plot
    y_min: f64,                // Minimum y value for the plot
    pub y_max: f64,            // Maximum y value for the plot
    pub margin: u32,           // Margin around the plot in pixels
}

impl Plot {
    // Creates a new Plot instance with default values
    pub fn new() -> Self {
        Self {
            width: 800,
            height: 600,
            title: String::new(),
            xlabel: String::new(),
            ylabel: String::new(),
            data: Vec::new(),
            bg_color: [255, 255, 255], // Default: white background
            axis_color: [0, 0, 0],     // Default: black axes
            line_color: [0, 0, 0],     // Default: black data lines
            line_thickness: 1,         // Default: 1-pixel thick lines
            x_min: -100.0,             // Default x-axis range
            x_max: 100.0,
            y_min: -100.0, // Default y-axis range
            y_max: 100.0,
            margin: 100, // Default margin around the plot
        }
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

    // Sets the color of the data lines
    pub fn line_color(mut self, line_color: &[u8; 3]) -> Self {
        self.line_color = *line_color;
        self
    }

    // Sets the color of the axes
    pub fn axis_color(mut self, axis_color: &[u8; 3]) -> Self {
        self.axis_color = *axis_color;
        self
    }

    // Sets the background color of the plot
    pub fn bg_color(mut self, bg_color: &[u8; 3]) -> Self {
        self.bg_color = *bg_color;
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

    // Adds data points to the plot
    pub fn add_data(mut self, x: &[f64], y: &[f64]) -> Self {
        self.data = x.iter().zip(y).map(|(&x, &y)| (x, y)).collect();
        self
    }

    // Draws the axes on the image
    fn draw_axes(&self, img: &mut RgbImage) {
        // Calculate the center of the axes with respect to the margin
        let center_x = (self.margin + (self.width - 2 * self.margin) / 2) as i32;
        let center_y = (self.margin + (self.height - 2 * self.margin) / 2) as i32;

        // Draw the X-axis (horizontal line at center_y)
        if center_y >= 0 && center_y < self.height as i32 {
            for x in self.margin..(self.width - self.margin) {
                img.put_pixel(x, center_y as u32, Rgb(self.axis_color));
            }
        }

        // Draw the Y-axis (vertical line at center_x)
        if center_x >= 0 && center_x < self.width as i32 {
            for y in self.margin..(self.height - self.margin) {
                img.put_pixel(center_x as u32, y, Rgb(self.axis_color));
            }
        }
    }

    // Draws lines connecting the data points with the specified thickness
    fn draw_lines_with_thickness(&self, img: &mut RgbImage, thickness: u32) {
        if self.data.len() < 2 {
            return; // Skip if there are fewer than 2 points
        }

        // Scaling factors adjusted for margins
        let scale_x = (self.width as f64 - 2.0 * self.margin as f64) / (self.x_max - self.x_min);
        let scale_y = (self.height as f64 - 2.0 * self.margin as f64) / (self.y_max - self.y_min);

        for i in 0..self.data.len() - 1 {
            let (x1, y1) = self.data[i];
            let (x2, y2) = self.data[i + 1];

            // Convert data coordinates to pixel coordinates
            let x1_pixel = ((x1 - self.x_min) * scale_x + self.margin as f64) as i32;
            let y1_pixel =
                (self.height as f64 - self.margin as f64 - ((y1 - self.y_min) * scale_y)) as i32;
            let x2_pixel = ((x2 - self.x_min) * scale_x + self.margin as f64) as i32;
            let y2_pixel =
                (self.height as f64 - self.margin as f64 - ((y2 - self.y_min) * scale_y)) as i32;

            self.draw_thick_line(img, x1_pixel, y1_pixel, x2_pixel, y2_pixel, thickness);
        }
    }

    // Draws a thick line between two points using Bresenham's algorithm
    fn draw_thick_line(
        &self,
        img: &mut RgbImage,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
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
                    if px >= 0 && px < self.width as i32 && py >= 0 && py < self.height as i32 {
                        img.put_pixel(px as u32, py as u32, Rgb(self.line_color));
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

    // Renders the plot as an image
    pub fn render(&self) -> RgbImage {
        let mut img = RgbImage::new(self.width, self.height);

        // Draw a simple white background
        for pixel in img.pixels_mut() {
            *pixel = Rgb(self.bg_color);
        }

        // Draw X-Y coordinate arrows
        self.draw_axes(&mut img);

        // Draw lines connecting the points with thickness
        self.draw_lines_with_thickness(&mut img, self.line_thickness);

        img
    }

    // Saves the rendered plot to a file
    pub fn save(&self, filename: &str) {
        let img = self.render();
        img.save(filename).unwrap();
    }
}

use crate::plot::styles::LineStyle;
use crate::plot::Plot;
use ab_glyph::{FontRef, PxScale};
use image::{Rgb, RgbImage};
use imageproc::drawing::{draw_text_mut, text_size};

impl Plot {
    /// Draws the legend on the provided image.
    ///
    /// # Arguments
    ///
    /// * `img` - A mutable reference to the image where the legend will be drawn.
    ///
    /// This function iterates over the datasets in the plot and draws a colored box
    /// and corresponding label for each dataset in the legend area of the image.
    pub fn draw_legend(&self, img: &mut RgbImage) {
        let font =
            FontRef::try_from_slice(include_bytes!("../../resources/fonts/Arial.ttf")).unwrap();
        let scale = PxScale { x: 15.0, y: 15.0 }; // Font size
        let legend_x = self.margin as i32 + 10;
        let mut legend_y = self.margin as i32 + 10;

        for (_, color, label, _) in &self.datasets {
            // Draw legend box
            for x in legend_x..legend_x + 20 {
                for y in legend_y..legend_y + 20 {
                    if x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32 {
                        img.put_pixel(x as u32, y as u32, Rgb(*color));
                    }
                }
            }

            // Draw legend text
            draw_text_mut(
                img,
                Rgb(self.font_color),
                legend_x + 30,
                legend_y,
                scale,
                &font,
                label,
            );

            legend_y += 30; // Move to the next legend item
        }
    }

    /// Draws the axes on the provided image.
    ///
    /// # Arguments
    ///
    /// * `img` - A mutable reference to the image where the axes will be drawn.
    ///
    /// This function draws the X and Y axes on the image, including the axis lines,
    /// tick marks, and labels.
    pub fn draw_axes(&self, img: &mut RgbImage) {
        // Calculate the center of the axes with respect to the margin
        let center_x = (self.margin + (self.width - 2 * self.margin) / 2) as i32;
        let center_y = (self.margin + (self.height - 2 * self.margin) / 2) as i32;

        // Draw the X-axis (horizontal line at center_y)
        if center_y >= 0 && center_y < self.height as i32 {
            for x in self.margin..(self.width - self.margin) {
                img.put_pixel(x, center_y as u32, Rgb(self.front_color));
            }
        }

        // Draw the Y-axis (vertical line at center_x)
        if center_x >= 0 && center_x < self.width as i32 {
            for y in self.margin..(self.height - self.margin) {
                img.put_pixel(center_x as u32, y, Rgb(self.front_color));
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
    pub fn draw_lines_with_thickness(&self, img: &mut RgbImage) {
        for (data, color, _, line_style) in &self.datasets {
            if data.len() < 2 {
                continue; // Skip if there are fewer than 2 points
            }

            let scale_x =
                (self.width as f64 - 2.0 * self.margin as f64) / (self.x_max - self.x_min);
            let scale_y =
                (self.height as f64 - 2.0 * self.margin as f64) / (self.y_max - self.y_min);

            let mut previous_point: Option<(i32, i32)> = None;

            for &(x, y) in data {
                // Clip the point to the bounds
                if x < self.x_min || x > self.x_max || y < self.y_min || y > self.y_max {
                    previous_point = None; // Skip rendering outside bounds
                    continue;
                }

                let x_pixel = ((x - self.x_min) * scale_x + self.margin as f64) as i32;
                let y_pixel =
                    (self.height as f64 - self.margin as f64 - (y - self.y_min) * scale_y) as i32;

                match line_style {
                    LineStyle::Solid => {
                        if let Some((prev_x, prev_y)) = previous_point {
                            self.draw_thick_line(
                                img,
                                prev_x,
                                prev_y,
                                x_pixel,
                                y_pixel,
                                *color,
                                self.line_thickness,
                            );
                        }
                    }
                    LineStyle::Dotted => {
                        if let Some((prev_x, prev_y)) = previous_point {
                            self.draw_dotted_line(img, prev_x, prev_y, x_pixel, y_pixel, *color);
                        }
                    }
                    LineStyle::Dashed => {
                        if let Some((prev_x, prev_y)) = previous_point {
                            self.draw_dashed_line(img, prev_x, prev_y, x_pixel, y_pixel, *color);
                        }
                    }
                    LineStyle::DashDot => {
                        if let Some((prev_x, prev_y)) = previous_point {
                            self.draw_dash_dot_line(img, prev_x, prev_y, x_pixel, y_pixel, *color);
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
    pub fn draw_thick_line(
        &self,
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
                    if px >= 0 && px < self.width as i32 && py >= 0 && py < self.height as i32 {
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
    pub fn draw_dotted_line(
        &self,
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
                for tx in -(self.line_thickness as i32 / 2)..=(self.line_thickness as i32 / 2) {
                    for ty in -(self.line_thickness as i32 / 2)..=(self.line_thickness as i32 / 2) {
                        let px = x + tx;
                        let py = y + ty;
                        if px >= 0 && px < self.width as i32 && py >= 0 && py < self.height as i32 {
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
    pub fn draw_dashed_line(
        &self,
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
                for tx in -(self.line_thickness as i32 / 2)..=(self.line_thickness as i32 / 2) {
                    for ty in -(self.line_thickness as i32 / 2)..=(self.line_thickness as i32 / 2) {
                        let px = x + tx;
                        let py = y + ty;
                        if px >= 0 && px < self.width as i32 && py >= 0 && py < self.height as i32 {
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
    pub fn draw_dash_dot_line(
        &self,
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
                for tx in -(self.line_thickness as i32 / 2)..=(self.line_thickness as i32 / 2) {
                    for ty in -(self.line_thickness as i32 / 2)..=(self.line_thickness as i32 / 2) {
                        let px = x + tx;
                        let py = y + ty;
                        if px >= 0 && px < self.width as i32 && py >= 0 && py < self.height as i32 {
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
    ///
    /// * `img` - A mutable reference to the image where the grid lines will be drawn.
    ///
    /// This function draws the grid lines on the image based on the specified grid style.
    pub fn draw_grid(&self, img: &mut RgbImage) {
        // Scaling factors adjusted for margins
        let scale_x = (self.width as f64 - 2.0 * self.margin as f64) / (self.x_max - self.x_min);
        let scale_y = (self.height as f64 - 2.0 * self.margin as f64) / (self.y_max - self.y_min);

        // Draw vertical grid lines
        let mut x = self.x_min;
        while x <= self.x_max {
            let x_pixel = ((x - self.x_min) * scale_x + self.margin as f64) as u32;
            for y in self.margin..(self.height - self.margin) {
                img.put_pixel(x_pixel, y, Rgb(self.grid_color));
            }
            x += (self.x_max - self.x_min) / 10.0; // Adjust grid spacing here
        }

        // Draw horizontal grid lines
        let mut y = self.y_min;
        while y <= self.y_max {
            let y_pixel =
                (self.height as f64 - self.margin as f64 - ((y - self.y_min) * scale_y)) as u32;
            for x in self.margin..(self.width - self.margin) {
                img.put_pixel(x, y_pixel, Rgb(self.grid_color));
            }
            y += (self.y_max - self.y_min) / 10.0; // Adjust grid spacing here
        }
    }

    // Draws labels for the x-axis and y-axis
    fn draw_axis_labels(&self, img: &mut RgbImage) {
        let font =
            FontRef::try_from_slice(include_bytes!("../../resources/fonts/Arial.ttf")).unwrap(); // Provide the path to your font file
        let scale = PxScale { x: 20.0, y: 20.0 }; // Font size

        let (_w_x, h_x) = text_size(scale, &font, &self.xlabel);
        let (w_y, _h_y) = text_size(scale, &font, &self.ylabel);

        // Draw y-axis label
        draw_text_mut(
            img,
            Rgb(self.font_color),
            (self.width - self.margin) as i32 + scale.x as i32, // Left margin
            ((self.height - h_x) / 2) as i32,                   // Adjust position to center
            scale,
            &font,
            &self.xlabel,
        );

        // Draw x-axis label
        draw_text_mut(
            img,
            Rgb(self.font_color),
            (self.width - w_y) as i32 / 2, // Adjust position to center
            (self.height - self.margin / 2) as i32, // Slightly above the bottom margin
            scale,
            &font,
            &self.ylabel,
        );
    }

    // Draws labels for the y-axis using `imageproc`
    fn draw_y_labels(&self, img: &mut RgbImage) {
        let scale_y = (self.height as f64 - 2.0 * self.margin as f64) / (self.y_max - self.y_min);
        let font =
            FontRef::try_from_slice(include_bytes!("../../resources/fonts/Arial.ttf")).unwrap(); // Provide the path to your font file
        let scale = PxScale { x: 15.0, y: 15.0 }; // Font size

        let mut y: f64 = self.y_min;
        while y <= self.y_max {
            let y_pixel =
                (self.height as f64 - self.margin as f64 - ((y - self.y_min) * scale_y)) as i32;

            // Convert y value to a string
            let label = format!("{:+.1}", y);
            let label_text_width = (label.len() - 1) as i32 * scale.x as i32;
            let label_text_height = scale.y as i32;
            let label_x = if self.margin as i32 >= label_text_width {
                self.margin as i32 - label_text_width
            } else {
                0
            };

            // Draw text directly on the image
            draw_text_mut(
                &mut *img,                       // The image to draw on
                Rgb(self.font_color),            // Text color
                label_x,                         // x-coordinate for text
                y_pixel - label_text_height / 2, // y-coordinate for text
                scale,                           // Font scale
                &font,                           // Font
                &label,                          // The label text
            );

            y += (self.y_max - self.y_min) / 10.0;
        }
    }

    // Draws labels for the x-axis
    fn draw_x_labels(&self, img: &mut RgbImage) {
        let scale_x = (self.width as f64 - 2.0 * self.margin as f64) / (self.x_max - self.x_min);
        let font =
            FontRef::try_from_slice(include_bytes!("../../resources/fonts/Arial.ttf")).unwrap(); // Provide the path to your font file
        let scale = PxScale { x: 15.0, y: 15.0 }; // Font size

        let mut x: f64 = self.x_min;
        while x <= self.x_max {
            let x_pixel = ((x - self.x_min) * scale_x + self.margin as f64) as i32;

            // Convert x value to a string
            let label = format!("{:+.1}", x);
            let (w, h) = text_size(scale, &font, &label);
            // Draw text directly on the image
            draw_text_mut(
                &mut *img,                                              // The image to draw on
                Rgb(self.font_color),                                   // Text color
                x_pixel - w as i32 / 2,                                 // x-coordinate for text
                self.height as i32 - self.margin as i32 + h as i32 / 2, // y-coordinate for text
                scale,                                                  // Font scale
                &font,                                                  // Font
                &label,                                                 // The label text
            );

            x += (self.x_max - self.x_min) / 10.0;
        }
    }

    // Draws the title of the plot
    fn draw_title(&self, img: &mut RgbImage) {
        let font =
            FontRef::try_from_slice(include_bytes!("../../resources/fonts/Arial.ttf")).unwrap(); // Provide the path to your font file
        let scale = PxScale { x: 25.0, y: 25.0 }; // Font size
        let (w, _h) = text_size(scale, &font, &self.title);
        // Calculate position for the title at the top center of the plot
        let x_position = (self.width as i32) / 2 - w as i32 / 2;
        let y_position = (self.margin as i32) / 4; // Adjust as needed for spacing

        draw_text_mut(
            img,
            Rgb(self.title_color),
            x_position,
            y_position,
            scale,
            &font,
            &self.title,
        );
    }

    pub fn render(&self) -> RgbImage {
        let mut img = RgbImage::new(self.width, self.height);

        // Draw a simple white background
        for pixel in img.pixels_mut() {
            *pixel = Rgb(self.background_color);
        }

        // Draw X-Y coordinate arrows
        self.draw_axes(&mut img);
        // Add y-axis labels
        self.draw_y_labels(&mut img);
        // Add x-axis labels
        self.draw_x_labels(&mut img);
        // Draw axis labels
        self.draw_axis_labels(&mut img);
        // Draw the title
        self.draw_title(&mut img);

        // Draw lines connecting the points with thickness
        self.draw_lines_with_thickness(&mut img);
        // Draw the legend
        self.draw_legend(&mut img);
        img
    }

    pub fn render_with_grid(&self) -> RgbImage {
        let mut img = RgbImage::new(self.width, self.height);

        // Draw a simple white background
        for pixel in img.pixels_mut() {
            *pixel = Rgb(self.background_color);
        }

        // Add grid lines
        self.draw_grid(&mut img);
        // Draw X-Y coordinate arrows
        self.draw_axes(&mut img);
        // Add y-axis labels
        self.draw_y_labels(&mut img);
        // Add x-axis labels
        self.draw_x_labels(&mut img);
        // Draw axis labels
        self.draw_axis_labels(&mut img);
        // Draw the title
        self.draw_title(&mut img);

        // Draw lines connecting the points with thickness
        self.draw_lines_with_thickness(&mut img);
        // Draw the legend
        self.draw_legend(&mut img);

        img
    }

    // Saves the rendered plot to a file
    pub fn save_with_grid(&self, filename: &str) {
        let img = self.render_with_grid();
        img.save(filename).unwrap();
    }

    // Saves the rendered plot to a file
    pub fn save(&self, filename: &str) {
        let img = self.render();
        img.save(filename).unwrap();
    }
}

use crate::plot::linetype::LineType;
use ab_glyph::{FontRef, PxScale};
use image::Rgb;
use imageproc::drawing::draw_text_mut;

pub struct Canvas {
    pub width: u32,
    pub height: u32,
    pub background_color: [u8; 3],
    pub buffer: Vec<u8>,
    pub margin: u32,
}

impl Canvas {
    pub fn new(width: u32, height: u32, background_color: [u8; 3], margin: u32) -> Self {
        let buffer = vec![0; (width * height * 3) as usize];
        Self {
            width,
            height,
            background_color,
            buffer,
            margin,
        }
    }

    pub fn clear(&mut self) {
        self.buffer.fill(self.background_color[0]);
    }

    pub fn draw_pixel(&mut self, x: u32, y: u32, color: [u8; 3]) {
        let index = ((y * self.width + x) * 3) as usize;
        if index + 2 < self.buffer.len() {
            self.buffer[index] = color[0];
            self.buffer[index + 1] = color[1];
            self.buffer[index + 2] = color[2];
        }
    }

    pub fn blend_pixel(&mut self, x: u32, y: u32, color: [u8; 3], alpha: f64) {
        let index = ((y * self.width + x) * 3) as usize;
        if index + 2 < self.buffer.len() {
            let existing_color = [
                self.buffer[index],
                self.buffer[index + 1],
                self.buffer[index + 2],
            ];

            let blended_color = [
                (color[0] as f64 * alpha + existing_color[0] as f64 * (1.0 - alpha)) as u8,
                (color[1] as f64 * alpha + existing_color[1] as f64 * (1.0 - alpha)) as u8,
                (color[2] as f64 * alpha + existing_color[2] as f64 * (1.0 - alpha)) as u8,
            ];

            self.buffer[index] = blended_color[0];
            self.buffer[index + 1] = blended_color[1];
            self.buffer[index + 2] = blended_color[2];
        }
    }

    pub fn draw_horizontal_line(&mut self, y: u32, color: [u8; 3]) {
        for x in self.margin..self.width - self.margin {
            self.draw_pixel(x, y, color);
        }
    }

    pub fn draw_vertical_line(&mut self, x: u32, color: [u8; 3]) {
        for y in self.margin..self.height - self.margin {
            self.draw_pixel(x, y, color);
        }
    }

    pub fn draw_grid(&mut self, grid_size: u32, color: [u8; 3]) {
        for x in (self.margin..=self.width - self.margin).step_by(grid_size as usize) {
            self.draw_vertical_line(x, color);
        }
        for y in (self.margin..=self.height - self.margin).step_by(grid_size as usize) {
            self.draw_horizontal_line(y, color);
        }
    }

    pub fn draw_text(
        &mut self,
        x: u32,
        y: u32,
        text: &str,
        color: [u8; 3],
        font: &FontRef,
        scale: PxScale,
    ) {
        let img: &mut [u8] = &mut self.buffer;
        let mut buffer =
            image::ImageBuffer::from_raw(self.width as u32, self.height as u32, img.to_vec())
                .unwrap();
        draw_text_mut(
            &mut buffer,
            Rgb(color),
            x as i32,
            y as i32,
            scale,
            &font,
            text,
        );

        self.buffer = buffer.into_raw();
        // Placeholder for text rendering logic.
        println!(
            "Drawing text '{}' at ({}, {}) with color {:?}",
            text, x, y, color
        );
    }

    pub fn draw_line(
        &mut self,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        color: [u8; 3],
        line_type: LineType,
    ) {
        let dx = (x2 - x1).abs();
        let dy = -(y2 - y1).abs();
        let sx = if x1 < x2 { 1 } else { -1 };
        let sy = if y1 < y2 { 1 } else { -1 };
        let mut err = dx + dy;

        let mut x = x1;
        let mut y = y1;

        match line_type {
            LineType::Solid => {
                // Draw a continuous line without any gaps
                while x != x2 || y != y2 {
                    self.draw_pixel(x as u32, y as u32, color);

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
                // Draw the final pixel
                self.draw_pixel(x2 as u32, y2 as u32, color);
            }
            LineType::Dashed(dash_length) | LineType::Dotted(dash_length) => {
                let mut is_drawing = true;
                let mut segment_length = 0;

                while x != x2 || y != y2 {
                    if is_drawing {
                        self.draw_pixel(x as u32, y as u32, color);
                    }

                    segment_length += 1;
                    if segment_length == dash_length {
                        is_drawing = !is_drawing; // Toggle drawing
                        segment_length = 0; // Reset segment length
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
                // Ensure the final pixel is drawn in drawing mode
                if is_drawing {
                    self.draw_pixel(x2 as u32, y2 as u32, color);
                }
            }
        }
    }

    pub fn save_as_image(&self, file_path: &str) {
        use image::{ImageBuffer, RgbImage};

        let img: RgbImage = ImageBuffer::from_raw(self.width, self.height, self.buffer.clone())
            .expect("Failed to create image buffer");
        img.save(file_path).expect("Failed to save image");
        println!("Canvas saved as: {}", file_path);
    }
}

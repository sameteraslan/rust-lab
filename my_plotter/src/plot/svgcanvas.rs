use std::{
    fs::File,
    io::{self, Write},
};

use ab_glyph::{Font, FontRef, Glyph};

use super::drawer::Drawer;

pub struct SvgCanvas {
    pub width: u32,
    pub height: u32,
    pub elements: Vec<String>, // Store SVG elements
    pub margin: u32,
    pub background_color: String,
}

impl SvgCanvas {
    pub fn new(width: u32, height: u32, background_color: &str, margin: u32) -> Self {
        Self {
            width,
            height,
            elements: vec![format!(
                r#"<?xml version="1.0" encoding="UTF-8"?>
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {} {}">"#,
                width, height
            )],
            margin,
            background_color: background_color.to_string(),
        }
    }

    pub fn clear(&mut self) {
        // Clear all SVG elements
        self.elements.clear();

        // Add the opening SVG tag with initial properties
        self.elements.push(format!(
            r#"<svg width="{:.2}" height="{:.2}" xmlns="http://www.w3.org/2000/svg">"#,
            self.width, self.height
        ));

        // Optionally add a background color
        self.elements.push(format!(
            r#"<rect width="{:.2}" height="{:.2}" fill="{}"/>"#,
            self.width, self.height, self.background_color
        ));

        // Add the closing SVG tag placeholder (to be finalized later)
        self.elements.push("</svg>".to_string());
    }

    pub fn draw_line(
        &mut self,
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
        color: &str,
        stroke_width: f64,
    ) {
        self.elements.push(format!(
            r#"<line x1="{:.2}" y1="{:.2}" x2="{:.2}" y2="{:.2}" stroke="{}" stroke-width="{:.2}"/>"#,
            x1, y1, x2, y2, color, stroke_width
        ));
    }

    pub fn draw_line_rgb(
        &mut self,
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
        color: [u8; 3],
        stroke_width: f64,
    ) {
        self.elements.push(format!(
            r#"<line x1="{:.2}" y1="{:.2}" x2="{:.2}" y2="{:.2}" stroke="rgb({},{},{})" stroke-width="{:.2}"/>"#,
            x1, y1, x2, y2, color[0], color[1], color[2], stroke_width
        ));
    }

    pub fn draw_rect(
        &mut self,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        fill_color: &str,
        stroke_color: &str,
        stroke_width: f64,
        opacity: f64
    ) {
        self.elements.push(format!(
            r#"<rect x="{:.2}" y="{:.2}" width="{:.2}" height="{:.2}" fill="{}" stroke="{}" stroke-width="{:.2}" fill-opacity="{}"/>"#,
            x, y, width, height, fill_color, stroke_color, stroke_width, opacity
        ));
    }

    pub fn add_font_style(&mut self, font_url: &str, class_name: &str, font_family: &str) {
        self.elements.push(format!(
            r#"<style>
                @import url('{}');
                .{} {{
                    font-family: '{}', sans-serif;
                }}
            </style>"#,
            font_url, class_name, font_family
        ));
    }

    pub fn draw_circle(&mut self, cx: f64, cy: f64, r: f64, color: &str) {
        self.elements.push(format!(
            r#"<circle cx="{:.2}" cy="{:.2}" r="{:.2}" fill="{}"/>"#,
            cx, cy, r, color
        ));
    }

    pub fn draw_text(&mut self, x: f64, y: f64, text: &str, font_size: f64, color: &str) {
        self.elements.push(format!(
            r#"<text x="{:.2}" y="{:.2}" font-size="{:.2}" text-anchor="middle" fill="{}">{}</text>"#,
            x, y, font_size, color, text
        ));
    }

    pub fn draw_title(&mut self, x: f64, y: f64, text: &str, font_size: f64, color: &str) {
        self.elements.push(format!(
            r#"<text x="{:.2}" y="{:.2}" font-size="{:.2}" text-anchor="middle" fill="{}">{}</text>"#,
            x, y, font_size, color ,text
        ));
    }

    pub fn draw_grid(
        &mut self,
        x_min: f64,
        x_max: f64,
        y_min: f64,
        y_max: f64,
        x_ticks: usize,
        y_ticks: usize,
        color: &str,
    ) {
        let x_step = (x_max - x_min) / x_ticks as f64;
        let y_step = (y_max - y_min) / y_ticks as f64;

        for i in 0..=x_ticks {
            let x = x_min + i as f64 * x_step;
            self.draw_line(x, y_min, x, y_max, color, 0.5);
        }

        for i in 0..=y_ticks {
            let y = y_min + i as f64 * y_step;
            self.draw_line(x_min, y, x_max, y, color, 0.5);
        }
    }

    pub fn save(&self, file_path: &str) -> io::Result<()> {
        let mut file = File::create(file_path)?;
        for element in &self.elements {
            writeln!(file, "{}", element)?;
        }
        writeln!(file, "</svg>")?;
        Ok(())
    }
}

use std::fmt::format;

use ab_glyph::{FontRef, PxScale};
use imageproc::drawing::text_size;

use super::{canvas::Canvas, cartesiangraph::CartesianGraph, drawer::Drawer, svgcanvas::SvgCanvas};

impl Drawer for CartesianGraph {
    fn draw_svg(&mut self, svg_canvas: &mut SvgCanvas) {
        // Clear existing SVG elements
        // svg_canvas.clear();

        let width = svg_canvas.width as f64;
        let height = svg_canvas.height as f64;
        let margin = svg_canvas.margin as f64;
        let font_size = 12.0;

        // Draw background
        svg_canvas.draw_rect(0.0, 0.0, width, height, "white", "black", 2.0, 1.0);

        // Draw Title
        svg_canvas.draw_title(
            width / 2.0,
            margin / 2.0,
            &self.title,
            font_size * 2.0,
            "black",
        );

        // Symmetric scaling
        self.update_range();

        let scale_x = (svg_canvas.width - 2 * svg_canvas.margin) as f64 / (self.x_max - self.x_min);
        let scale_y =
            (svg_canvas.height - 2 * svg_canvas.margin) as f64 / (self.y_max - self.y_min);

        // Draw grid
        let num_ticks = 20;
        svg_canvas.draw_grid(
            margin,
            width - margin,
            margin,
            height - margin,
            num_ticks,
            num_ticks,
            "lightgray",
        );

        // Draw axes
        let center_x = margin + (0.0 - self.x_min) * scale_x;
        let center_y = height - margin - (0.0 - self.y_min) * scale_y;

        svg_canvas.draw_line(margin, center_y, width - margin, center_y, "black", 2.0);
        svg_canvas.draw_line(center_x, margin, center_x, height - margin, "black", 2.0);

        // Draw tick marks and labels

        // X-axis
        let mut x_axis_ticks = String::new();
        for i in 0..=num_ticks {
            let value = self.x_min + i as f64 * (self.x_max - self.x_min) / num_ticks as f64;
            let x = margin + i as f64 * (width - 2.0 * margin) / num_ticks as f64;
            let tick_start_y = center_y - 5.0;
            let tick_end_y = center_y + 5.0;

            x_axis_ticks.push_str(&format!(
                "M {:.2},{:.2} L {:.2},{:.2} ",
                x, tick_start_y, x, tick_end_y
            ));

            // Draw value as text (fallback to basic SVG <text>)
            svg_canvas.elements.push(format!(
            r#"<text x="{:.2}" y="{:.2}" font-size="{:.2}" text-anchor="middle" fill="black">{:.1}</text>"#,
            x, height - margin + font_size * 1.5, font_size, value));
        }
        svg_canvas.elements.push(format!(
            r#"<path d="{}" stroke="black" stroke-width="1" fill="none"/>"#,
            x_axis_ticks
        ));

        // Y-axis
        let mut y_axis_ticks = String::new();
        for i in 0..=num_ticks {
            let value = self.y_min + i as f64 * (self.y_max - self.y_min) / num_ticks as f64;
            let y = height - margin - i as f64 * (height - 2.0 * margin) / num_ticks as f64;
            let tick_start_x = center_x - 5.0;
            let tick_end_x = center_x + 5.0;

            y_axis_ticks.push_str(&format!(
                "M {:.2},{:.2} L {:.2},{:.2} ",
                tick_start_x, y, tick_end_x, y
            ));

            // Draw value as text (fallback to basic SVG <text>)
            svg_canvas.elements.push(format!(
            r#"<text x="{:.2}" y="{:.2}" font-size="{:.2}" text-anchor="end" fill="black">{:.1}</text>"#,
            margin - 5.0, y + font_size * 0.3, font_size, value
        ));
        }
        svg_canvas.elements.push(format!(
            r#"<path d="{}" stroke="black" stroke-width="1" fill="none"/>"#,
            y_axis_ticks
        ));

        // Draw X-axis label
        svg_canvas.elements.push(format!(
        r#"<text x="{:.2}" y="{:.2}" font-size="{:.2}" text-anchor="middle" fill="black">{}</text>"#,
        width / 2.0,
        height - margin / 3.0,
        font_size * 1.5,
        self.x_label
    ));

        // Draw Y-axis label (rotated)
        svg_canvas.elements.push(format!(
        r#"<text x="{:.2}" y="{:.2}" font-size="{:.2}" text-anchor="middle" fill="black" transform="rotate(-90 {:.2} {:.2})">{}</text>"#,
        margin / 3.0,
        height / 2.0,
        font_size * 1.5,
        margin / 3.0,
        height / 2.0,
        self.y_label
    ));

        // Plot datasets
        for dataset in &self.datasets {
            for window in dataset.points.windows(2) {
                if let [p1, p2] = window {
                    let x1 = margin + (p1.0 - self.x_min) * scale_x;
                    let y1 = height - margin - (p1.1 - self.y_min) * scale_y;
                    let x2 = margin + (p2.0 - self.x_min) * scale_x;
                    let y2 = height - margin - (p2.1 - self.y_min) * scale_y;

                    svg_canvas.draw_line_rgb(x1, y1, x2, y2, dataset.color, 2.0);
                }
            }
        }

        // Draw legend
        let mut legend_x = margin + 5.0;
        let legend_y = margin;

        let mut elements = String::new();
        for dataset in &self.datasets {
            elements.push_str(&format!(
                r#"<rect x="{:.2}" y="{:.2}" width="{:.2}" height="{:.2}" fill="rgb({},{},{})"/>"#,
                legend_x,
                legend_y,
                font_size,
                font_size,
                dataset.color[0],
                dataset.color[1],
                dataset.color[2]
            ));

            elements.push_str(&format!(
                r#"<text x="{:.2}" y="{:.2}" font-size="{:.2}" fill="rgb({},{},{})">{}</text>"#,
                legend_x + font_size * 1.3,
                legend_y + font_size - 2.0,
                font_size,
                dataset.color[0],
                dataset.color[1],
                dataset.color[2],
                dataset.label
            ));

            legend_x += font_size * (dataset.label.len() - 1) as f64;
        }

        svg_canvas.draw_rect(
            margin,
            margin - 10.0,
            legend_x - margin + 5.0,
            font_size + 20.0,
            "white",
            "black",
            0.5,
            0.5,
        );
        svg_canvas.elements.push(elements);
    }

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

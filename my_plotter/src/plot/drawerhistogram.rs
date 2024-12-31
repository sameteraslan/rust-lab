use ab_glyph::{FontRef, PxScale};
use imageproc::drawing::text_size;

use super::{canvas::Canvas, drawer::Drawer, historgram::Histogram, svgcanvas::SvgCanvas};

impl Drawer for Histogram {
    fn draw_svg(&mut self, svg_canvas: &mut SvgCanvas) {
        // Clear existing SVG elements

        let width = svg_canvas.width as f64;
        let height = svg_canvas.height as f64;
        let margin = svg_canvas.margin as f64;
        let font_size = 12.0;

        // Draw background
        svg_canvas.draw_rect(0.0, 0.0, width, height, "white", "black", 1.0, 1.0);

        // Draw Title
        svg_canvas.draw_title(
            width / 2.0,
            margin / 2.0,
            &self.title,
            font_size * 2.0,
            "black",
        );

        // Calculate range and scales
        let y_max = self.bin_counts.iter().cloned().fold(0.0, f64::max);
        let scale_x = (width - 2.0 * margin) / (self.max - self.min);
        let scale_y = (height - 2.0 * margin) / y_max;

        // Draw grid
        let num_ticks = 10;
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
        let origin_x = margin;
        let origin_y = height - margin;

        svg_canvas.draw_line(margin, origin_y, width - margin, origin_y, "black", 2.0); // X-axis
        svg_canvas.draw_line(margin, margin, margin, height - margin, "black", 2.0); // Y-axis

        // X-axis
        let mut x_axis_ticks = String::new();
        for i in 0..=num_ticks {
            let value = self.max + i as f64 * (self.max - self.min) / num_ticks as f64;
            let x = margin + i as f64 * (width - 2.0 * margin) / num_ticks as f64;
            let tick_start_y = origin_y - 5.0;
            let tick_end_y = origin_y + 5.0;

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
            let value = 0.0 + i as f64 * (y_max - 0.0) / num_ticks as f64;
            let y = height - margin - i as f64 * (height - 2.0 * margin) / num_ticks as f64;
            let tick_start_x = origin_x - 5.0;
            let tick_end_x = origin_x + 5.0;

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
        svg_canvas.draw_text(
            width / 2.0,
            height - margin / 4.0,
            &self.x_label,
            font_size * 1.5,
            "black",
        );

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

        // Draw histogram bars
        for (i, &count) in self.bin_counts.iter().enumerate() {
            let bin_start = self.min + i as f64 * self.bin_width;
            let bin_end = bin_start + self.bin_width;

            let x_start = margin + (bin_start - self.min) * scale_x;
            let x_end = margin + (bin_end - self.min) * scale_x;
            let bar_width = x_end - x_start;
            let bar_height = count * scale_y;

            svg_canvas.draw_rect(
                x_start,
                origin_y - bar_height,
                bar_width,
                bar_height,
                &format!("rgb({},{},{})", self.color[0], self.color[1], self.color[2]),
                "black",
                1.0,
                1.0,
            );
        }
    }

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

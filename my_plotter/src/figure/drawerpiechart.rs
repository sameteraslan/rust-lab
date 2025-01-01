use std::f64::consts::PI;

use ab_glyph::{FontRef, PxScale};
use imageproc::drawing::text_size;

use super::{canvas::Canvas, drawer::Drawer, piechart::PieChart, svgcanvas::SvgCanvas};

impl Drawer for PieChart {
    fn draw_svg(&mut self, svg_canvas: &mut SvgCanvas) {
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

        // Calculate total value of all slices
        let total: f64 = self.datasets.iter().map(|dataset| dataset.1).sum();

        // Calculate center and radius
        let cx = width / 2.0;
        let cy = height / 2.0;
        let radius = (width.min(height) - 2.0 * margin) / 2.0;

        // Begin group for pie chart with transformation
        svg_canvas.elements.push(format!(
            r#"<g transform="translate({:.2},{:.2})" stroke="black" stroke-width="1">"#,
            cx, cy
        ));

        // Track the starting angle in radians
        let mut start_angle = 0.0;

        // Draw pie slices
        for dataset in &self.datasets {
            let value_ratio = dataset.1 / total; // Ratio of this slice to the total
            let sweep_angle = value_ratio * 2.0 * std::f64::consts::PI; // Convert ratio to radians
            let end_angle = start_angle + sweep_angle;

            // Calculate start and end points of the slice
            let x1 = radius * start_angle.cos();
            let y1 = radius * start_angle.sin();
            let x2 = radius * end_angle.cos();
            let y2 = radius * end_angle.sin();

            // Determine if the slice is larger than 180 degrees
            let large_arc_flag = if sweep_angle > std::f64::consts::PI {
                1
            } else {
                0
            };

            // Generate the path for the slice
            svg_canvas.elements.push(format!(
               r#"<path d="M 0 0 L {:.2} {:.2} A {:.2} {:.2} 0 {} 1 {:.2} {:.2} Z" fill="rgb({},{},{})"/>"#,
               x1, y1, radius, radius, large_arc_flag, x2, y2,
               dataset.2[0], dataset.2[1], dataset.2[2]
           ));

            // Calculate label position (midpoint of the slice angle)
            let mid_angle = start_angle + sweep_angle / 2.0;
            let label_x = (radius * 0.6) * mid_angle.cos(); // 60% of radius for better placement
            let label_y = (radius * 0.6) * mid_angle.sin();

            // Draw percentage label
            svg_canvas.elements.push(format!(
               r#"<text x="{:.2}" y="{:.2}" font-size="{:.2}" fill="black" text-anchor="middle" alignment-baseline="middle">{:.1}%</text>"#,
               label_x, label_y, font_size, value_ratio * 100.0
           ));

            // Update start angle for the next slice
            start_angle = end_angle;
        }

        // Close group
        svg_canvas.elements.push("</g>".to_string());

        // Draw legend in the bottom-left corner
        let legend_x_start = 5.0; // Start at the very left with margin spacing
        let legend_y = height - margin / 2.0; // Move to bottom-left corner

        let mut legend_x = legend_x_start;
        let mut elements = String::new();

        for dataset in &self.datasets {
            elements.push_str(&format!(
                r#"<rect x="{:.2}" y="{:.2}" width="{:.2}" height="{:.2}" fill="rgb({},{},{})"/>"#,
                legend_x, legend_y, font_size, font_size, dataset.2[0], dataset.2[1], dataset.2[2]
            ));

            elements.push_str(&format!(
                r#"<text x="{:.2}" y="{:.2}" font-size="{:.2}" fill="black">{}</text>"#,
                legend_x + font_size * 1.3,
                legend_y + font_size - 2.0,
                font_size,
                dataset.0
            ));

            legend_x += font_size * 5.0 + dataset.0.len() as f64 * font_size * 0.6;
        }

        svg_canvas.draw_rect(
            legend_x_start - 5.0,
            legend_y - 5.0,
            legend_x - legend_x_start + 5.0,
            font_size + 10.0,
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
            FontRef::try_from_slice(include_bytes!("../../resources/fonts/Arial.ttf")).unwrap();
        let scale_title = PxScale { x: 20.0, y: 20.0 };

        // Draw the title
        let (w_title, h_title) = text_size(scale_title, &font, &self.title);
        let title_x = (canvas.width).saturating_sub(w_title) / 2;
        let title_y = (canvas.margin / 3).saturating_sub(h_title) as u32;
        canvas.draw_text(title_x, title_y, &self.title, [0, 0, 0], &font, scale_title);

        // Calculate total value
        let total: f64 = self.datasets.iter().map(|(_, value, _)| value).sum();
        if total == 0.0 {
            return;
        }

        // Center and radius of the pie chart
        let center_x = canvas.width / 2;
        let center_y = canvas.height / 2;
        let radius = (canvas.width.min(canvas.height) / 2 - canvas.margin) as i32;

        let mut start_angle = 0.0;
        for (_label, value, color) in &self.datasets {
            let percentage = value / total;
            let sweep_angle = 2.0 * PI * percentage;

            // Draw the slice
            self.draw_slice(
                canvas,
                center_x as i32,
                center_y as i32,
                radius,
                start_angle,
                start_angle + sweep_angle,
                *color,
            );

            // Calculate mid-angle for label placement
            let mid_angle = start_angle + sweep_angle / 2.0;
            let label_x = center_x as f64 + (radius as f64 * 0.6 * mid_angle.cos());
            let label_y = center_y as f64 - (radius as f64 * 0.6 * mid_angle.sin());
            canvas.draw_text(
                label_x as u32,
                label_y as u32,
                &format!("{:.1}%", percentage * 100.0),
                [0, 0, 0],
                &font,
                PxScale { x: 12.0, y: 12.0 },
            );

            start_angle += sweep_angle;
        }

        // Draw legend
        self.draw_legend(canvas);
    }

    fn draw_legend(&self, canvas: &mut Canvas) {
        let font =
            FontRef::try_from_slice(include_bytes!("../../resources/fonts/Arial.ttf")).unwrap();
        let scale = PxScale { x: 15.0, y: 15.0 };

        let square_size = 10;
        let padding = 5;
        let line_height = 20;
        let x = canvas.margin;
        let mut y = canvas.height - canvas.margin;

        for (label, _, color) in &self.datasets {
            // Draw the square
            for dy in 0..square_size {
                for dx in 0..square_size {
                    canvas.draw_pixel(x + dx, y + dy, *color);
                }
            }

            // Draw the label
            canvas.draw_text(x + square_size + padding, y, label, [0, 0, 0], &font, scale);

            y += line_height;
        }
    }
}

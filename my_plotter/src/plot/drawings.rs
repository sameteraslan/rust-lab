use super::plot;
use crate::plot::bar_chart_plot::BarChartDrawing;
use crate::plot::cartesian_graph_plot::CartesianGraphDrawing;
use crate::plot::styles::PlotType;
use ab_glyph::{FontRef, PxScale};
use image::{Rgb, RgbImage};
use imageproc::drawing::{draw_text_mut, text_size};

pub struct Drawing;

impl Drawing {
    // Draws the title of the plot
    fn draw_title(plot: &mut plot::Plot, img: &mut RgbImage) {
        let font =
            FontRef::try_from_slice(include_bytes!("../../resources/fonts/Arial.ttf")).unwrap(); // Provide the path to your font file
        let scale = PxScale { x: 25.0, y: 25.0 }; // Font size
        let (w, _h) = text_size(scale, &font, &plot.title);
        // Calculate position for the title at the top center of the plot
        let x_position = (plot.width as i32) / 2 - w as i32 / 2;
        let y_position = (plot.margin as i32) / 4; // Adjust as needed for spacing

        draw_text_mut(
            img,
            Rgb(plot.title_color),
            x_position,
            y_position,
            scale,
            &font,
            &plot.title,
        );
    }

    // Draws labels for the y-axis using `imageproc`
    fn draw_y_labels(plot: &mut plot::Plot, img: &mut RgbImage) {
        let scale_y = (plot.height as f64 - 2.0 * plot.margin as f64) / (plot.y_max - plot.y_min);
        let font =
            FontRef::try_from_slice(include_bytes!("../../resources/fonts/Arial.ttf")).unwrap(); // Provide the path to your font file
        let scale = PxScale { x: 15.0, y: 15.0 }; // Font size
        let (w, h) = text_size(scale, &font, format!("{:+.1}", plot.y_min).as_str());

        let mut y: f64 = plot.y_min;
        while y <= plot.y_max {
            let y_pixel =
                (plot.height as f64 - plot.margin as f64 - ((y - plot.y_min) * scale_y)) as i32;

            // Convert y value to a string
            let label = format!("{:+.1}", y);
            let label_x = if plot.margin >= w + h {
                (plot.margin - w - h) as i32
            } else {
                0
            };

            // Draw text directly on the image
            draw_text_mut(
                &mut *img,              // The image to draw on
                Rgb(plot.font_color),   // Text color
                label_x,                // x-coordinate for text
                y_pixel - h as i32 / 2, // y-coordinate for text
                scale,                  // Font scale
                &font,                  // Font
                &label,                 // The label text
            );

            y += (plot.y_max - plot.y_min) / 10.0;
        }
    }

    // Draws labels for the x-axis
    fn draw_x_labels(plot: &mut plot::Plot, img: &mut RgbImage) {
        let scale_x = (plot.width as f64 - 2.0 * plot.margin as f64) / (plot.x_max - plot.x_min);
        let font =
            FontRef::try_from_slice(include_bytes!("../../resources/fonts/Arial.ttf")).unwrap(); // Provide the path to your font file
        let scale = PxScale { x: 15.0, y: 15.0 }; // Font size
        let (w, h) = text_size(scale, &font, format!("{:+.1}", plot.x_max).as_str());

        let mut x: f64 = plot.x_min;
        while x <= plot.x_max {
            let x_pixel = ((x - plot.x_min) * scale_x + plot.margin as f64) as i32;

            // Convert x value to a string
            let label = format!("{:+.1}", x);

            // Draw text directly on the image
            draw_text_mut(
                &mut *img,                                              // The image to draw on
                Rgb(plot.font_color),                                   // Text color
                x_pixel - w as i32 / 2,                                 // x-coordinate for text
                plot.height as i32 - plot.margin as i32 + h as i32 / 2, // y-coordinate for text
                scale,                                                  // Font scale
                &font,                                                  // Font
                &label,                                                 // The label text
            );

            x += (plot.x_max - plot.x_min) / 10.0;
        }
    }

    /// Draws the legend on the provided image.
    ///
    /// # Arguments
    ///
    /// * `img` - A mutable reference to the image where the legend will be drawn.
    ///
    /// This function iterates over the datasets in the plot and draws a colored box
    /// and corresponding label for each dataset in the legend area of the image.
    fn draw_legend(plot: &mut plot::Plot, img: &mut RgbImage) {
        let font =
            FontRef::try_from_slice(include_bytes!("../../resources/fonts/Arial.ttf")).unwrap();
        let scale = PxScale { x: 15.0, y: 15.0 }; // Font size
        let legend_x = plot.margin as i32 + 10;
        let mut legend_y = plot.margin as i32 + 10;

        for (_, color, label, _) in &plot.datasets {
            // Draw legend box
            for x in legend_x..legend_x + 20 {
                for y in legend_y..legend_y + 20 {
                    if x >= 0 && x < plot.width as i32 && y >= 0 && y < plot.height as i32 {
                        img.put_pixel(x as u32, y as u32, Rgb(*color));
                    }
                }
            }

            // Draw legend text
            draw_text_mut(
                img,
                Rgb(plot.font_color),
                legend_x + 30,
                legend_y,
                scale,
                &font,
                label,
            );

            legend_y += 30; // Move to the next legend item
        }
    }

    pub fn render(plot: &mut plot::Plot) -> RgbImage {
        let mut img = RgbImage::new(plot.width, plot.height);

        // Draw a white background
        for pixel in img.pixels_mut() {
            *pixel = Rgb(plot.background_color);
        }

        if plot.background_with_grid {
            Drawing::draw_grid(plot, &mut img);
        }

        // Match on the plot type
        match plot.plot_type {
            PlotType::CartesianGraph => {
                Drawing::draw_axes(plot, &mut img);
                Drawing::draw_lines_with_thickness(plot, &mut img);
                Drawing::draw_legend(plot, &mut img);
                Drawing::draw_axis_labels(plot, &mut img);
                Drawing::draw_x_labels(plot, &mut img);
                Drawing::draw_y_labels(plot, &mut img);
            }
            PlotType::BarChart => {
                Drawing::draw_bars(plot, &mut img);
            }
            PlotType::PieChart => {
                // Future implementation for pie charts
            }
        }

        // Common elements for all plot types
        Drawing::draw_title(plot, &mut img);
        img
    }

    ///
    /// * `img` - A mutable reference to the image where the grid lines will be drawn.
    ///
    /// This function draws the grid lines on the image based on the specified grid style.
    fn draw_grid(plot: &mut plot::Plot, img: &mut RgbImage) {
        // Scaling factors adjusted for margins
        let scale_x = (plot.width as f64 - 2.0 * plot.margin as f64) / (plot.x_max - plot.x_min);
        let scale_y = (plot.height as f64 - 2.0 * plot.margin as f64) / (plot.y_max - plot.y_min);

        // Draw vertical grid lines
        let mut x = plot.x_min;
        while x <= plot.x_max {
            let x_pixel = ((x - plot.x_min) * scale_x + plot.margin as f64) as u32;
            for y in plot.margin..(plot.height - plot.margin) {
                img.put_pixel(x_pixel, y, Rgb(plot.grid_color));
            }
            x += (plot.x_max - plot.x_min) / 10.0; // Adjust grid spacing here
        }

        // Draw horizontal grid lines
        let mut y = plot.y_min;
        while y <= plot.y_max {
            let y_pixel =
                (plot.height as f64 - plot.margin as f64 - ((y - plot.y_min) * scale_y)) as u32;
            for x in plot.margin..(plot.width - plot.margin) {
                img.put_pixel(x, y_pixel, Rgb(plot.grid_color));
            }
            y += (plot.y_max - plot.y_min) / 10.0; // Adjust grid spacing here
        }
    }

    // Draws labels for the x-axis and y-axis
    fn draw_axis_labels(plot: &mut plot::Plot, img: &mut RgbImage) {
        let font =
            FontRef::try_from_slice(include_bytes!("../../resources/fonts/Arial.ttf")).unwrap(); // Provide the path to your font file
        let scale = PxScale { x: 20.0, y: 20.0 }; // Font size

        let (_w_x, h_x) = text_size(scale, &font, &plot.xlabel);
        let (w_y, _h_y) = text_size(scale, &font, &plot.ylabel);

        // Draw y-axis label
        draw_text_mut(
            img,
            Rgb(plot.font_color),
            (plot.width - plot.margin) as i32 + scale.x as i32, // Left margin
            ((plot.height - h_x) / 2) as i32,                   // Adjust position to center
            scale,
            &font,
            &plot.xlabel,
        );

        // Draw x-axis label
        draw_text_mut(
            img,
            Rgb(plot.font_color),
            (plot.width - w_y) as i32 / 2, // Adjust position to center
            (plot.height - plot.margin / 2) as i32, // Slightly above the bottom margin
            scale,
            &font,
            &plot.ylabel,
        );
    }

    // Saves the rendered plot to a file
    pub fn save(plot: &mut plot::Plot, filename: &str) {
        let img = Drawing::render(plot);
        img.save(filename).unwrap();
    }
}

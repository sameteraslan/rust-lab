use super::{plot, PlotConfig};
use super::renderer::Renderer;
use crate::drawings::Drawing;
use crate::DisplayablePlot;
use image::RgbImage;
use imageproc::drawing::{draw_line_segment_mut, draw_text_mut};
use minifb::{Key, MouseMode, Window, WindowOptions};
use std::thread;
use std::time::Duration;

pub struct Winop;

impl Winop {
    pub fn new() -> Self {
        Self
    }

    // Displays the plot in a separate window
    pub fn display_with_window<T: Renderer>(plot: &mut T) {
        let mut img = RgbImage::new(plot.width(), plot.height());
        plot.render_plot(&mut img);

        let width = plot.width() as usize;
        let height = plot.height() as usize;

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
            "Plot Display",
            width,
            height,
            WindowOptions {
                resize: false,
                scale: minifb::Scale::X1,
                ..WindowOptions::default()
            },
        )
        .unwrap_or_else(|e| panic!("Unable to open Window: {}", e));

        // Display the image
        while window.is_open() && !window.is_key_pressed(Key::Escape, minifb::KeyRepeat::No) {
            window.update_with_buffer(&buffer, width, height).unwrap();
        }
    }

    pub fn display<T: DisplayablePlot + Renderer>(plot: &mut T) {
        let mut width = plot.width() as usize;
        let mut height = plot.height() as usize;
        let title = plot.title();
        let mut window = Window::new(
        title,
            width as usize,
            height as usize,
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
            if current_size != (width as usize, height as usize) {
                width = current_size.0;
                height = current_size.1;
            }

            let mut img = RgbImage::new(width as u32, height as u32);
            plot.render_plot(&mut img);

            // Show hints if enabled
            if show_hints {
                let hints = vec![
                    "H: Toggle Hints",
                    "C: Toggle hover mode",
                    "NumPad +: Zoom in",
                    "NumPad -: Zoom out",
                    "Escape: Exit",
                ];

                let font = plot.get_font();
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
                    let base_plot = plot.as_mut_base_plot();
                    for dataset in &base_plot.datasets {
                        if let Some(closest_point) = dataset.find_closest_point(base_plot, mouse_x, mouse_y) {
                            // Display the closest point coordinates on the image
                            draw_text_mut(
                                &mut img,
                                image::Rgb([0, 0, 0]), // Text color
                                mouse_x as i32,
                                mouse_y as i32 - 15,
                                ab_glyph::PxScale { x: 12.0, y: 12.0 },
                                &base_plot.get_font(),
                                &format!("({:.2}, {:.2})", closest_point.0, closest_point.1),
                            );

                            // Draw an arrow from the closest point to the mouse position
                            let scale_x = (base_plot.x_max - base_plot.x_min)
                                / (width as u32 - 2 * base_plot.margin) as f64;
                            let scale_y = (base_plot.y_max - base_plot.y_min)
                                / (height as u32 - 2 * base_plot.margin) as f64;

                            let px = ((closest_point.0 - base_plot.x_min) / scale_x + base_plot.margin as f64)
                                as f32;
                            let py = (height as f64
                                - base_plot.margin as f64
                                - (closest_point.1 - base_plot.y_min) / scale_y)
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
            }

            let buffer: Vec<u32> = img
                .pixels()
                .map(|pixel| {
                    let [r, g, b] = pixel.0;
                    (r as u32) << 16 | (g as u32) << 8 | (b as u32) // Convert RGB to u32
                })
                .collect();

            window
                .update_with_buffer(&buffer, width, height)
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
                Winop::zoom(plot.as_mut_base_plot(), 0.9); // Zoom in
            }
            if window.is_key_pressed(Key::NumPadMinus, minifb::KeyRepeat::No) {
                Winop::zoom(plot.as_mut_base_plot(), 1.1); // Zoom out
            }
        }
    }

    // // Finds the closest data point to the given pixel position
    // fn find_closest_point(plot: &mut PlotConfig, mouse_x: u32, mouse_y: u32) -> Option<(f64, f64)> {
    //     let scale_x = (plot.x_max - plot.x_min) / (plot.width - 2 * plot.margin) as f64;
    //     let scale_y = (plot.y_max - plot.y_min) / (plot.height - 2 * plot.margin) as f64;

    //     let mut closest_point = None;
    //     let mut min_distance = f64::MAX;

    //     for (dataset, _, _, _) in &plot.datasets {
    //         for &(x, y) in dataset {
    //             let px = ((x - plot.x_min) / scale_x + plot.margin as f64) as u32;
    //             let py =
    //                 (plot.height as f64 - plot.margin as f64 - (y - plot.y_min) / scale_y) as u32;

    //             let distance = ((px as i32 - mouse_x as i32).pow(2)
    //                 + (py as i32 - mouse_y as i32).pow(2)) as f64;

    //             if distance < min_distance {
    //                 min_distance = distance;
    //                 closest_point = Some((x, y));
    //             }
    //         }
    //     }

    //     closest_point
    // }

    // Adjusts the zoom level
    fn zoom(plot: &mut PlotConfig, factor: f64) {
        let x_center = (plot.x_min + plot.x_max) / 2.0;
        let y_center = (plot.y_min + plot.y_max) / 2.0;

        let x_range = (plot.x_max - plot.x_min) * factor;
        let y_range = (plot.y_max - plot.y_min) * factor;

        plot.x_min = x_center - x_range / 2.0;
        plot.x_max = x_center + x_range / 2.0;
        plot.y_min = y_center - y_range / 2.0;
        plot.y_max = y_center + y_range / 2.0;
    }

    // pub fn display_real_time<F>(plot: &mut PlotConfig, mut data_generator: F, fps: u64)
    // where
    //     F: FnMut() -> (f64, f64),
    // {
    //     let mut window = Window::new(
    //         &plot.title,
    //         plot.width as usize,
    //         plot.height as usize,
    //         WindowOptions {
    //             resize: true,
    //             scale: minifb::Scale::X1,
    //             ..WindowOptions::default()
    //         },
    //     )
    //     .unwrap_or_else(|e| panic!("Unable to open Window: {}", e));

    //     let mut hover_enabled = false; // Track whether hover is enabled
    //     let mut show_hints = false; // Track whether hints are enabled

    //     while window.is_open() && !window.is_key_pressed(Key::Escape, minifb::KeyRepeat::No) {
    //         // Check if the window size has changed and update plot dimensions
    //         let current_size = window.get_size();
    //         if current_size != (plot.width as usize, plot.height as usize) {
    //             plot.width = current_size.0 as u32;
    //             plot.height = current_size.1 as u32;
    //         }

    //         // Generate new data and update the dataset
    //         let (new_x, new_y) = data_generator();
    //         if let Some(dataset) = plot.datasets.first_mut() {
    //             dataset.0.push((new_x, new_y));

    //             // Remove old data points to keep the graph within bounds
    //             if dataset.0.len() > 500 {
    //                 dataset.0.remove(0);
    //             }
    //         }
    
    //         let mut img = RgbImage::new(plot.width, plot.height);
    //         Renderer::render_plot(plot, &mut img); // Render the plot with current settings
    //         Renderer::render_legend(plot, &mut img); // Render the plot with current settings

    //         // Show hints if enabled
    //         if show_hints {
    //             let hints = vec![
    //                 "H: Toggle hover mode",
    //                 "NumPad +: Zoom in",
    //                 "NumPad -: Zoom out",
    //                 "Escape: Exit",
    //             ];

    //             let font = plot.get_font();
    //             let mut y_offset = 20; // Start drawing hints slightly down from the top

    //             for hint in hints {
    //                 draw_text_mut(
    //                     &mut img,
    //                     image::Rgb([0, 0, 0]), // Text color
    //                     10,                    // X position
    //                     y_offset,              // Y position
    //                     ab_glyph::PxScale { x: 12.0, y: 12.0 },
    //                     &font,
    //                     hint,
    //                 );
    //                 y_offset += 20; // Move to the next line
    //             }
    //         }

    //         // Check if hover is enabled and process mouse events
    //         if hover_enabled {
    //             if let Some(mouse_pos) = window.get_mouse_pos(MouseMode::Pass) {
    //                 let (mouse_x, mouse_y) = (mouse_pos.0 as u32, mouse_pos.1 as u32);

    //                 // Find the closest point to the mouse
    //                 if let Some((closest_x, closest_y)) =
    //                     Winop::find_closest_point(plot, mouse_x, mouse_y)
    //                 {
    //                     // Display the closest point coordinates on the image
    //                     draw_text_mut(
    //                         &mut img,
    //                         image::Rgb([0, 0, 0]), // Text color
    //                         mouse_x as i32,
    //                         mouse_y as i32 - 15,
    //                         ab_glyph::PxScale { x: 12.0, y: 12.0 },
    //                         &plot.get_font(),
    //                         &format!("({:.2}, {:.2})", closest_x, closest_y),
    //                     );

    //                     // Draw an arrow from the closest point to the mouse position
    //                     let scale_x =
    //                         (plot.x_max - plot.x_min) / (plot.width - 2 * plot.margin) as f64;
    //                     let scale_y =
    //                         (plot.y_max - plot.y_min) / (plot.height - 2 * plot.margin) as f64;

    //                     let px = ((closest_x - plot.x_min) / scale_x + plot.margin as f64) as f32;
    //                     let py = (plot.height as f64
    //                         - plot.margin as f64
    //                         - (closest_y - plot.y_min) / scale_y)
    //                         as f32;

    //                     draw_line_segment_mut(
    //                         &mut img,
    //                         (px, py),
    //                         (mouse_x as f32, mouse_y as f32),
    //                         image::Rgb([255, 0, 0]), // Arrow color
    //                     );
    //                 }
    //             }
    //         }

    //         let buffer: Vec<u32> = img
    //             .pixels()
    //             .map(|pixel| {
    //                 let [r, g, b] = pixel.0;
    //                 (r as u32) << 16 | (g as u32) << 8 | (b as u32) // Convert RGB to u32
    //             })
    //             .collect();

    //         window
    //             .update_with_buffer(&buffer, plot.width as usize, plot.height as usize)
    //             .unwrap();

    //         // Handle hover toggle with the "H" key
    //         if window.is_key_pressed(Key::C, minifb::KeyRepeat::No) {
    //             hover_enabled = !hover_enabled; // Toggle hover mode
    //         }

    //         // Handle hints toggle with the "I" key
    //         if window.is_key_pressed(Key::H, minifb::KeyRepeat::No) {
    //             show_hints = !show_hints; // Toggle hints display
    //         }

    //         // Handle zooming
    //         if window.is_key_pressed(Key::NumPadPlus, minifb::KeyRepeat::No) {
    //             Winop::zoom(plot, 0.9); // Zoom in
    //         }
    //         if window.is_key_pressed(Key::NumPadMinus, minifb::KeyRepeat::No) {
    //             Winop::zoom(plot, 1.1); // Zoom out
    //         }

    //         // Limit update rate to control real-time rendering speed
    //         thread::sleep(Duration::from_millis(1000 / fps));
    //     }
    // }
}

use eframe::egui;
use rand::Rng;
use egui_plot::{Plot, Line, Value, Values};
use std::time::Instant; // Import Instant


struct RealTimeChart {
    data: Vec<(f64, f64)>, // Stores (time, value) pairs
    start_time: Instant,
}

impl Default for RealTimeChart {
    fn default() -> Self {
        Self {
            data: Vec::new(),
            start_time: Instant::now(),
        }
    }
}

impl RealTimeChart {
    fn update_data(&mut self) {
        let elapsed = self.start_time.elapsed().as_secs_f64();
        let value = rand::thread_rng().gen_range(0.0..100.0);
        self.data.push((elapsed, value));

        // Keep the last 100 data points
        if self.data.len() > 100 {
            self.data.remove(0);
        }
    }
}

impl eframe::App for RealTimeChart {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Real-Time Chart");

            // Convert the data into points for plotting
            let points: Vec<[f64; 2]> = self
                .data
                .iter()
                .map(|&(x, y)| [x, y])
                .collect();

            Plot::new("real_time_chart")
            Plot::new("real_time_chart")
                .show(ui, |plot_ui| {
                    plot_ui.line(egui_plot::Line::new(egui_plot::Values::from_values_iter(
                        points.iter().map(|&[x, y]| Value::new(x, y)),
                    )));
                });
        });
    }
}

fn main() {
    let options = eframe::NativeOptions {
        ..Default::default()
    };

    eframe::run_native(
        "Real-Time Chart in Rust",
        options,
        Box::new(|_cc| Ok(Box::new(RealTimeChart::default()))),
    );
}
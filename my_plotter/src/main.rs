use my_plotter::{LineStyle, Plot}; // Import the custom Plot library and LineStyle
use std::f64::consts::PI;
use std::thread;

fn main() {
    // Generate data for sin(x) and cos(x)
    let num_points = 1000;
    let step = 2.0 * PI / num_points as f64;

    let sin_data: Vec<(f64, f64)> = (0..=num_points)
        .map(|i| {
            let x = -PI + i as f64 * step;
            (x, x.sin())
        })
        .collect();

    let cos_data: Vec<(f64, f64)> = (0..num_points)
        .map(|i| {
            let x = -PI + i as f64 * step;
            (x, x.cos())
        })
        .collect();

        let handle1 = thread::spawn(|| {
        // Create the plot
        let mut plot = Plot::new()
            .width(800)
            .height(600)
            .x_max(PI)
            .y_max(1.0)
            .margin(100)
            .line_thickness(1)
            .title("Sine and Cosine")
            .xlabel("x")
            .ylabel("f(x)")
            .add_dataset(sin_data, [255, 0, 0], "sin(x)", LineStyle::Dashed) // Red for sin(x)
            .add_dataset(cos_data, [0, 0, 255], "cos(x)", LineStyle::DashDot) // Blue for cos(x)
            .add_dataset(
                vec![
                    (-1.0, 0.0),
                    (0.0, 1.0),
                    (1.0, 0.0),
                    (0.0, -1.0),
                    (-1.0, 0.0),
                ],
                [0, 255, 0],
                "diamond",
                LineStyle::Dotted,
            ) // Green line
            .add_dataset(
                vec![
                    (-1.6, 0.0),
                    (0.0, 1.0),
                    (1.6, 0.0),
                    (0.0, -1.0),
                    (-1.6, 0.0),
                ],
                [50, 50, 50],
                "diamond 2",
                LineStyle::Dotted,
            ); // custom line

        // Save the plot with grid and labels to a file
        plot.save_with_grid("sin_wave_with_grid.png");
        plot.save("sin_wave.png");

        plot.display();
    });

    let handle2 = thread::spawn(|| {
        let mut plot = Plot::new()
            .width(800)
            .height(600)
            .title("Real-Time Graph")
            .xlabel("Time")
            .ylabel("Value")
            .add_dataset(vec![], [255, 0, 0], "Data", LineStyle::Solid);

        // Simulate real-time data
        let mut t: f64 = 0.0;
        let data_generator = || {
            t += 0.1;
            (t, (t * 2.0).sin()) // Generate sine wave data
        };

        plot.display_real_time(data_generator);
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
}

# Plotter Library -> WIP!
This project is a simple yet customizable 2D plotting library written in Rust. It enables the creation of plots with customizable dimensions, colors, axis labels, and data points. The rendered plots can be saved as image files.

## Features
- **Customizable plot dimensions**: Specify the width and height of the plot.
- **Color customization**: Define colors for the background, axes, and data lines.
- **Data visualization**: Plot data points with adjustable line thickness.
- **Dynamic scaling**: Automatically scale data points to fit within the defined axes.
- **Margin support**: Add margins around the plot for better visualization.
- **Save functionality**: Save the rendered plot as an image file.

## Usage

### Example Code
```rust
use my_plotter::Plot;
use rand::Rng;

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

    handle1.join().unwrap();
}
```

### Output
The above code generates a plot and saves it as `plot.png` in the current working directory.

![](screenshots/plot.png)
![](screenshots/sin_wave.png)
![](screenshots/sin_wave_with_grid.png)

## Customization Options
- **Title**: Set the title of the plot.
- **X and Y Labels**: Label the x-axis and y-axis.
- **Dimensions**: Specify the width and height of the plot.
- **Colors**: Customize background, axis, and line colors.
- **Margins**: Add margins around the plot.
- **Line Thickness**: Adjust the thickness of data lines.


## Installation

Add the following to your `Cargo.toml`:
```toml
[dependencies]
ab_glyph = "0.2.29"
image = "0.25"    # For rendering images
imageproc = "0.25.0"
minifb = "0.27.0"
nalgebra = "0.30" # For handling data (optional)
rand = "0.8.5"    # For testing (optional)
rusttype = "0.9.3"
```

## License
This project is licensed under the GNU Version 3 License.

## Contributing
Contributions are welcome! Feel free to open issues or submit pull requests to improve the library.

---

Happy plotting! 🚀

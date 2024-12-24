use my_plotter::Plot; // Import the custom Plot library
use rand::Rng; // Import random number generation functionality

fn main() {
    // Set the number of points to generate for the plot
    let points_capacity = 5;

    // Create vectors to store x and y coordinates, pre-allocating memory for efficiency
    let mut points_x: Vec<f64> = Vec::with_capacity(points_capacity);
    let mut points_y: Vec<f64> = Vec::with_capacity(points_capacity);

    // Generate random x and y coordinates within specified ranges
    for _i in 0..points_capacity {
        points_x.push(rand::thread_rng().gen_range(-500.0..500.0)); // Random x between -500 and 500
        points_y.push(rand::thread_rng().gen_range(-100.0..100.0)); // Random y between -100 and 100
    }

    // Create a new plot instance with customized properties
    let plot = Plot::new()
        .title("Simple Plot") // Set the plot title
        .xlabel("X") // Label for the x-axis
        .ylabel("Y") // Label for the y-axis
        .width(800) // Width of the plot in pixels
        .height(600) // Height of the plot in pixels
        .bg_color(&[255, 255, 255]) // Set background color to white
        .axis_color(&[100, 150, 200]) // Set axis color to a light blue
        .line_thickness(2) // Set the thickness of the data lines
        .add_data(&points_x, &points_y) // Add the generated random points to the plot
        .x_max(500.0) // Set the maximum x-axis value
        .y_max(100.0); // Set the maximum y-axis value

    // Save the rendered plot to a file
    plot.save("plot.png");
}

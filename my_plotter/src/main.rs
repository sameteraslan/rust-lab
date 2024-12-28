use my_plotter::dataset::Dataset;
use my_plotter::drawer::Drawer;
use my_plotter::plot::barchart::BarChart;
use my_plotter::plot::bardataset::BarDataset;
use my_plotter::plot::canvas::Canvas;
use my_plotter::plot::cartesiangraph::CartesianGraph;
use my_plotter::plot::cartesiangraphdataset::CartesianDataset;
use my_plotter::plot::linetype::LineType;
use my_plotter::plot::orientation::Orientation;
use my_plotter::plot::piechart::PieChart;
use my_plotter::plot::scatterdottype::ScatterDotType;
use my_plotter::quadrant1graph::Quadrant1Graph;
use my_plotter::scattergraph::ScatterGraph;
use my_plotter::scattergraphdataset::ScatterGraphDataset;
use rand::Rng;
use std::f64::consts::PI;

fn main() {
    // Initialize the canvas
    let mut canvas = Canvas::new(800, 600, [255, 255, 255], 80);
    let mut bar_chart = BarChart::new("Yearly Income", "Year", "Income", Orientation::Vertical);

    let mut dataset1 = BarDataset::new("Company A", [220, 0, 0]);
    dataset1.add_data(2020.0, 100.0);
    dataset1.add_data(2021.0, 200.0);
    dataset1.add_data(2022.0, 150.0);

    let mut dataset2 = BarDataset::new("Company B", [0, 220, 0]);
    dataset2.add_data(2020.0, 120.0);
    dataset2.add_data(2021.0, 180.0);
    dataset2.add_data(2022.0, 220.0);

    let mut dataset3 = BarDataset::new("Company C", [0, 0, 220]);
    dataset3.add_data(2020.0, 150.0);
    dataset3.add_data(2021.0, 250.0);
    dataset3.add_data(2022.0, 400.0);

    let mut dataset4 = BarDataset::new("Company D", [150, 100, 50]);
    dataset4.add_data(2020.0, 50.0);
    dataset4.add_data(2021.0, 256.0);
    dataset4.add_data(2022.0, 40.0);

    bar_chart.add_dataset(dataset1);
    bar_chart.add_dataset(dataset2);
    bar_chart.add_dataset(dataset3);
    bar_chart.add_dataset(dataset4);

    bar_chart.draw(&mut canvas);
    canvas.save_as_image("grouped_vertical_bar_chart.png");

    // Initialize the canvas
    let mut canvas = Canvas::new(800, 600, [255, 255, 255], 80);
    let mut bar_chart = BarChart::new("Yearly Income", "Year", "Income", Orientation::Horizontal);

    let mut dataset1 = BarDataset::new("Company A", [220, 0, 0]);
    dataset1.add_data(2020.0, 100.0);
    dataset1.add_data(2021.0, 200.0);
    dataset1.add_data(2022.0, 150.0);

    let mut dataset2 = BarDataset::new("Company B", [0, 220, 0]);
    dataset2.add_data(2020.0, 120.0);
    dataset2.add_data(2021.0, 180.0);
    dataset2.add_data(2022.0, 220.0);

    let mut dataset3 = BarDataset::new("Company C", [0, 0, 220]);
    dataset3.add_data(2020.0, 150.0);
    dataset3.add_data(2021.0, 250.0);
    dataset3.add_data(2022.0, 400.0);

    let mut dataset4 = BarDataset::new("Company D", [150, 100, 50]);
    dataset4.add_data(2020.0, 50.0);
    dataset4.add_data(2021.0, 256.0);
    dataset4.add_data(2022.0, 40.0);

    bar_chart.add_dataset(dataset1);
    bar_chart.add_dataset(dataset2);
    bar_chart.add_dataset(dataset3);
    bar_chart.add_dataset(dataset4);

    bar_chart.draw(&mut canvas);
    canvas.save_as_image("grouped_horizontal_bar_chart.png");

    // Create a CartesianGraph
    let mut cartesian_graph = CartesianGraph::new("Math Functions", "X Axis", "Y Axis");

    // Add datasets to CartesianGraph
    let mut sine_wave = CartesianDataset::new([0, 0, 255], "sin(x)", LineType::Dashed(10));
    let mut cosine_wave = CartesianDataset::new([255, 0, 0], "cos(x)", LineType::Solid);

    // Add datasets
    let mut line1 = CartesianDataset::new([0, 0, 220], "line1", LineType::Solid);
    let mut line2 = CartesianDataset::new([220, 0, 0], "line2", LineType::Dashed(50));
    let mut line3 = CartesianDataset::new([0, 220, 0], "line3", LineType::Dotted(50));
    let mut line4 = CartesianDataset::new([150, 100, 50], "line4", LineType::Solid);
    let mut line5 = CartesianDataset::new([0, 0, 220], "line1", LineType::Solid);
    let mut line6 = CartesianDataset::new([220, 0, 0], "line2", LineType::Dashed(100));
    let mut line7 = CartesianDataset::new([0, 220, 0], "line3", LineType::Dotted(100));
    let mut line8 = CartesianDataset::new([150, 100, 50], "line4", LineType::Solid);

    let num_points = 10;
    // let step = 4.0 * std::f64::consts::PI / num_points as f64;
    for x in 0..=num_points {
        let xf = x as f64;
        line1.add_point((xf, xf));
        line2.add_point((xf, xf * 2.0));
        line3.add_point((xf, xf * 3.0));
        line4.add_point((xf, xf * 4.0));
        line5.add_point((xf, xf));
        line6.add_point((-xf, xf * 2.0));
        line7.add_point((-2.0 * xf, -xf * 3.0));
        line8.add_point((xf, xf * 4.0));
    }

    let num_points = 1000;
    let step = 2.0 * PI / num_points as f64;
    for x in 0..=num_points {
        let xf = -PI + x as f64 * step;
        sine_wave.add_point((xf, xf.sin()));
        cosine_wave.add_point((xf, xf.cos()));
    }

    cartesian_graph.add_dataset(sine_wave);
    cartesian_graph.add_dataset(cosine_wave);
    cartesian_graph.add_dataset(line5);
    cartesian_graph.add_dataset(line6);
    cartesian_graph.add_dataset(line7);
    cartesian_graph.add_dataset(line8);

    // Draw CartesianGraph
    cartesian_graph.draw(&mut canvas);
    cartesian_graph.draw_legend(&mut canvas);
    canvas.save_as_image("cartesian_graph.png");

    // Initialize canvas
    let mut canvas = Canvas::new(800, 600, [255, 255, 255], 80);

    // Create a Quadrant1Graph
    let mut quadrant1_graph = Quadrant1Graph::new("Quadrant 1 Graph", "X Axis", "Y Axis");

    quadrant1_graph.add_dataset(line1);
    quadrant1_graph.add_dataset(line2);
    quadrant1_graph.add_dataset(line3);
    quadrant1_graph.add_dataset(line4);

    // Draw the Quadrant1Graph
    quadrant1_graph.draw(&mut canvas);
    canvas.save_as_image("quadrant1_graph.png");

    // Pie Chart
    let mut canvas = Canvas::new(800, 600, [255, 255, 255], 80);
    let mut pie_chart = PieChart::new("Market Share");

    pie_chart.add_slice("Company A", 30.0, [220, 0, 0]);
    pie_chart.add_slice("Company B", 45.0, [0, 220, 0]);
    pie_chart.add_slice("Company C", 25.0, [0, 0, 220]);

    pie_chart.draw(&mut canvas);
    canvas.save_as_image("pie_chart.png");

    // Scatter Graph
    let mut canvas = Canvas::new(800, 600, [255, 255, 255], 80);
    let mut scatter_graph = ScatterGraph::new("Data Points", "X", "Y");
    let mut rng = rand::thread_rng();

    let mut dataset1 =
        ScatterGraphDataset::new([220, 0, 0], "Dataset 1", ScatterDotType::Circle(2));
    let mut dataset2 =
        ScatterGraphDataset::new([0, 220, 0], "Dataset 2", ScatterDotType::Cross(5));
    let mut dataset3 =
        ScatterGraphDataset::new([0, 0, 220], "Dataset 3", ScatterDotType::Square(5));
    let mut dataset4 =
        ScatterGraphDataset::new([50, 50, 50], "Dataset 4", ScatterDotType::Triangle(5));

    for _ in 0..500 {
        let x = rng.gen_range(0.0..3.0);
        let y = rng.gen_range(0.5..2.0);
        dataset1.add_point((x, y));
    }

    for _ in 0..500 {
        let x = rng.gen_range(0.0..3.0);
        let y = rng.gen_range(0.0..2.0);
        dataset2.add_point((x, y));
    }

    for _ in 0..500 {
        let x = rng.gen_range(0.0..3.0);
        let y = rng.gen_range(2.0..3.0);
        dataset3.add_point((x, y));
    }

    for _ in 0..500 {
        let x = rng.gen_range(1.5..3.0);
        let y = rng.gen_range(1.5..3.0);
        dataset4.add_point((x, y));
    }

    scatter_graph.add_dataset(dataset1);
    scatter_graph.add_dataset(dataset2);
    scatter_graph.add_dataset(dataset3);
    scatter_graph.add_dataset(dataset4);

    scatter_graph.draw(&mut canvas);
    canvas.save_as_image("scatter_graph.png");
}

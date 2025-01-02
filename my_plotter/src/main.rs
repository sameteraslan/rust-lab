use my_plotter::figure::canvas::pixelcanvas::PixelCanvas;
use my_plotter::figure::canvas::svgcanvas::SvgCanvas;
use my_plotter::figure::configuration::figureconfig::FigureConfig;
use my_plotter::figure::datasets::areachartdataset::AreaChartDataset;
use my_plotter::figure::datasets::bardataset::BarDataset;
use my_plotter::figure::datasets::cartesiangraphdataset::CartesianDataset;
use my_plotter::figure::datasets::dataset::Dataset;
use my_plotter::figure::datasets::scattergraphdataset::ScatterGraphDataset;
use my_plotter::figure::display::winop::Winop;
use my_plotter::figure::drawers::drawer::Drawer;
use my_plotter::figure::figuretypes::areachart::AreaChart;
use my_plotter::figure::figuretypes::cartesiangraph::CartesianGraph;
use my_plotter::figure::figuretypes::groupbarchart::GroupBarChart;
use my_plotter::figure::figuretypes::histogram::Histogram;
use my_plotter::figure::figuretypes::piechart::PieChart;
use my_plotter::figure::figuretypes::quadrant1graph::Quadrant1Graph;
use my_plotter::figure::figuretypes::scattergraph::ScatterGraph;
use my_plotter::figure::utilities::linetype::LineType;
use my_plotter::figure::utilities::orientation::Orientation;
use my_plotter::figure::utilities::scatterdottype::ScatterDotType;
use rand::Rng;
use std::f64::consts::PI;
use std::thread;

fn main() {
    // Initialize the PixelCanvas
    let figure_config = FigureConfig {
        font_size_title: 20.0,
        font_size_label: 16.0,
        font_size_legend: 14.0,
        color_axis: [0, 0, 0],
        color_background: [0, 0, 0],
        color_grid: [220, 220, 220],
        num_axis_ticks: 20,
        num_grid_horizontal: 20,
        num_grid_vertical: 20,
        font_label: "C:/Users/samet/Desktop/Rust/rust-lab/my_plotter/resources/fonts/Arial.ttf"
            .to_string(),
        font_title: "C:/Users/samet/Desktop/Rust/rust-lab/my_plotter/resources/fonts/Arial.ttf"
            .to_string(),
        ..Default::default()
    };

    let mut pixel_canvas = PixelCanvas::new(800, 600, [255, 255, 255], 80);
    let mut bar_chart = GroupBarChart::new(
        "Yearly Income",
        "Year",
        "Income",
        Orientation::Horizontal,
        figure_config.clone(),
    );

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

    bar_chart.draw(&mut pixel_canvas);
    pixel_canvas.save_as_image("grouped_vertical_bar_chart.png");

    // // Initialize the PixelCanvas
    // let mut pixel_canvas = PixelCanvas::new(800, 600, [255, 255, 255], 80);
    // let mut bar_chart = BarChart::new("Yearly Income", "Year", "Income", Orientation::Horizontal);

    // let mut dataset1 = BarDataset::new("Company A", [220, 0, 0]);
    // dataset1.add_data(2020.0, 100.0);
    // dataset1.add_data(2021.0, 200.0);
    // dataset1.add_data(2022.0, 150.0);

    // let mut dataset2 = BarDataset::new("Company B", [0, 220, 0]);
    // dataset2.add_data(2020.0, 120.0);
    // dataset2.add_data(2021.0, 180.0);
    // dataset2.add_data(2022.0, 220.0);

    // let mut dataset3 = BarDataset::new("Company C", [0, 0, 220]);
    // dataset3.add_data(2020.0, 150.0);
    // dataset3.add_data(2021.0, 250.0);
    // dataset3.add_data(2022.0, 400.0);

    // let mut dataset4 = BarDataset::new("Company D", [150, 100, 50]);
    // dataset4.add_data(2020.0, 50.0);
    // dataset4.add_data(2021.0, 256.0);
    // dataset4.add_data(2022.0, 40.0);

    // bar_chart.add_dataset(dataset1);
    // bar_chart.add_dataset(dataset2);
    // bar_chart.add_dataset(dataset3);
    // bar_chart.add_dataset(dataset4);

    // bar_chart.draw(&mut pixel_canvas);
    // pixel_canvas.save_as_image("grouped_horizontal_bar_chart.png");

    // Create a CartesianGraph
    let mut cartesian_graph =
        CartesianGraph::new("Math Functions", "X Axis", "Y Axis", &figure_config.clone());

    // Add datasets to CartesianGraph
    let mut sine_wave = CartesianDataset::new([0, 0, 255], "sin(x)", LineType::Dashed(10));
    let mut cosine_wave = CartesianDataset::new([255, 0, 0], "cos(x)", LineType::Solid);

    // Add datasets
    let mut line5 = CartesianDataset::new([0, 0, 220], "line1", LineType::Solid);
    let mut line6 = CartesianDataset::new([220, 0, 0], "line2", LineType::Dashed(100));
    let mut line7 = CartesianDataset::new([0, 220, 0], "line3", LineType::Dotted(100));
    let mut line8 = CartesianDataset::new([150, 100, 50], "line4", LineType::Solid);

    let num_points = 10;
    let step = 2.0 * PI / num_points as f64;
    for x in 0..=num_points {
        let xf = -PI + x as f64 * step;
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
    cartesian_graph.draw(&mut pixel_canvas);
    cartesian_graph.draw_legend(&mut pixel_canvas);
    pixel_canvas.save_as_image("cartesian_graph.png");

    // // Initialize PixelCanvas
    // let mut pixel_canvas = PixelCanvas::new(800, 600, [255, 255, 255], 80);

    // // Create a Quadrant1Graph
    // let mut quadrant1_graph = Quadrant1Graph::new("Quadrant 1 Graph", "X Axis", "Y Axis");

    // quadrant1_graph.add_dataset(line1);
    // quadrant1_graph.add_dataset(line2);
    // quadrant1_graph.add_dataset(line3);
    // quadrant1_graph.add_dataset(line4);

    // // Draw the Quadrant1Graph
    // quadrant1_graph.draw(&mut pixel_canvas);
    // pixel_canvas.save_as_image("quadrant1_graph.png");

    // // Pie Chart
    // let mut pixel_canvas = PixelCanvas::new(800, 600, [255, 255, 255], 80);
    // let mut pie_chart = PieChart::new("Market Share");

    // pie_chart.add_slice("Company A", 30.0, [220, 0, 0]);
    // pie_chart.add_slice("Company B", 45.0, [0, 220, 0]);
    // pie_chart.add_slice("Company C", 25.0, [0, 0, 220]);

    // pie_chart.draw(&mut pixel_canvas);
    // pixel_canvas.save_as_image("pie_chart.png");

    // // Scatter Graph
    // let mut pixel_canvas = PixelCanvas::new(800, 600, [255, 255, 255], 80);
    // let mut scatter_graph = ScatterGraph::new("Data Points", "X", "Y");
    // let mut rng = rand::thread_rng();

    // let mut dataset1 =
    //     ScatterGraphDataset::new([220, 0, 0], "Dataset 1", ScatterDotType::Circle(2));
    // let mut dataset2 = ScatterGraphDataset::new([0, 220, 0], "Dataset 2", ScatterDotType::Cross(5));
    // let mut dataset3 =
    //     ScatterGraphDataset::new([0, 0, 220], "Dataset 3", ScatterDotType::Square(5));
    // let mut dataset4 =
    //     ScatterGraphDataset::new([50, 50, 50], "Dataset 4", ScatterDotType::Triangle(5));

    // for _ in 0..500 {
    //     let x = rng.gen_range(0.0..3.0);
    //     let y = rng.gen_range(0.5..2.0);
    //     dataset1.add_point((x, y));
    // }

    // for _ in 0..500 {
    //     let x = rng.gen_range(0.0..3.0);
    //     let y = rng.gen_range(0.0..2.0);
    //     dataset2.add_point((x, y));
    // }

    // for _ in 0..500 {
    //     let x = rng.gen_range(0.0..3.0);
    //     let y = rng.gen_range(2.0..3.0);
    //     dataset3.add_point((x, y));
    // }

    // for _ in 0..500 {
    //     let x = rng.gen_range(1.5..3.0);
    //     let y = rng.gen_range(1.5..3.0);
    //     dataset4.add_point((x, y));
    // }

    // scatter_graph.add_dataset(dataset1);
    // scatter_graph.add_dataset(dataset2);
    // scatter_graph.add_dataset(dataset3);
    // scatter_graph.add_dataset(dataset4);

    // scatter_graph.draw(&mut pixel_canvas);
    // pixel_canvas.save_as_image("scatter_graph.png");

    // // Area Chart

    let mut pixel_canvas = PixelCanvas::new(800, 600, [255, 255, 255], 80);
    let mut area_chart = AreaChart::new(
        "Area Chart Example",
        "X Axis",
        "Y Axis",
        figure_config.clone(),
    );

    let mut dataset1 = AreaChartDataset::new([220, 0, 0], "Dataset 1", 0.5);
    dataset1.add_point((0.0, 0.0));
    dataset1.add_point((1.0, 2.0));
    dataset1.add_point((2.0, 1.0));
    dataset1.add_point((3.0, 3.0));

    let mut dataset2 = AreaChartDataset::new([0, 220, 0], "Dataset 2", 0.5);
    dataset2.add_point((0.0, 1.0));
    dataset2.add_point((1.0, 1.5));
    dataset2.add_point((2.0, 0.5));
    dataset2.add_point((3.0, 2.0));

    let mut dataset3 = AreaChartDataset::new([0, 0, 220], "Dataset 3", 0.5);
    dataset3.add_point((0.0, 2.5));
    dataset3.add_point((1.0, 0.5));
    dataset3.add_point((2.0, 0.5));
    dataset3.add_point((3.0, 1.5));

    area_chart.add_dataset(dataset1);
    area_chart.add_dataset(dataset2);
    area_chart.add_dataset(dataset3);

    area_chart.draw(&mut pixel_canvas);
    pixel_canvas.save_as_image("area_chart.png");

    // // Histogram

    // Generate random data
    let mut rng = rand::thread_rng();
    let data: Vec<f64> = (0..1000).map(|_| rng.gen_range(-3.0..3.0)).collect();

    // Create a Histogram
    let mut histogram = Histogram::new(
        "Histogram Example",
        "Values",
        "Frequency",
        30,
        [135, 206, 250], // Skyblue
        figure_config.clone(),
    );
    histogram.add_data_vec(data);

    // Draw the Histogram
    let mut pixel_canvas = PixelCanvas::new(800, 600, [255, 255, 255], 80); // White background

    histogram.draw(&mut pixel_canvas);
    pixel_canvas.save_as_image("histogram.png");

    // // Winop::display_with_window(&mut pixel_canvas, "Histogram Example");
    // Winop::display_interactive(&mut pixel_canvas, &mut histogram, "Interactive Histogram");

    // let mut pixel_canvas = PixelCanvas::new(800, 600, [255, 255, 255], 80);
    // let mut bar_chart = BarChart::new("Yearly Income", "Year", "Income", Orientation::Vertical);

    // let mut dataset1 = BarDataset::new("Company A", [220, 0, 0]);
    // dataset1.add_data(2020.0, 100.0);
    // dataset1.add_data(2021.0, 200.0);
    // dataset1.add_data(2022.0, 150.0);

    // let mut dataset2 = BarDataset::new("Company B", [0, 220, 0]);
    // dataset2.add_data(2020.0, 120.0);
    // dataset2.add_data(2021.0, 180.0);
    // dataset2.add_data(2022.0, 220.0);

    // let mut dataset3 = BarDataset::new("Company C", [0, 0, 220]);
    // dataset3.add_data(2020.0, 150.0);
    // dataset3.add_data(2021.0, 250.0);
    // dataset3.add_data(2022.0, 400.0);

    // let mut dataset4 = BarDataset::new("Company D", [150, 100, 50]);
    // dataset4.add_data(2020.0, 50.0);
    // dataset4.add_data(2021.0, 256.0);
    // dataset4.add_data(2022.0, 40.0);

    // bar_chart.add_dataset(dataset1);
    // bar_chart.add_dataset(dataset2);
    // bar_chart.add_dataset(dataset3);
    // bar_chart.add_dataset(dataset4);

    // bar_chart.draw(&mut pixel_canvas);
    // pixel_canvas.save_as_image("grouped_vertical_bar_chart.png");

    // let handle1 = thread::spawn(move || {
    //     let mut rng = rand::thread_rng();
    //     // Closure to update data
    //     let update_data = move |chart: &mut BarChart| {
    //         for i in 0..chart.datasets.len() {
    //             for point in chart.datasets[i].data.iter_mut() {
    //                 point.1 += rng.gen_range(-20.0..30.0); // Increment y-value
    //             }
    //         }
    //     };

    //     // Display the bar chart in real-time
    //     Winop::display_real_time(
    //         &mut PixelCanvas,
    //         &mut bar_chart,
    //         "Real-Time Bar Chart",
    //         update_data,
    //         30,
    //     );
    // });

    // let mut pixel_canvas = PixelCanvas::new(800, 600, [255, 255, 255], 80);
    // let mut cartesian_graph = CartesianGraph::new("Real-Time Cartesian Graph", "X", "Y");

    // let sine_wave = CartesianDataset::new([0, 0, 255], "sin(x)", LineType::Solid);
    // cartesian_graph.add_dataset(sine_wave);

    // let handle2 = thread::spawn(move || {
    //     // Closure to update sine wave data
    //     let mut x_value: f64 = 0.0;
    //     let update_data = move |graph: &mut CartesianGraph| {
    //         for i in 0..graph.datasets.len() {
    //             let y: f64 = x_value.sin();
    //             graph.datasets[i].add_point((x_value, y));
    //         }
    //         x_value += 0.1;
    //         graph.update_range();
    //     };

    //     // Display the Cartesian graph in real-time
    //     Winop::display_real_time(
    //         &mut PixelCanvas,
    //         &mut cartesian_graph,
    //         "Real-Time Cartesian Graph",
    //         update_data,
    //         60,
    //     );
    // });

    // let mut pixel_canvas = PixelCanvas::new(800, 600, [255, 255, 255], 80);
    // let mut pie_chart = PieChart::new("Real-Time Market Share");

    // pie_chart.add_slice("Company A", 33.0, [220, 0, 0]);
    // pie_chart.add_slice("Company B", 33.0, [0, 220, 0]);
    // pie_chart.add_slice("Company C", 33.0, [0, 0, 220]);

    // let handle3 = thread::spawn(move || {
    //     let mut rng = rand::thread_rng();
    //     let update_data = move |chart: &mut PieChart| {
    //         chart.datasets[0].1 += rng.gen_range(0.0..2.0); // Increment Company A's share
    //         chart.datasets[1].1 += rng.gen_range(0.0..2.0); // Increment Company B's share
    //         chart.datasets[2].1 += rng.gen_range(0.0..2.0); // Increment Company C's share
    //     };

    //     // Display the pie chart in real-time
    //     Winop::display_real_time(
    //         &mut PixelCanvas,
    //         &mut pie_chart,
    //         "Real-Time Pie Chart",
    //         update_data,
    //         10,
    //     );
    // });

    // let handle4 = thread::spawn(move || {
    //     let mut pixel_canvas = PixelCanvas::new(800, 600, [255, 255, 255], 80);
    //     let mut scatter_graph = ScatterGraph::new("Real-Time Scatter Graph", "X", "Y");

    //     let dataset1 = ScatterGraphDataset::new([0, 220, 0], "Data1", ScatterDotType::Circle(5));
    //     let dataset2 = ScatterGraphDataset::new([220, 0, 0], "Data2", ScatterDotType::Square(5));
    //     let dataset3 = ScatterGraphDataset::new([0, 0, 220], "Data3", ScatterDotType::Triangle(5));
    //     let mut rng = rand::thread_rng();
    //     scatter_graph.add_dataset(dataset1);
    //     scatter_graph.add_dataset(dataset2);
    //     scatter_graph.add_dataset(dataset3);

    //     // Closure to update scatter graph data
    //     let update_data = move |graph: &mut ScatterGraph| {
    //         for i in 0..graph.datasets.len() {
    //             let x = rng.gen_range(0.0..10.0);
    //             let y = rng.gen_range(0.0..10.0);
    //             graph.datasets[i].add_point((x, y));
    //         }
    //     };

    //     // Display the scatter graph in real-time
    //     Winop::display_real_time(
    //         &mut PixelCanvas,
    //         &mut scatter_graph,
    //         "Real-Time Scatter Graph",
    //         update_data,
    //         30,
    //     );
    // });

    // // histrogram
    // let handle5 = thread::spawn(move || {
    //     let mut rng = rand::thread_rng();
    //     let data: Vec<f64> = (0..1000).map(|_| rng.gen_range(-3.0..3.0)).collect();

    //     // Create a Histogram
    //     let mut histogram = Histogram::new(
    //         "Histogram Example",
    //         "Values",
    //         "Frequency",
    //         30,
    //         [135, 206, 250], // Skyblue
    //     );
    //     histogram.add_data_vec(data);

    //     // Draw the Histogram
    //     let mut pixel_canvas = PixelCanvas::new(800, 600, [255, 255, 255], 80);
    //     // White background

    //     let update_data = move |graph: &mut Histogram| {
    //         graph.add_data(rng.gen_range(-3.0..3.0));
    //     };

    //     // Winop::display_with_window(&mut pixel_canvas, "Histogram Example");
    //     // Display the histogram graph in real-time
    //     Winop::display_real_time(
    //         &mut PixelCanvas,
    //         &mut histogram,
    //         "Real-Time Histogram Graph",
    //         update_data,
    //         30,
    //     );
    // });

    // let handle6 = thread::spawn(move || {
    //     let mut pixel_canvas = PixelCanvas::new(800, 600, [255, 255, 255], 80);
    //     let mut bar_chart =
    //         BarChart::new("Yearly Income", "Year", "Income", Orientation::Horizontal);

    //     let mut dataset1 = BarDataset::new("Company A", [220, 0, 0]);
    //     dataset1.add_data(2020.0, 100.0);
    //     dataset1.add_data(2021.0, 200.0);
    //     dataset1.add_data(2022.0, 150.0);

    //     let mut dataset2 = BarDataset::new("Company B", [0, 220, 0]);
    //     dataset2.add_data(2020.0, 120.0);
    //     dataset2.add_data(2021.0, 180.0);
    //     dataset2.add_data(2022.0, 220.0);

    //     let mut dataset3 = BarDataset::new("Company C", [0, 0, 220]);
    //     dataset3.add_data(2020.0, 150.0);
    //     dataset3.add_data(2021.0, 250.0);
    //     dataset3.add_data(2022.0, 400.0);

    //     let mut dataset4 = BarDataset::new("Company D", [150, 100, 50]);
    //     dataset4.add_data(2020.0, 50.0);
    //     dataset4.add_data(2021.0, 256.0);
    //     dataset4.add_data(2022.0, 40.0);

    //     bar_chart.add_dataset(dataset1);
    //     bar_chart.add_dataset(dataset2);
    //     bar_chart.add_dataset(dataset3);
    //     bar_chart.add_dataset(dataset4);

    //     bar_chart.draw(&mut pixel_canvas);
    //     pixel_canvas.save_as_image("grouped_horizontal_bar_chart.png");

    //     // let mut start_time = Instant::now();
    //     let mut rng = rand::thread_rng();
    //     // Closure to update data
    //     let update_data = move |chart: &mut BarChart| {
    //         for i in 0..chart.datasets.len() {
    //             for point in chart.datasets[i].data.iter_mut() {
    //                 point.1 += rng.gen_range(-20.0..30.0); // Increment x-value
    //             }
    //         }
    //     };

    //     // Display the bar chart in real-time
    //     Winop::display_real_time(
    //         &mut PixelCanvas,
    //         &mut bar_chart,
    //         "Real-Time Bar Chart",
    //         update_data,
    //         30,
    //     );
    // });

    // handle1.join().unwrap();
    // handle2.join().unwrap();
    // handle3.join().unwrap();
    // handle4.join().unwrap();
    // handle5.join().unwrap();
    // handle6.join().unwrap();

    let mut graph = CartesianGraph::new("Example Graph", "X Axis", "Y Axis", &figure_config);

    // Add datasets to the graph
    let mut dataset1 = CartesianDataset::new([220, 0, 0], "Dataset1", LineType::Solid);
    let mut dataset2 = CartesianDataset::new([0, 220, 0], "Dataset2", LineType::Solid);
    let mut dataset3 = CartesianDataset::new([0, 0, 220], "Dataset3", LineType::Solid);

    let num_points = 1000;
    let step = 4.0 * std::f64::consts::PI / num_points as f64;

    for x in 0..=num_points {
        let xf = -PI + x as f64 * step;
        dataset1.add_point((xf, xf));
        dataset2.add_point((xf, 3.0 * xf.sin()));
        dataset3.add_point((xf, 3.0 * xf.cos()));
    }
    graph.add_dataset(dataset1);
    graph.add_dataset(dataset2);
    graph.add_dataset(dataset3);

    // Render as pixel PixelCanvas
    let mut pixel_canvas = PixelCanvas::new(800, 600, [255, 255, 255], 80);
    graph.draw(&mut pixel_canvas);
    pixel_canvas.save_as_image("output.png");

    // Render as SVG
    let mut svg_canvas = SvgCanvas::new(1000, 1000, "white", 80);
    graph.draw_svg(&mut svg_canvas);
    svg_canvas.save("output.svg").unwrap();

    let mut pixel_canvas = SvgCanvas::new(800, 600, "white", 80);
    let mut area_chart = AreaChart::new(
        "Area Chart Example",
        "X Axis",
        "Y Axis",
        figure_config.clone(),
    );

    let mut dataset1 = AreaChartDataset::new([220, 0, 0], "Dataset 1", 0.5);
    dataset1.add_point((0.0, 0.0));
    dataset1.add_point((1.0, 2.0));
    dataset1.add_point((2.0, 1.0));
    dataset1.add_point((3.0, 3.0));

    let mut dataset2 = AreaChartDataset::new([0, 220, 0], "Dataset 2", 0.5);
    dataset2.add_point((0.0, 1.0));
    dataset2.add_point((1.0, 1.5));
    dataset2.add_point((2.0, 0.5));
    dataset2.add_point((3.0, 2.0));

    let mut dataset3 = AreaChartDataset::new([0, 0, 220], "Dataset 3", 0.5);
    dataset3.add_point((0.0, 2.5));
    dataset3.add_point((1.0, 0.5));
    dataset3.add_point((2.0, 0.5));
    dataset3.add_point((3.0, 1.5));

    area_chart.add_dataset(dataset1);
    area_chart.add_dataset(dataset2);
    area_chart.add_dataset(dataset3);

    area_chart.draw_svg(&mut pixel_canvas);
    pixel_canvas.save("area_chart.svg").unwrap();

    let mut line1 = CartesianDataset::new([0, 0, 220], "line1", LineType::Solid);
    let mut line2 = CartesianDataset::new([220, 0, 0], "line2", LineType::Dashed(50));
    let mut line3 = CartesianDataset::new([0, 220, 0], "line3", LineType::Dotted(50));
    let mut line4 = CartesianDataset::new([150, 100, 50], "line4", LineType::Solid);

    let num_points = 10;
    // let step = 4.0 * std::f64::consts::PI / num_points as f64;
    for x in 0..=num_points {
        let xf = x as f64;
        line1.add_point((xf, xf));
        line2.add_point((xf, xf * 2.0));
        line3.add_point((xf, xf * 3.0));
        line4.add_point((xf, xf * 4.0));
    }

    // Initialize PixelCanvas
    let mut pixel_canvas = SvgCanvas::new(800, 600, "white", 80);

    // Create a Quadrant1Graph
    let mut quadrant1_graph = Quadrant1Graph::new(
        "Quadrant 1 Graph",
        "X Axis",
        "Y Axis",
        figure_config.clone(),
    );

    quadrant1_graph.add_dataset(line1);
    quadrant1_graph.add_dataset(line2);
    quadrant1_graph.add_dataset(line3);
    quadrant1_graph.add_dataset(line4);

    // Draw the Quadrant1Graph
    quadrant1_graph.draw_svg(&mut pixel_canvas);
    pixel_canvas.save("quadrant1_graph.svg").unwrap();

    // Scatter Graph
    let mut pixel_canvas = SvgCanvas::new(800, 600, "white", 80);
    let mut scatter_graph = ScatterGraph::new("Data Points", "X", "Y", figure_config.clone());
    let mut rng = rand::thread_rng();

    let mut dataset1 =
        ScatterGraphDataset::new([220, 0, 0], "Dataset 1", ScatterDotType::Circle(3));
    let mut dataset2 = ScatterGraphDataset::new([0, 220, 0], "Dataset 2", ScatterDotType::Cross(5));
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

    scatter_graph.draw_svg(&mut pixel_canvas);
    pixel_canvas.save("scatter_graph.svg").unwrap();

    // Generate random data
    let mut rng = rand::thread_rng();
    let data: Vec<f64> = (0..1000).map(|_| rng.gen_range(-3.0..3.0)).collect();

    // Create a Histogram
    let mut histogram = Histogram::new(
        "Histogram Example",
        "Values",
        "Frequency",
        30,
        [135, 206, 250], // Skyblue
        figure_config.clone(),
    );
    histogram.add_data_vec(data);

    // Draw the Histogram
    let mut pixel_canvas = SvgCanvas::new(800, 600, "white", 80); // White background

    histogram.draw_svg(&mut pixel_canvas);
    pixel_canvas.save("histogram.svg").unwrap();

    // Pie Chart
    let mut pixel_canvas = SvgCanvas::new(800, 600, "white", 80);
    let mut pie_chart = PieChart::new("Market Share", figure_config.clone());

    pie_chart.add_slice("Company A", 30.0, [220, 0, 0]);
    pie_chart.add_slice("Company B", 45.0, [0, 220, 0]);
    pie_chart.add_slice("Company C", 25.0, [0, 0, 220]);

    pie_chart.draw_svg(&mut pixel_canvas);
    pixel_canvas.save("pie_chart.svg").unwrap();

    // Initialize the PixelCanvas
    let mut pixel_canvas = SvgCanvas::new(800, 600, "white", 80);
    let mut bar_chart = GroupBarChart::new(
        "Yearly Income",
        "Year",
        "Income",
        Orientation::Vertical,
        FigureConfig::default(),
    );

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

    bar_chart.draw_svg(&mut pixel_canvas);
    pixel_canvas.save("grouped_vertical_bar_chart.svg").unwrap();

    // Initialize the PixelCanvas
    let figure_config = FigureConfig {
        font_size_title: 20.0,
        font_size_label: 16.0,
        font_size_legend: 14.0,
        color_axis: [0, 0, 0],
        color_background: [0, 0, 0],
        color_grid: [0, 0, 0],
        num_axis_ticks: 20,
        num_grid_horizontal: 20,
        num_grid_vertical: 20,
        font_label: "C:/Users/samet/Desktop/Rust/rust-lab/my_plotter/resources/fonts/Arial.ttf"
            .to_string(),
        font_title: "C:/Users/samet/Desktop/Rust/rust-lab/my_plotter/resources/fonts/Arial.ttf"
            .to_string(),
        ..Default::default()
    };
    let mut pixel_canvas = SvgCanvas::new(800, 600, "white", 80);
    let mut bar_chart = GroupBarChart::new(
        "Yearly Income",
        "Year",
        "Income",
        Orientation::Vertical,
        figure_config,
    );

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

    bar_chart.draw_svg(&mut pixel_canvas);
    pixel_canvas
        .save("grouped_horizontal_bar_chart.svg")
        .unwrap();

    let svg_text = pixel_canvas.get_svg_as_text();
    Winop::display_svg(&svg_text, "Bar Chart Example");
}

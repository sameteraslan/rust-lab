use serde_json::Value;
use std::sync::{Arc, Mutex};
use tokio::time::{sleep, Duration};
use rtplot::{Figure, PlotType};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // API endpoint and parameters
    let url = "https://fapi.binance.com/futures/data/takerlongshortRatio";
    let symbol = "BTCUSDT"; // Cryptocurrency pair
    let interval = "5m"; // Data interval
    let limit = 100; // Number of data points to fetch

    // Shared vector for storing ratio data (thread-safe)
    let ratio_data = Arc::new(Mutex::new(Vec::new()));

    // Clone for plotting thread
    let plot_data = Arc::clone(&ratio_data);

    // Clone for background data-fetching thread
    let data_updater = Arc::clone(&ratio_data);

    // Configure and initialize the plot
    let mut plot_figure = Figure::new(100) // Display 100 samples at a time
        .ylim([0.0, 3.0]) // Set Y-axis range based on expected values
        .xlabel("Time") // X-axis label
        .ylabel("Buy/Sell Ratio (BTCUSDT)") // Y-axis label
        .plot_type(PlotType::Line) // Line plot
        .color(0x80, 0x00, 0x80); // Purple color

    // Spawn a background thread to fetch and update data asynchronously
    std::thread::spawn(move || {
        // Create a Tokio runtime for async operations
        let rt = tokio::runtime::Runtime::new().unwrap();

        // Start the asynchronous event loop
        rt.block_on(async move {
            loop {
                // Fetch data from the Binance API
                let client = reqwest::Client::new();
                let response = client
                    .get(url)
                    .query(&[
                        ("symbol", symbol), // Symbol parameter
                        ("period", interval), // Period parameter
                        ("limit", &limit.to_string()), // Limit parameter
                    ])
                    .send()
                    .await
                    .expect("Failed to fetch data")
                    .text()
                    .await
                    .expect("Failed to read response");

                // Parse the response JSON
                let parsed_data: Value = serde_json::from_str(&response).expect("Invalid JSON");

                // Lock the shared vector to safely update data
                let mut ratio_lock = data_updater.lock().unwrap();
                ratio_lock.clear(); // Clear old data

                // Extract and parse the buy/sell ratios
                if let Some(data_array) = parsed_data.as_array() {
                    for item in data_array {
                        if let Some(buy_sell_ratio) = item["buySellRatio"].as_str() {
                            if let Ok(ratio) = buy_sell_ratio.parse::<f32>() {
                                ratio_lock.push(ratio); // Update the vector
                            }
                        }
                    }
                }

                // Wait for 5 seconds before the next fetch
                sleep(Duration::from_secs(5)).await;
            }
        });
    });

    // Start the plotting loop on the main thread
    Figure::display(&mut plot_figure, |fig| {
        // Lock the shared vector for plotting
        let current_ratios = plot_data.lock().unwrap();

        // Plot data only if it's available
        if !current_ratios.is_empty() {
            fig.plot_stream(&current_ratios);
        }

        // Sleep to control the plot refresh rate
        std::thread::sleep(Duration::from_secs(1));
    });

    Ok(())
}

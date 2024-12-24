# Rust Lab Repository

Welcome to the **Rust Lab** repository! This collection showcases various projects and experiments developed in Rust, focusing on practical implementations and demonstrating the language's power and versatility.

## Table of Contents

- [Introduction](#introduction)
- [Project Structure](#project-structure)
- [Getting Started](#getting-started)
- [Projects](#projects)
- [Contributing](#contributing)
- [License](#license)

## Introduction

This repository is a hands-on laboratory for experimenting with Rust. Each folder contains a unique project, exploring concepts ranging from system programming to web development and data visualization. Whether you're a beginner or an experienced developer, you'll find examples to learn and build upon.

## Project Structure

The repository is organized into the following folders:

### 1. `binance_analyze`
- **Description**: A real-time data visualization tool that fetches and plots cryptocurrency buy/sell ratios using the Binance API.
- **Key Features**:
  - Fetches live data every 5 seconds.
  - Displays a dynamic line chart using the `rtplot` library.

### 2. `web_server`
- **Description**: A lightweight http server implementation using Rust's `actix-web` library.
- **Key Features**:
  - Dynamic Routing.
  - Static File Serving.
  - Integration Tests.

### 3. `cli_snake_game`
- **Description**: This project implements a simple snake game using Rust. The game runs in the terminal and uses the crossterm library for handling input and output.
- **Key Features**:
  - Real-time snake movement with directional controls.
  - Collision detection with walls and self.
  - Dynamic game speed based on the score.
  - Logging game events to a file.
  - Customizable game configuration through a JSON file.


## Getting Started

1. **Clone the repository**:
   ```bash
   git clone https://github.com/sameteraslan/rust-lab.git
   cd rust-lab
   ```

2. **Navigate to a specific project**:
   ```bash
   cd cli_snake_game
   ```

3. **Build and run the project**:
   ```bash
   cargo build
   cargo run
   ```

## Projects

Each project is self-contained and includes:

- A `Cargo.toml` file with dependencies.
- Rust source code demonstrating key concepts.
- Readme files with detailed explanations (where applicable).

## Contributing

We welcome contributions! If you have an idea for a new project or improvements to an existing one, feel free to:

1. Fork this repository.
2. Create a new branch for your feature or fix.
3. Submit a pull request with a detailed description of your changes.

## License

This project is licensed under the GNU Version 3 License. See the [LICENSE](./LICENSE) file for details.

---

Enjoy exploring and experimenting with Rust in the **Rust Lab**!


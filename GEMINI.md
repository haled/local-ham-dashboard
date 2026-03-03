# Terminal HamClock (local-ham-dashboard)

A terminal and web-based dashboard for amateur radio operators, inspired by the original HamClock.

## Project Overview
This project provides a real-time dashboard for HAM radio enthusiasts. It displays essential data such as time, weather, solar activity, HF propagation, satellite passes, and a live news ticker. It supports two display modes: a Terminal UI (TUI) and an HTML-based Web UI.

### Main Technologies
- **Language:** Rust (Edition 2024)
- **TUI Framework:** [Ratatui](https://ratatui.rs/) with [Crossterm](https://github.com/crossterm-rs/crossterm)
- **Web Framework:** [Axum](https://github.com/tokio-rs/axum) for serving the HTML UI and a JSON API.
- **CLI Parsing:** [Clap](https://github.com/clap-rs/clap) for command-line arguments.
- **Async Runtime:** [Tokio](https://tokio.rs/)
- **Data Fetching:** [Reqwest](https://github.com/seanmonstar/reqwest) for HTTP, [RSS](https://github.com/mvdnes/rust-rss) for news.
- **Orbit Prediction:** [sgp4](https://github.com/utiasSTARS/sgp4)
- **Serialization:** [Serde](https://serde.rs/) and `serde_json`.

### Architecture
- **Central State:** `AppState` is wrapped in an `Arc<Mutex<AppState>>` and shared across tasks and the web server.
- **Background Tasks:** Each data source (Weather, Solar, News, Satellites) runs in its own `tokio::spawn` loop, fetching data at configurable intervals and updating the shared state.
- **Dual UI Support:**
  - **TUI Mode:** Renders a classic dashboard in the terminal using Ratatui.
  - **HTML Mode:** Starts a web server that serves static assets from `/static` and provides a JSON API at `/api/data`.
- **Modules:**
  - `src/weather.rs`: Fetches current weather from Open-Meteo.
  - `src/solar.rs`: Fetches solar indices and band propagation from HamQSL.
  - `src/news.rs`: Aggregates and parses RSS feeds.
  - `src/satellite.rs`: Predicts satellite passes using TLE data from Celestrak.
  - `src/map.rs`: Logic for calculating day/night terminator (Gray Line).

## Building and Running

### Prerequisites
- [Rust and Cargo](https://rustup.rs/) (latest stable)

### Configuration
1. Copy `config.yaml.example` to `config.yaml`.
2. Edit `config.yaml` to include your callsign, latitude, longitude, and preferred RSS feeds.

### Key Commands
- **Run (Default TUI):** `cargo run`
- **Run (Web UI):** `cargo run -- --ui html --port 3000`
- **Build:** `cargo build --release`
- **Test:** `cargo test`

## Development Conventions
- **Error Handling:** Use `color_eyre::Result` for application-level results.
- **Data Serialization:** All data structures in the shared state must implement `Serialize` to support the JSON API.
- **UI Components:** 
  - TUI logic is in `src/main.rs`.
  - Web UI assets are in the `/static` directory (`index.html`, `style.css`, `script.js`).
- **Configuration:** All new global settings should be added to the `Config` struct in `src/main.rs` and documented in `config.yaml.example`.

# Terminal HamClock (local-ham-dashboard)

A terminal-based dashboard for amateur radio operators, inspired by the original HamClock but optimized for the CLI using Rust and the Ratatui TUI library.

## Project Overview
This project provides a real-time dashboard in the terminal for HAM radio enthusiasts. It displays essential data such as time, weather, solar activity, HF propagation, satellite passes, and a live news ticker.

### Main Technologies
- **Language:** Rust (Edition 2024)
- **TUI Framework:** [Ratatui](https://ratatui.rs/) with [Crossterm](https://github.com/crossterm-rs/crossterm)
- **Async Runtime:** [Tokio](https://tokio.rs/)
- **Data Fetching:** [Reqwest](https://github.com/seanmonstar/reqwest) for HTTP, [RSS](https://github.com/mvdnes/rust-rss) for news
- **Orbit Prediction:** [sgp4](https://github.com/utiasSTARS/sgp4)
- **Error Handling:** [color-eyre](https://github.com/eyre-rs/color-eyre)

### Architecture
- **Central State:** `AppState` is wrapped in an `Arc<Mutex<AppState>>` and shared across tasks.
- **Async Tasks:** Each data source (Weather, Solar, News, Satellites) runs in its own `tokio::spawn` loop, fetching data at configurable intervals.
- **TUI Loop:** The main thread handles the TUI event loop, rendering the UI and processing user input (like quitting with 'q').
- **Modules:**
  - `src/weather.rs`: Fetches current weather from Open-Meteo.
  - `src/solar.rs`: Fetches solar indices and band propagation from HamQSL.
  - `src/news.rs`: Aggregates and parses RSS feeds.
  - `src/satellite.rs`: Predicts satellite passes using TLE data from Celestrak.
  - `src/map.rs`: Renders an ASCII world map with a day/night terminator.

## Building and Running

### Prerequisites
- [Rust and Cargo](https://rustup.rs/) (latest stable)

### Configuration
1. Copy `config.yaml.example` to `config.yaml`.
2. Edit `config.yaml` to include your callsign, latitude, longitude, and preferred RSS feeds.

### Key Commands
- **Build:** `cargo build`
- **Run:** `cargo run` (Ensure `config.yaml` is present in the root directory)
- **Test:** `cargo test`

## Development Conventions
- **Error Handling:** Use `color_eyre::Result` for application-level results.
- **Data Fetching:** New data sources should be implemented as async functions in their own modules.
- **UI Components:** UI logic is mostly contained in `src/main.rs` using Ratatui's layout and widget systems.
- **Configuration:** All new global settings should be added to the `Config` struct in `src/main.rs` and documented in `config.yaml.example`.

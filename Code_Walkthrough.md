# Code Walkthrough: Terminal HamClock

This document provides a detailed walkthrough of the Rust codebase for the Terminal HamClock project. It explains how the application is structured, how data is fetched asynchronously, and how the TUI is rendered.

---

## 1. Application Entry Point: `src/main.rs`

The `main.rs` file is the heart of the application. it handles configuration, global state, task spawning, and the main TUI loop.

### Configuration & State
- **`Config` Struct:** Defines the structure of `config.yaml`, including callsign, location (lat/lon), RSS feeds, and refresh intervals.
- **`AppState` Struct:** Holds the latest data for all dashboard blocks (Weather, Solar, News, Satellites). It's wrapped in `Arc<Mutex<AppState>>` to allow safe concurrent access from multiple background tasks and the main UI thread.

### Main Function (`async fn main`)
1. **Initialization:** Installs `color-eyre` for better error reporting and loads the configuration.
2. **Background Tasks:** Spawns several `tokio::spawn` loops. Each loop:
    - Fetches data from a specific module (e.g., `weather::fetch_weather`).
    - Updates the shared `AppState` with the new data.
    - Sleeps for a duration defined in the configuration.
3. **TUI Setup:** Enables raw mode and enters the alternate terminal screen using `crossterm`.
4. **Main Loop:** 
    - Updates a `scroll_offset` for the news ticker.
    - Calls `terminal.draw(|f| ui(f, &app_state))` to render the UI.
    - Polls for user input (specifically 'q' to quit).
    - Manages the tick rate to ensure smooth updates.

### UI Rendering (`fn ui`)
The UI is built using `Ratatui`'s layout system.
- **Layout:** The screen is split into three main vertical chunks: Top (Blocks A-D), Middle (Blocks E-F and the Map), and Bottom (News Ticker).
- **Widgets:** Each block (Identity, Weather, Propagation, Solar, Location, Satellites) is rendered as a `Paragraph` or `List` widget with custom styling and borders.
- **FIGlet Support:** The `figlet-rs` crate is used to render the callsign and grid square in a large, decorative font.

---

## 2. Weather Module: `src/weather.rs`

This module fetches real-time weather data.
- **Data Source:** [Open-Meteo](https://open-meteo.com/).
- **Functionality:** 
    - `fetch_weather`: Constructs a URL based on the user's latitude and longitude.
    - Uses `reqwest` to make an HTTP GET request and `serde` to deserialize the JSON response.
    - Returns a `WeatherData` struct containing temperature, humidity, and apparent temperature (wind chill).

---

## 3. Solar & Propagation Module: `src/solar.rs`

This module tracks solar activity and HF band conditions.
- **Data Source:** [HamQSL (N0NBH)](https://www.hamqsl.com/solarxml.php).
- **Functionality:**
    - `fetch_solar`: Fetches an XML feed containing solar indices (SFI, SN, etc.) and band status (Good, Fair, Poor).
    - **Manual Parsing:** Instead of a full XML parser, it uses a lightweight manual approach (`extract_tag` helper) to pull values from specific XML tags, which is faster and more robust for this specific simple XML structure.

---

## 4. News Module: `src/news.rs`

The news ticker at the bottom of the screen is powered by this module.
- **Data Source:** User-defined RSS feeds in `config.yaml`.
- **Functionality:**
    - `fetch_news`: Iterates through the list of URLs.
    - Uses the `rss` crate to parse the feed content and extract item titles.
    - Aggregates all headlines into a `NewsData` struct.

---

## 5. Satellite Tracking Module: `src/satellite.rs`

Predicts upcoming amateur radio satellite passes.
- **Data Source:** [Celestrak](https://celestrak.org/) (Amateur satellite TLEs).
- **Functionality:**
    - `fetch_and_predict`: Downloads Two-Line Element (TLE) sets.
    - **Prediction Logic:** Uses the `sgp4` crate to propagate the satellite's orbit from its epoch to the current time and future minutes.
    - **Visibility Calculation:** For each satellite, it calculates its current latitude/longitude and checks the elevation relative to the user's location.
    - Returns the next 5 passes that reach an elevation of at least 10°.

---

## 6. World Map & Gray Line: `src/map.rs`

Renders an ASCII world map showing day and night regions.
- **Logic:**
    - `is_daylight`: Calculates the solar declination and hour angle based on the current UTC time and date.
    - Uses the solar altitude angle formula to determine if a specific lat/lon coordinate is in daylight (altitude > 0).
- **Rendering:** In `main.rs`, the `render_world_map` function iterates over the available UI area, converts each character cell to lat/lon, checks daylight status, and fills the cell with either a dot (`.`) for day or a block (`█`) for night.

---

## 7. Project Dependencies (`Cargo.toml`)

- **`ratatui` & `crossterm`:** The foundation for the terminal user interface.
- **`tokio`:** The asynchronous runtime enabling concurrent data fetching.
- **`reqwest`:** For performing HTTP requests to external APIs.
- **`serde` & `serde_yaml`:** For configuration and JSON parsing.
- **`chrono`:** For all time-related calculations (essential for the clock, map, and satellite passes).
- **`sgp4`:** For orbital mechanics and satellite propagation.
- **`figlet-rs`:** For stylized text in the Identity and Location blocks.

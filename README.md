# Terminal HamClock

A terminal and web-based dashboard for amateur radio operators, inspired by the original HamClock. It's built with Rust and supports both a classic CLI TUI and a modern HTML interface.

![Terminal HamClock Demo](https://via.placeholder.com/800x400.png?text=Terminal+HamClock+Demo+Placeholder)

## Features

- **Dual Mode UI:** Choose between a classic terminal-based (Ratatui) dashboard or a modern web-based (HTML/CSS) kiosk display.
- **Identity & Time (Block A):** Stylized callsign display with a real-time UTC clock and date.
- **Local Weather (Block B):** Real-time temperature, humidity, and apparent temperature (wind chill) for your location via Open-Meteo.
- **HF Propagation (Block C):** Color-coded status for HF bands (80m-6m), fetched from HamQSL.
- **Solar Activity (Block D):** Key indices (SFI, SN, A-Index, K-Index, X-Ray) from HamQSL.
- **Station Location (Block E):** Latitude, Longitude, and a large Maidenhead Grid Square display.
- **Satellite Tracker (Block F):** Predicts the next 5 upcoming amateur satellite passes (elevation > 10°).
- **News Ticker (Block G):** A scrolling ticker merging headlines from your favorite RSS feeds with red separators.
- **World Map (Block H):** 
  - **TUI:** Real-time ASCII map showing the current day/night terminator (Gray Line).
  - **Web UI:** Color map with continental imagery and a semi-transparent night shadow.

## Installation

Ensure you have [Rust and Cargo](https://rustup.rs/) installed.

1. Clone the repository:
   ```bash
   git clone https://github.com/haled/local-ham-dashboard.git
   cd local-ham-dashboard
   ```

2. Configuration:
   ```bash
   cp config.yaml.example config.yaml
   # Edit config.yaml with your callsign, lat/lon, and other settings
   ```

3. Build the application:
   ```bash
   cargo build --release
   ```

## Usage

### Run Terminal UI (TUI)
This is the default mode. Launch it directly from your CLI:
```bash
cargo run --release
```

### Run Web UI (HTML)
Starts a local web server (default port 3000) that you can open in any browser:
```bash
cargo run --release -- --ui html
```
To run on a specific port:
```bash
cargo run --release -- --ui html --port 8080
```

### Running in Kiosk Mode (e.g., Raspberry Pi)
If you're using this as a dedicated display, launch the web UI and use a kiosk-compatible browser:
```bash
chromium-browser --kiosk http://localhost:3000
```

## Configuration

The application is configured via `config.yaml` in the project root:

```yaml
# Identity & Contact
callsign: "YOUR_CALLSIGN"
user_agent: "TerminalHamClock/0.1.0 (your-email@example.com)"

# Location Information
latitude: 38.6270
longitude: -90.1994
grid_square: "EM48"

# News Ticker (RSS Feeds)
news_rss_feeds:
  - "https://feeds.npr.org/1001/rss.xml"      # National News

# Refresh Intervals (in minutes unless specified)
refresh_intervals:
  clock_ms: 1000          # 1 second refresh
  weather_min: 10
  solar_min: 30
  news_min: 30
  satellite_min: 5
```

## Data Sources

- **Weather:** [Open-Meteo](https://open-meteo.com/)
- **Solar/Propagation:** [HamQSL (N0NBH)](https://www.hamqsl.com/solar.html)
- **Satellite TLEs:** [Celestrak](https://celestrak.org/)
- **News:** RSS feeds (NPR, etc.)

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

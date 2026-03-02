# Terminal HamClock

A terminal-based dashboard for amateur radio operators, inspired by the original HamClock but optimized for the CLI using Rust and the Ratatui TUI library.

![Terminal HamClock Demo](https://via.placeholder.com/800x400.png?text=Terminal+HamClock+Demo+Placeholder)

## Features

- **Identity & Time (Block A):** Large FIGlet-styled callsign with a 1-second UTC clock and date.
- **Local Weather (Block B):** Real-time temperature, humidity, and wind chill for your location via Open-Meteo.
- **HF Propagation (Block C):** Color-coded status (Green/Yellow/Red) for HF bands from 80m-6m, fetched from HamQSL.
- **Solar Activity (Block D):** Key indices (SFI, SN, A-Index, K-Index, X-Ray) from HamQSL.
- **Station Location (Block E):** Your Latitude, Longitude, and a large FIGlet-styled Maidenhead Grid Square.
- **Satellite Tracker (Block F):** Predicts the next 5 upcoming AMSAT satellite passes (AOS and Max Elevation > 10°).
- **News Ticker (Block G):** A horizontal scrolling ticker merging local and national headlines from RSS feeds.
- **World Map (Block H):** A real-time ASCII-based world map showing the current day/night terminator (Gray Line).

## Installation

Ensure you have [Rust and Cargo](https://rustup.rs/) installed.

1. Clone the repository:
   ```bash
   git clone https://github.com/your-username/local-ham-dashboard.git
   cd local-ham-dashboard
   ```

2. Build and run the application:
   ```bash
   cargo run
   ```

## Configuration

The application is configured via a `config.yaml` file in the project root. You can customize your callsign, location, and news sources:

```yaml
# Identity & Contact
callsign: "YOUR_CALLSIGN"
user_agent: "TerminalHamClock/0.1.0 (your-email@example.com)"

# Location Information
latitude: 0.0
longitude: 0.0
grid_square: "AA00aa"

# News Ticker (RSS Feeds)
news_rss_feeds:
  - "https://www.ksdk.com/feeds/rss/news/local" # St. Louis Local
  - "https://feeds.npr.org/1001/rss.xml"      # National

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
- **News:** NPR and KSDK (via RSS)

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

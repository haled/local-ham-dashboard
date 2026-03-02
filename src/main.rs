mod weather;
mod solar;
mod news;
mod map;
mod satellite;

use color_eyre::Result;
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph, ListItem, List},
};
use serde::Deserialize;
use std::{fs, io, sync::Arc, time::{Duration, Instant}};
use chrono::Utc;
use tokio::sync::Mutex;
use figlet_rs::FIGfont;

#[derive(Debug, Deserialize, Clone)]
struct Config {
    callsign: String,
    user_agent: String,
    latitude: f64,
    longitude: f64,
    grid_square: String,
    news_rss_feeds: Vec<String>,
    refresh_intervals: RefreshIntervals,
    units: String,
}

#[derive(Debug, Deserialize, Clone)]
struct RefreshIntervals {
    clock_ms: u64,
    weather_min: u64,
    solar_min: u64,
    news_min: u64,
    satellite_min: u64,
}

struct AppState {
    config: Config,
    weather: Option<weather::WeatherData>,
    solar: Option<solar::SolarData>,
    news: Option<news::NewsData>,
    satellites: Option<Vec<satellite::SatPass>>,
    scroll_offset: usize,
    font: Option<FIGfont>,
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let config: Config = serde_yaml::from_str(&fs::read_to_string("config.yaml")?)?;
    let font = FIGfont::standard().ok();

    let state = Arc::new(Mutex::new(AppState {
        config: config.clone(),
        weather: None,
        solar: None,
        news: None,
        satellites: None,
        scroll_offset: 0,
        font,
    }));

    enable_raw_mode()?;
    io::stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;

    let weather_state = Arc::clone(&state);
    let weather_config = config.clone();
    tokio::spawn(async move {
        loop {
            match weather::fetch_weather(weather_config.latitude, weather_config.longitude, &weather_config.user_agent).await {
                Ok(data) => {
                    let mut s = weather_state.lock().await;
                    s.weather = Some(data);
                }
                Err(e) => {
                    let _ = fs::write("error_weather.log", format!("{:?}", e));
                }
            }
            tokio::time::sleep(Duration::from_secs(weather_config.refresh_intervals.weather_min * 60)).await;
        }
    });

    let solar_state = Arc::clone(&state);
    let solar_config = config.clone();
    tokio::spawn(async move {
        loop {
            match solar::fetch_solar().await {
                Ok(data) => {
                    let mut s = solar_state.lock().await;
                    s.solar = Some(data);
                }
                Err(e) => {
                    let _ = fs::write("error_solar.log", format!("{:?}", e));
                }
            }
            tokio::time::sleep(Duration::from_secs(solar_config.refresh_intervals.solar_min * 60)).await;
        }
    });

    let news_state = Arc::clone(&state);
    let news_config = config.clone();
    tokio::spawn(async move {
        loop {
            if let Ok(data) = news::fetch_news(&news_config.news_rss_feeds).await {
                let mut s = news_state.lock().await;
                s.news = Some(data);
            }
            tokio::time::sleep(Duration::from_secs(news_config.refresh_intervals.news_min * 60)).await;
        }
    });

    let sat_state = Arc::clone(&state);
    let sat_config = config.clone();
    tokio::spawn(async move {
        loop {
            if let Ok(data) = satellite::fetch_and_predict(sat_config.latitude, sat_config.longitude).await {
                let mut s = sat_state.lock().await;
                s.satellites = Some(data);
            }
            tokio::time::sleep(Duration::from_secs(sat_config.refresh_intervals.satellite_min * 60)).await;
        }
    });

    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_millis(config.refresh_intervals.clock_ms);

    loop {
        let mut app_state = state.lock().await;
        app_state.scroll_offset = (app_state.scroll_offset + 3) % 10000;
        terminal.draw(|f| ui(f, &app_state))?;
        drop(app_state);

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
    }

    disable_raw_mode()?;
    io::stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

fn ui(f: &mut Frame, state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(12), 
            Constraint::Min(10),   
            Constraint::Length(3), 
        ])
        .split(f.area());

    let top_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(35), // Widened for Block A and E
            Constraint::Percentage(21),
            Constraint::Percentage(21),
            Constraint::Percentage(23),
        ])
        .split(chunks[0]);

    let middle_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(35), // Widened for Location block
            Constraint::Percentage(65),
        ])
        .split(chunks[1]);

    let left_middle_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(65), // Increased space for larger Location font
            Constraint::Percentage(35),
        ])
        .split(middle_chunks[0]);

    let border_style = Style::default().fg(Color::LightBlue);

    // Block A: Identity & Time
    let now = Utc::now();
    let time_str = now.format("%H:%M:%S").to_string();
    let date_str = now.format("%Y-%m-%d").to_string();
    
    let mut block_a_lines = Vec::new();
    if let Some(font) = &state.font {
        if let Some(fig) = font.convert(&state.config.callsign) {
            for line in fig.to_string().lines() {
                if !line.trim().is_empty() {
                    block_a_lines.push(Line::from(Span::styled(line.trim_end().to_string(), Style::default().fg(Color::Yellow))));
                }
            }
        }
    }

    block_a_lines.push(Line::from(Span::styled(format!("Time: {}", time_str), Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))));
    block_a_lines.push(Line::from(Span::styled(format!("Date: {}", date_str), Style::default().fg(Color::Yellow))));

    f.render_widget(
        Paragraph::new(block_a_lines)
            .block(Block::default().borders(Borders::ALL).title(" Identity ").border_style(border_style)),
        top_chunks[0],
    );

    // Block B: Weather
    let weather_text = if let Some(w) = &state.weather {
        format!("Temp:  {}\nHum:   {}\nChill: {}", w.temperature, w.humidity, w.wind_chill)
    } else {
        "Loading Weather...".to_string()
    };
    f.render_widget(
        Paragraph::new(weather_text)
            .block(Block::default().borders(Borders::ALL).title(" Weather ").border_style(border_style)),
        top_chunks[1],
    );

    // Block C: Propagation
    let prop_items: Vec<ListItem> = if let Some(s) = &state.solar {
        s.band_conditions.iter()
            .map(|(band, status)| {
                let color = match status.to_lowercase().as_str() {
                    "good" => Color::Green,
                    "fair" => Color::Yellow,
                    "poor" => Color::Red,
                    _ => Color::White,
                };
                ListItem::new(Line::from(vec![
                    Span::raw(format!("{}: ", band)),
                    Span::styled(status, Style::default().fg(color)),
                ]))
            })
            .collect()
    } else {
        vec![ListItem::new("Loading...")]
    };
    f.render_widget(
        List::new(prop_items)
            .block(Block::default().borders(Borders::ALL).title(" Propagation ").border_style(border_style)),
        top_chunks[2],
    );

    // Block D: Solar
    let solar_text = if let Some(s) = &state.solar {
        format!("SFI: {}\nSN:  {}\nA-Idx: {}\nK-Idx: {}\nX-Ray: {}", s.sfi, s.sn, s.a_index, s.k_index, s.x_ray)
    } else {
        "Loading...".to_string()
    };
    f.render_widget(
        Paragraph::new(solar_text)
            .block(Block::default().borders(Borders::ALL).title(" Solar ").border_style(border_style)),
        top_chunks[3],
    );

    // Block E: Location (Large Grid and Yellow)
    let mut loc_lines = Vec::new();
    if let Some(font) = &state.font {
        loc_lines.push(Line::from(Span::styled("GRID SQUARE:", Style::default().fg(Color::Yellow))));
        if let Some(fig) = font.convert(&state.config.grid_square) {
            for line in fig.to_string().lines() {
                if !line.trim().is_empty() {
                    loc_lines.push(Line::from(Span::styled(line.trim_end().to_string(), Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))));
                }
            }
        }
    }
    loc_lines.push(Line::from(Span::styled(format!("LAT: {}", state.config.latitude), Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))));
    loc_lines.push(Line::from(Span::styled(format!("LON: {}", state.config.longitude), Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))));

    f.render_widget(
        Paragraph::new(loc_lines)
            .block(Block::default().borders(Borders::ALL).title(" Location ").border_style(border_style)),
        left_middle_chunks[0],
    );

    // Block F: Satellite
    let sat_items: Vec<ListItem> = if let Some(sats) = &state.satellites {
        sats.iter()
            .map(|s| ListItem::new(format!("{} @ {} (Max: {:.0}°)", s.name, s.aos.format("%H:%M").to_string(), s.max_el)))
            .collect()
    } else {
        vec![ListItem::new("Calculating Passes...")]
    };
    f.render_widget(
        List::new(sat_items)
            .block(Block::default().borders(Borders::ALL).title(" Satellites ").border_style(border_style)),
        left_middle_chunks[1],
    );

    render_world_map(f, middle_chunks[1], border_style);

    // Block G: News Ticker
    let news_text = if let Some(n) = &state.news {
        let combined = n.headlines.join("  |  ");
        let start = state.scroll_offset % combined.len().max(1);
        format!("{}  |  {}", &combined[start..], &combined[..start])
    } else {
        "News: Loading RSS headlines...".to_string()
    };
    f.render_widget(
        Paragraph::new(news_text)
            .block(Block::default().borders(Borders::ALL).title(" News Ticker ").border_style(border_style)),
        chunks[2],
    );
}

fn render_world_map(f: &mut Frame, area: Rect, border_style: Style) {
    let now = Utc::now();
    let width = area.width as usize;
    let height = area.height as usize;
    let mut canvas = vec![vec![' '; width]; height];

    for y in 0..height {
        for x in 0..width {
            let lat = 90.0 - (y as f64 / height as f64) * 180.0;
            let lon = (x as f64 / width as f64) * 360.0 - 180.0;
            
            if map::is_daylight(lat, lon, now) {
                canvas[y][x] = '.'; // Day
            } else {
                canvas[y][x] = '█'; // Night
            }
        }
    }

    let map_lines: Vec<String> = canvas.into_iter().map(|row| row.into_iter().collect()).collect();
    let map_text = map_lines.join("\n");
    
    f.render_widget(
        Paragraph::new(map_text)
            .block(Block::default().borders(Borders::ALL).title(" World Map (Gray Line) ").border_style(border_style)),
        area,
    );
}

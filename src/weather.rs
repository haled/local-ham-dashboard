use serde::Deserialize;
use color_eyre::Result;

#[derive(Debug, Clone, Default)]
pub struct WeatherData {
    pub temperature: String,
    pub humidity: String,
    pub wind_chill: String,
}

#[derive(Deserialize)]
struct OpenMeteoResponse {
    current: CurrentWeather,
}

#[derive(Deserialize)]
struct CurrentWeather {
    temperature_2m: f64,
    relative_humidity_2m: i32,
    apparent_temperature: f64,
}

pub async fn fetch_weather(lat: f64, lon: f64, _user_agent: &str) -> Result<WeatherData> {
    // Open-Meteo is a simple one-shot API call
    let url = format!(
        "https://api.open-meteo.com/v1/forecast?latitude={:.4}&longitude={:.4}&current=temperature_2m,relative_humidity_2m,apparent_temperature&temperature_unit=fahrenheit",
        lat, lon
    );

    let client = reqwest::Client::new();
    let resp: OpenMeteoResponse = client.get(&url)
        .send().await?
        .json().await?;

    Ok(WeatherData {
        temperature: format!("{:.1}°F", resp.current.temperature_2m),
        humidity: format!("{}%", resp.current.relative_humidity_2m),
        wind_chill: format!("{:.1}°F", resp.current.apparent_temperature),
    })
}

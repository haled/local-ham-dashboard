use chrono::{DateTime, Utc, Timelike, Datelike};
use std::f64::consts::PI;

pub fn is_daylight(lat: f64, lon: f64, time: DateTime<Utc>) -> bool {
    let day_of_year = time.ordinal() as f64;
    let hour = time.hour() as f64 + (time.minute() as f64 / 60.0) + (time.second() as f64 / 3600.0);
    
    // Solar declination (approximate)
    let declination = 23.45 * (2.0 * PI * (284.0 + day_of_year) / 365.0).sin();
    let decl_rad = declination.to_radians();
    let lat_rad = lat.to_radians();

    // Solar time and hour angle
    let solar_time = (hour + lon / 15.0) % 24.0;
    let hour_angle = (15.0 * (solar_time - 12.0)).to_radians();

    // Solar altitude angle
    let altitude = (lat_rad.sin() * decl_rad.sin() + lat_rad.cos() * decl_rad.cos() * hour_angle.cos()).asin();
    
    altitude > 0.0
}

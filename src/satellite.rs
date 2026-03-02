use color_eyre::Result;
use chrono::{DateTime, Utc, TimeZone};
use sgp4::{Elements, Constants};

#[derive(Debug, Clone, Default)]
pub struct SatPass {
    pub name: String,
    pub aos: DateTime<Utc>,
    pub max_el: f64,
}

pub async fn fetch_and_predict(lat: f64, lon: f64) -> Result<Vec<SatPass>> {
    let url = "https://celestrak.org/NORAD/elements/gp.php?GROUP=amateur&FORMAT=tle";
    let body = reqwest::get(url).await?.text().await?;
    let lines: Vec<&str> = body.lines().collect();

    let mut passes = Vec::new();
    let now = Utc::now();

    for i in (0..lines.len()).step_by(3) {
        if i + 2 >= lines.len() { break; }
        
        let name = lines[i].trim().to_string();
        let line1 = lines[i+1].as_bytes();
        let line2 = lines[i+2].as_bytes();

        if let Ok(elements) = Elements::from_tle(Some(name.clone()), line1, line2) {
            if let Ok(constants) = Constants::from_elements(&elements) {
                // SGP4 elements.datetime is NaiveDateTime, convert to UTC
                let epoch_utc: DateTime<Utc> = Utc.from_utc_datetime(&elements.datetime);

                for minute in 0..120 {
                    let t = now + chrono::Duration::minutes(minute);
                    let diff = t.signed_duration_since(epoch_utc);
                    let minutes_since_epoch = diff.num_seconds() as f64 / 60.0;
                    
                    if let Ok(prediction) = constants.propagate(sgp4::MinutesSinceEpoch(minutes_since_epoch)) {
                        let (sat_lat, sat_lon, _) = eci_to_geodetic(prediction.position);
                        let el = calculate_elevation(lat, lon, sat_lat, sat_lon);
                        
                        if el > 10.0 {
                            passes.push(SatPass {
                                name: name.clone(),
                                aos: t,
                                max_el: el,
                            });
                            break;
                        }
                    }
                }
            }
        }
    }

    passes.sort_by_key(|p| p.aos);
    Ok(passes.into_iter().take(5).collect())
}

fn eci_to_geodetic(pos: [f64; 3]) -> (f64, f64, f64) {
    let x = pos[0];
    let y = pos[1];
    let z = pos[2];
    let r = (x*x + y*y + z*z).sqrt();
    let lat = (z / r).asin().to_degrees();
    let lon = y.atan2(x).to_degrees();
    (lat, lon, r - 6371.0)
}

fn calculate_elevation(user_lat: f64, user_lon: f64, sat_lat: f64, sat_lon: f64) -> f64 {
    let d_lat = (sat_lat - user_lat).to_radians();
    let d_lon = (sat_lon - user_lon).to_radians();
    let a = (d_lat/2.0).sin().powi(2) + user_lat.to_radians().cos() * sat_lat.to_radians().cos() * (d_lon/2.0).sin().powi(2);
    let c = 2.0 * a.sqrt().atan2((1.0-a).sqrt());
    let distance = 6371.0 * c;
    90.0 - (distance / 6371.0).to_degrees()
}

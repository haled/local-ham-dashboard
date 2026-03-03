use color_eyre::Result;
use serde::Serialize;

#[derive(Debug, Clone, Default, Serialize)]
pub struct SolarData {
    pub sfi: String,
    pub sn: String,
    pub a_index: String,
    pub k_index: String,
    pub x_ray: String,
    pub band_conditions: Vec<(String, String)>,
}

pub async fn fetch_solar() -> Result<SolarData> {
    let url = "https://www.hamqsl.com/solarxml.php";
    let body = reqwest::get(url).await?.text().await?;
    
    let mut data = SolarData::default();
    
    // Manual parsing to avoid complex XML schema issues
    for line in body.lines() {
        let line = line.trim();
        if line.contains("<solarflux>") {
            data.sfi = extract_tag(line, "solarflux");
        } else if line.contains("<sunspots>") {
            data.sn = extract_tag(line, "sunspots");
        } else if line.contains("<aindex>") {
            data.a_index = extract_tag(line, "aindex");
        } else if line.contains("<kindex>") {
            data.k_index = extract_tag(line, "kindex");
        } else if line.contains("<xray>") {
            data.x_ray = extract_tag(line, "xray");
        } else if line.contains("<band name=") {
            // Extract band condition: <band name="80m-40m" time="day">Fair</band>
            if let Some(name_start) = line.find("name=\"") {
                let rest = &line[name_start + 6..];
                if let Some(name_end) = rest.find("\"") {
                    let name = &rest[..name_end];
                    let status = extract_tag(line, "band");
                    data.band_conditions.push((name.to_string(), status));
                }
            }
        }
    }

    Ok(data)
}

fn extract_tag(line: &str, tag: &str) -> String {
    let start_tag = format!("<{}>", tag);
    let end_tag = format!("</{}>", tag);
    
    if let Some(start) = line.find(&start_tag) {
        if let Some(end) = line.find(&end_tag) {
            return line[start + start_tag.len()..end].trim().to_string();
        }
    }
    
    // Check for attributes in tags (e.g., <band name="...">)
    let start_tag_attr = format!("<{} ", tag);
    if let Some(start) = line.find(&start_tag_attr) {
        if let Some(content_start) = line[start..].find(">") {
            let actual_start = start + content_start + 1;
            if let Some(end) = line.find(&end_tag) {
                return line[actual_start..end].trim().to_string();
            }
        }
    }

    "N/A".to_string()
}

use color_eyre::Result;
use rss::Channel;
use serde::Serialize;

#[derive(Debug, Clone, Default, Serialize)]
pub struct NewsData {
    pub headlines: Vec<String>,
}

pub async fn fetch_news(feed_urls: &[String]) -> Result<NewsData> {
    let mut all_headlines = Vec::new();

    for url in feed_urls {
        if let Ok(content) = reqwest::get(url).await?.bytes().await {
            if let Ok(channel) = Channel::read_from(&content[..]) {
                for item in channel.items() {
                    if let Some(title) = item.title() {
                        all_headlines.push(title.to_string());
                    }
                }
            }
        }
    }

    Ok(NewsData { headlines: all_headlines })
}

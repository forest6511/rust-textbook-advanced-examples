use crate::types::{CrawlError, Page};
use reqwest::{Client, StatusCode};
use std::time::Duration;
use url::Url;

const USER_AGENT: &str = "rust-textbook-advanced-crawler/0.1 (+contact-url)";

pub fn build_client() -> reqwest::Result<Client> {
    Client::builder()
        .user_agent(USER_AGENT)
        .timeout(Duration::from_secs(30))
        .connect_timeout(Duration::from_secs(10))
        .pool_max_idle_per_host(8)
        .build()
}

pub async fn fetch(client: &Client, url: &Url) -> Result<Page, CrawlError> {
    let resp = client.get(url.as_str()).send().await?;
    let status = resp.status();
    if status == StatusCode::TOO_MANY_REQUESTS {
        let retry_after = parse_retry_after(resp.headers());
        return Err(CrawlError::RateLimited { retry_after });
    }
    if !status.is_success() {
        return Err(CrawlError::Status(status.as_u16()));
    }
    let html = resp.text().await?;
    Ok(Page {
        url: url.clone(),
        status: status.as_u16(),
        html,
        fetched_at: now_secs(),
    })
}

fn parse_retry_after(headers: &reqwest::header::HeaderMap) -> Option<Duration> {
    headers
        .get(reqwest::header::RETRY_AFTER)?
        .to_str()
        .ok()?
        .parse::<u64>()
        .ok()
        .map(Duration::from_secs)
}

fn now_secs() -> i64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}

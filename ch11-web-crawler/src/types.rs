use std::time::Duration;
use thiserror::Error;
use url::Url;

#[derive(Debug, Error)]
pub enum CrawlError {
    #[error("HTTP transport error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("status {0}")]
    Status(u16),

    #[error("rate limited (retry_after={retry_after:?})")]
    RateLimited { retry_after: Option<Duration> },
}

impl CrawlError {
    pub fn is_retryable(&self) -> bool {
        match self {
            CrawlError::Http(e) => e.is_timeout() || e.is_connect(),
            CrawlError::Status(c) => *c >= 500,
            CrawlError::RateLimited { .. } => true,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Page {
    pub url: Url,
    pub status: u16,
    pub html: String,
    pub fetched_at: i64,
}

#[derive(Debug, Clone)]
pub struct CrawlResult {
    pub page: Page,
    pub links: Vec<Url>,
}

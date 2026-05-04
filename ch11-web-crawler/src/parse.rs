use crate::types::{CrawlResult, Page};
use scraper::{Html, Selector};

pub struct LinkExtractor {
    selector: Selector,
}

impl LinkExtractor {
    pub fn new() -> Self {
        Self {
            selector: Selector::parse("a[href]")
                .expect("static selector compiles"),
        }
    }

    pub fn extract(&self, page: Page) -> CrawlResult {
        let doc = Html::parse_document(&page.html);
        let mut links = Vec::new();
        for el in doc.select(&self.selector) {
            let Some(href) = el.value().attr("href") else { continue };
            let Ok(joined) = page.url.join(href) else { continue };
            let mut u = joined;
            u.set_fragment(None);
            links.push(u);
        }
        CrawlResult { page, links }
    }
}

impl Default for LinkExtractor {
    fn default() -> Self {
        Self::new()
    }
}

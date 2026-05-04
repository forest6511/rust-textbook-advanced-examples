use anyhow::{Context, Result};
use ch11_web_crawler::{Config, run};
use std::path::PathBuf;
use tracing_subscriber::EnvFilter;
use url::Url;

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    let seeds: Vec<Url> = std::env::args()
        .skip(1)
        .map(|s| Url::parse(&s).context("invalid seed URL"))
        .collect::<Result<Vec<_>>>()?;
    if seeds.is_empty() {
        eprintln!("usage: crawler <seed-url> [<seed-url> ...]");
        std::process::exit(1);
    }

    let cfg = Config {
        seeds,
        db_path: PathBuf::from("crawl.db"),
        max_pages: 100,
        workers: 4,
        max_depth: 2,
    };
    let written = run(cfg).await?;
    println!("crawl complete. {written} pages saved to crawl.db");
    Ok(())
}

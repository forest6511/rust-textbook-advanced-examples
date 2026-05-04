use crate::fetch::{build_client, fetch};
use crate::limiter::Limiters;
use crate::parse::LinkExtractor;
use crate::queue::VisitedSet;
use crate::storage::spawn_writer;
use crate::types::{CrawlError, CrawlResult, Page};
use anyhow::{Result, anyhow};
use backon::{ExponentialBuilder, Retryable};
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Client;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;
use tokio::sync::{Notify, Semaphore, mpsc, watch};
use tokio::task::JoinSet;
use tracing::{info, warn};
use url::Url;

pub struct Config {
    pub seeds: Vec<Url>,
    pub db_path: PathBuf,
    pub max_pages: usize,
    pub workers: usize,
    pub max_depth: u32,
}

#[derive(Clone)]
struct Ctx {
    client: Arc<Client>,
    limiters: Arc<Limiters>,
    extractor: Arc<LinkExtractor>,
    visited: Arc<VisitedSet>,
    url_tx: mpsc::UnboundedSender<(Url, u32)>,
    result_tx: mpsc::Sender<CrawlResult>,
    bar: Arc<ProgressBar>,
    max_depth: u32,
    in_flight: Arc<AtomicUsize>,
    done: Arc<Notify>,
}

pub async fn run(cfg: Config) -> Result<usize> {
    let client = Arc::new(build_client()?);
    let limiters = Arc::new(Limiters::new());
    let extractor = Arc::new(LinkExtractor::new());
    let visited = Arc::new(VisitedSet::new());
    let in_flight = Arc::new(AtomicUsize::new(0));
    let done = Arc::new(Notify::new());

    let (url_tx, mut url_rx) = mpsc::unbounded_channel::<(Url, u32)>();
    let (result_tx, result_rx) = mpsc::channel::<CrawlResult>(64);
    let (shutdown_tx, mut shutdown_rx) = watch::channel(false);

    let writer = spawn_writer(cfg.db_path.clone(), result_rx);

    let bar = Arc::new(ProgressBar::new(cfg.max_pages as u64));
    bar.set_style(ProgressStyle::with_template("{bar:40} {pos}/{len} {msg}")?);

    for seed in &cfg.seeds {
        if visited.insert(seed.clone()) {
            in_flight.fetch_add(1, Ordering::SeqCst);
            url_tx.send((seed.clone(), 0)).ok();
        }
    }

    let sem = Arc::new(Semaphore::new(cfg.workers));
    let mut tasks: JoinSet<()> = JoinSet::new();
    let ctx = Ctx {
        client,
        limiters,
        extractor,
        visited: visited.clone(),
        url_tx: url_tx.clone(),
        result_tx: result_tx.clone(),
        bar: bar.clone(),
        max_depth: cfg.max_depth,
        in_flight: in_flight.clone(),
        done: done.clone(),
    };

    let scheduler = async {
        loop {
            tokio::select! {
                () = done.notified() => {
                    info!("all in-flight work completed");
                    break;
                }
                changed = shutdown_rx.changed() => {
                    if changed.is_ok() && *shutdown_rx.borrow() {
                        break;
                    }
                }
                next = url_rx.recv() => {
                    let Some((url, depth)) = next else { break };
                    if visited.len() > cfg.max_pages {
                        decrement(&in_flight, &done);
                        continue;
                    }
                    let permit = sem.clone().acquire_owned().await
                        .expect("semaphore should not be closed");
                    let ctx = ctx.clone();
                    tasks.spawn(async move {
                        process_url(&ctx, url, depth).await;
                        decrement(&ctx.in_flight, &ctx.done);
                        drop(permit);
                    });
                }
            }
        }
    };

    tokio::select! {
        _ = tokio::signal::ctrl_c() => {
            warn!("Ctrl+C received, shutting down");
            let _ = shutdown_tx.send(true);
        }
        () = scheduler => {}
    }

    drop(url_tx);
    drop(result_tx);
    drop(ctx);
    info!("draining tasks");

    let drain = async { while tasks.join_next().await.is_some() {} };
    let _ = tokio::time::timeout(Duration::from_secs(30), drain).await;
    info!("tasks drained, joining writer");

    let written = tokio::task::spawn_blocking(move || {
        writer.join().map_err(|_| anyhow!("writer thread panicked"))?
    })
    .await
    .map_err(|e| anyhow!("spawn_blocking failed: {e}"))??;
    info!(written, "writer joined");
    bar.finish_with_message(format!("written {written} pages"));
    info!(crawled = visited.len(), written, "crawl finished");
    Ok(written)
}

fn decrement(in_flight: &AtomicUsize, done: &Notify) {
    if in_flight.fetch_sub(1, Ordering::SeqCst) == 1 {
        done.notify_one();
    }
}

async fn process_url(ctx: &Ctx, url: Url, depth: u32) {
    let host = url.host_str().unwrap_or("").to_string();
    ctx.limiters.acquire(&host).await;

    let result = match fetch_with_retry(&ctx.client, &url).await {
        Ok(p) => ctx.extractor.extract(p),
        Err(e) => {
            warn!(error = %e, url = %url, "fetch failed");
            return;
        }
    };
    ctx.bar.inc(1);

    if depth < ctx.max_depth {
        for link in &result.links {
            if ctx.visited.insert(link.clone()) {
                ctx.in_flight.fetch_add(1, Ordering::SeqCst);
                if ctx.url_tx.send((link.clone(), depth + 1)).is_err() {
                    decrement(&ctx.in_flight, &ctx.done);
                }
            }
        }
    }
    let _ = ctx.result_tx.send(result).await;
}

pub async fn fetch_with_retry(
    client: &Client,
    url: &Url,
) -> Result<Page, CrawlError> {
    (|| async { fetch(client, url).await })
        .retry(
            ExponentialBuilder::default()
                .with_min_delay(Duration::from_millis(200))
                .with_max_delay(Duration::from_secs(30))
                .with_max_times(5)
                .with_jitter(),
        )
        .when(|e: &CrawlError| e.is_retryable())
        .notify(|err, dur| warn!(?err, ?dur, "retry scheduled"))
        .await
}

use ch11_web_crawler::parse::LinkExtractor;
use ch11_web_crawler::types::Page;
use criterion::{Criterion, criterion_group, criterion_main};
use rayon::prelude::*;
use std::hint::black_box;
use std::sync::Arc;
use url::Url;

const SAMPLE_HTML: &str = include_str!("sample_page.html");

fn make_pages(n: usize) -> Vec<Page> {
    (0..n)
        .map(|i| Page {
            url: Url::parse(&format!("https://example.com/{i}")).unwrap(),
            status: 200,
            html: SAMPLE_HTML.to_string(),
            fetched_at: 0,
        })
        .collect()
}

fn parse_sequential(pages: &[Page]) -> usize {
    let extractor = LinkExtractor::new();
    pages
        .iter()
        .map(|p| extractor.extract(p.clone()).links.len())
        .sum()
}

fn parse_rayon(pages: &[Page]) -> usize {
    let extractor = LinkExtractor::new();
    pages
        .par_iter()
        .map(|p| extractor.extract(p.clone()).links.len())
        .sum()
}

async fn parse_spawn_blocking(pages: Vec<Page>) -> usize {
    let extractor = Arc::new(LinkExtractor::new());
    let mut handles = Vec::with_capacity(pages.len());
    for p in pages {
        let ex = extractor.clone();
        handles.push(tokio::task::spawn_blocking(move || {
            ex.extract(p).links.len()
        }));
    }
    let mut total = 0usize;
    for h in handles {
        total += h.await.expect("spawn_blocking task panicked");
    }
    total
}

fn bench(c: &mut Criterion) {
    let pages = make_pages(100);
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("tokio runtime");

    let mut group = c.benchmark_group("parse_100_pages");
    group.bench_function("sequential", |b| {
        b.iter(|| black_box(parse_sequential(black_box(&pages))));
    });
    group.bench_function("rayon_par_iter", |b| {
        b.iter(|| black_box(parse_rayon(black_box(&pages))));
    });
    group.bench_function("tokio_spawn_blocking", |b| {
        b.iter(|| {
            rt.block_on(async {
                black_box(parse_spawn_blocking(black_box(pages.clone())).await)
            })
        });
    });
    group.finish();
}

criterion_group!(benches, bench);
criterion_main!(benches);

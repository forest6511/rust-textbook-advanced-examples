use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};
use tokio::runtime::Runtime;

async fn checksum_async(data: &[u8]) -> u64 {
    tokio::task::yield_now().await;
    data.iter().map(|&b| b as u64).sum()
}

fn bench_checksum(c: &mut Criterion) {
    let rt = Runtime::new().expect("tokio runtime");
    let data = vec![0xAAu8; 4096];

    c.bench_function("checksum_async_4kb", |b| {
        b.to_async(&rt)
            .iter(|| async { black_box(checksum_async(&data).await) });
    });
}

criterion_group!(benches, bench_checksum);
criterion_main!(benches);

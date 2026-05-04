use ch10_memory_opt::capacity;
use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

fn bench_capacity(c: &mut Criterion) {
    let items: Vec<u32> = (0..1024).collect();
    let mut g = c.benchmark_group("capacity");
    g.bench_function("Vec::new", |b| {
        b.iter(|| {
            let v = capacity::collect_with_default(black_box(&items));
            black_box(v.len())
        })
    });
    g.bench_function("Vec::with_capacity", |b| {
        b.iter(|| {
            let v = capacity::collect_with_capacity(black_box(&items));
            black_box(v.len())
        })
    });
    g.finish();
}

criterion_group!(benches, bench_capacity);
criterion_main!(benches);

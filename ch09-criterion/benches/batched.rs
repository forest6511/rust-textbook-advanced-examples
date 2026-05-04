use std::hint::black_box;

use criterion::{BatchSize, Criterion, criterion_group, criterion_main};

fn bench_sort_naive(c: &mut Criterion) {
    c.bench_function("sort_naive", |b| {
        b.iter(|| {
            let mut v: Vec<i32> = (0..1000).rev().collect();
            v.sort();
            black_box(v);
        });
    });
}

fn bench_sort_batched(c: &mut Criterion) {
    c.bench_function("sort_batched_ref", |b| {
        b.iter_batched_ref(
            || (0..1000).rev().collect::<Vec<i32>>(),
            |v| v.sort(),
            BatchSize::SmallInput,
        );
    });
}

criterion_group!(benches, bench_sort_naive, bench_sort_batched);
criterion_main!(benches);

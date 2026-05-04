use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};

fn fib_recursive(n: u64) -> u64 {
    if n < 2 {
        n
    } else {
        fib_recursive(n - 1) + fib_recursive(n - 2)
    }
}

fn bench_fib_for_profile(c: &mut Criterion) {
    c.bench_function("fib_recursive_profile_25", |b| {
        b.iter(|| fib_recursive(black_box(25)));
    });
}

criterion_group!(benches, bench_fib_for_profile);
criterion_main!(benches);

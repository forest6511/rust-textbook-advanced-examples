use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};

fn fib_recursive(n: u64) -> u64 {
    if n < 2 {
        n
    } else {
        fib_recursive(n - 1) + fib_recursive(n - 2)
    }
}

fn fib_iterative(n: u64) -> u64 {
    let (mut a, mut b) = (0u64, 1u64);
    for _ in 0..n {
        let next = a + b;
        a = b;
        b = next;
    }
    a
}

fn bench_fib(c: &mut Criterion) {
    c.bench_function("fib_recursive_20", |b| {
        b.iter(|| fib_recursive(black_box(20)));
    });
    c.bench_function("fib_iterative_20", |b| {
        b.iter(|| fib_iterative(black_box(20)));
    });
}

criterion_group!(benches, bench_fib);
criterion_main!(benches);

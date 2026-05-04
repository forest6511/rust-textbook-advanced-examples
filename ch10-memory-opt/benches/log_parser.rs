use ch10_memory_opt::log_parser;
use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

fn bench_log_parser(c: &mut Criterion) {
    let data = log_parser::sample_log();
    let mut g = c.benchmark_group("log_parser");
    g.bench_function("naive_lines", |b| {
        b.iter(|| black_box(log_parser::parse_naive(black_box(data.as_bytes()))))
    });
    g.bench_function("reuse_read_line", |b| {
        b.iter(|| black_box(log_parser::parse_reuse(black_box(data.as_bytes()))))
    });
    g.finish();
}

criterion_group!(benches, bench_log_parser);
criterion_main!(benches);

use std::hint::black_box;

use criterion::{
    BenchmarkId, Criterion, Throughput, criterion_group, criterion_main,
};

fn count_lines(bytes: &[u8]) -> usize {
    bytes.iter().filter(|&&b| b == b'\n').count()
}

fn bench_count_lines(c: &mut Criterion) {
    let mut group = c.benchmark_group("count_lines");
    for &size_kb in &[4u64, 16, 64, 256] {
        let bytes = make_log(size_kb as usize * 1024);
        let lines = count_lines(&bytes) as u64;

        group.throughput(Throughput::ElementsAndBytes {
            elements: lines,
            bytes: bytes.len() as u64,
        });
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{size_kb}KB")),
            &bytes,
            |b, input| b.iter(|| count_lines(black_box(input))),
        );
    }
    group.finish();
}

fn make_log(target_bytes: usize) -> Vec<u8> {
    let line = b"2026-05-04T08:30:00 INFO request id=42 path=/api/v1/users\n";
    let repeats = target_bytes / line.len() + 1;
    line.repeat(repeats)
}

criterion_group!(benches, bench_count_lines);
criterion_main!(benches);

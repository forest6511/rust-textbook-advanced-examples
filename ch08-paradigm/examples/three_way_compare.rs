//! 同じ CPU bound タスクを thread / async / rayon の 3 パラダイムで実行し、
//! 経過時間を比較する。`Instant` 1 回計測の簡易ベンチで、criterion で本格計測する
//! 前のオーダー把握に使う。

use std::hint::black_box;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::thread;
use std::time::Instant;

use rayon::prelude::*;
use tokio::runtime::Builder;
use tokio::task::JoinSet;

const N: u64 = 10_000_000;

fn cheap(x: u64) -> u64 {
    let mut acc = x;
    for _ in 0..16 {
        acc = acc.wrapping_mul(31).wrapping_add(7);
        acc = black_box(acc);
    }
    acc
}

fn run_sequential() -> (u64, std::time::Duration) {
    let t0 = Instant::now();
    let sum: u64 = (0..N).map(cheap).sum();
    (sum, t0.elapsed())
}

fn run_threads(n_threads: usize) -> (u64, std::time::Duration) {
    let t0 = Instant::now();
    let total = Arc::new(AtomicU64::new(0));
    let chunk = N / n_threads as u64;
    let mut handles = Vec::with_capacity(n_threads);
    for i in 0..n_threads {
        let total = Arc::clone(&total);
        let start = i as u64 * chunk;
        let end = if i == n_threads - 1 { N } else { start + chunk };
        handles.push(thread::spawn(move || {
            let local: u64 = (start..end).map(cheap).sum();
            total.fetch_add(local, Ordering::Relaxed);
        }));
    }
    for h in handles {
        h.join().expect("worker thread panicked");
    }
    let sum = total.load(Ordering::Relaxed);
    (sum, t0.elapsed())
}

fn run_async(n_tasks: usize) -> (u64, std::time::Duration) {
    let rt = Builder::new_multi_thread()
        .worker_threads(n_tasks)
        .build()
        .expect("failed to build runtime");
    let t0 = Instant::now();
    let sum = rt.block_on(async {
        let mut set = JoinSet::new();
        let chunk = N / n_tasks as u64;
        for i in 0..n_tasks {
            let start = i as u64 * chunk;
            let end = if i == n_tasks - 1 { N } else { start + chunk };
            set.spawn(async move { (start..end).map(cheap).sum::<u64>() });
        }
        let mut total: u64 = 0;
        while let Some(res) = set.join_next().await {
            total = total.wrapping_add(res.expect("task panicked"));
        }
        total
    });
    (sum, t0.elapsed())
}

fn run_rayon() -> (u64, std::time::Duration) {
    let t0 = Instant::now();
    let sum: u64 = (0..N).into_par_iter().map(cheap).sum();
    (sum, t0.elapsed())
}

fn main() {
    let cores = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(4);
    println!("cores            : {cores}");

    let (s0, d0) = run_sequential();
    let (s1, d1) = run_threads(cores);
    let (s2, d2) = run_async(cores);
    let (s3, d3) = run_rayon();

    assert_eq!(s0, s1);
    assert_eq!(s0, s2);
    assert_eq!(s0, s3);

    println!("sequential       : {d0:?}");
    println!("std::thread x{cores:<2}    : {d1:?}");
    println!("tokio task  x{cores:<2}    : {d2:?}");
    println!("rayon par_iter   : {d3:?}");
}

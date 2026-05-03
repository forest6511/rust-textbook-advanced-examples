//! Tokio で I/O を多重化し、`spawn_blocking` の中で rayon を使って CPU bound 処理を
//! 行うハイブリッドパターン。Web クローラ的な構成のミニチュアで、I/O 部分を
//! `tokio::time::sleep` で模擬している。

use std::time::Instant;

use rayon::prelude::*;
use tokio::task::JoinSet;
use tokio::time::{Duration, sleep};

async fn fetch(id: u64) -> Vec<u64> {
    sleep(Duration::from_millis(50)).await;
    (0..100_000_u64).map(|x| x.wrapping_add(id)).collect()
}

async fn analyze(payload: Vec<u64>) -> u64 {
    tokio::task::spawn_blocking(move || {
        payload.par_iter().map(|&x| x.wrapping_mul(31)).sum::<u64>()
    })
    .await
    .expect("rayon job panicked")
}

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() {
    let t0 = Instant::now();

    let mut set = JoinSet::new();
    for id in 0..10 {
        set.spawn(async move {
            let payload = fetch(id).await;
            analyze(payload).await
        });
    }

    let mut total: u64 = 0;
    while let Some(res) = set.join_next().await {
        total = total.wrapping_add(res.expect("task panicked"));
    }

    let elapsed = t0.elapsed();
    println!("hybrid total     : {total}");
    println!("hybrid elapsed   : {elapsed:?}");
}

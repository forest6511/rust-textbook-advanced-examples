use rayon::ThreadPoolBuilder;
use rayon::prelude::*;

fn main() {
    let pool = ThreadPoolBuilder::new()
        .num_threads(4)
        .thread_name(|i| format!("worker-{i}"))
        .build()
        .expect("failed to build pool");

    let result: i64 = pool.install(|| {
        let v: Vec<i64> = (0..1_000_000).collect();
        v.par_iter().sum()
    });

    println!("pool size = {}", pool.current_num_threads());
    println!("result    = {result}");
}

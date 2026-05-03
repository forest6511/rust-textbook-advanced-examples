use tokio::task::JoinSet;
use tokio::time::{Duration, sleep};

#[tokio::main]
async fn main() {
    let mut set: JoinSet<u32> = JoinSet::new();

    for i in 1..=4u32 {
        set.spawn(async move {
            sleep(Duration::from_millis(20 * (5 - i) as u64)).await;
            i
        });
    }

    let mut results = Vec::with_capacity(4);
    while let Some(res) = set.join_next().await {
        results.push(res.expect("task panicked"));
    }

    println!("completion order = {results:?}");
}

use std::pin::pin;
use tokio::time::{Duration, sleep};

async fn count_up() -> u32 {
    sleep(Duration::from_millis(30)).await;
    7
}

#[tokio::main]
async fn main() {
    let fut = count_up();
    let mut fut = pin!(fut);

    tokio::select! {
        v = fut.as_mut() => println!("first: {v}"),
        _ = sleep(Duration::from_millis(100)) => println!("first: timeout"),
    }
}

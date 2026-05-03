use tokio::sync::oneshot;
use tokio::time::{Duration, sleep};

async fn fetch_value() -> u32 {
    let (tx, rx) = oneshot::channel::<u32>();

    tokio::spawn(async move {
        sleep(Duration::from_millis(20)).await;
        let _ = tx.send(42);
    });

    rx.await.expect("worker dropped sender")
}

#[tokio::main]
async fn main() {
    let v = fetch_value().await;
    println!("value = {v}");
}

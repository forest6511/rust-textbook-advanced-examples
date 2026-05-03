use std::sync::{Arc, Mutex};
use tokio::time::{Duration, sleep};

#[tokio::main]
async fn main() {
    let counter = Arc::new(Mutex::new(0u64));
    let mut handles = Vec::new();

    for _ in 0..4 {
        let counter = Arc::clone(&counter);
        handles.push(tokio::spawn(async move {
            {
                let mut g = counter.lock().expect("mutex poisoned");
                *g += 1;
            }
            sleep(Duration::from_millis(10)).await;
        }));
    }

    for h in handles {
        h.await.expect("task panicked");
    }

    println!("counter = {}", counter.lock().expect("mutex poisoned"));
}

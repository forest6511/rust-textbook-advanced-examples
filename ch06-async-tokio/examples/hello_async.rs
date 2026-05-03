use tokio::time::{Duration, sleep};

async fn greet(name: &str) -> String {
    sleep(Duration::from_millis(50)).await;
    format!("hello, {name}")
}

#[tokio::main]
async fn main() {
    let msg = greet("tokio").await;
    println!("{msg}");
}

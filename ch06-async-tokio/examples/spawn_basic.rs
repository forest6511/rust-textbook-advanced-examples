use tokio::time::{Duration, sleep};

#[tokio::main]
async fn main() {
    let h1 = tokio::spawn(async {
        sleep(Duration::from_millis(30)).await;
        1u32
    });
    let h2 = tokio::spawn(async {
        sleep(Duration::from_millis(10)).await;
        2u32
    });

    let a = h1.await.expect("task 1 panicked");
    let b = h2.await.expect("task 2 panicked");

    println!("a + b = {}", a + b);
}

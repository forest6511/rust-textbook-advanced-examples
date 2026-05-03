use tokio::time::{Duration, sleep, timeout};

async fn slow_op() -> u32 {
    sleep(Duration::from_millis(200)).await;
    42
}

#[tokio::main]
async fn main() {
    let result = tokio::select! {
        v = slow_op() => format!("ok: {v}"),
        _ = sleep(Duration::from_millis(50)) => "timed out".to_string(),
    };
    println!("select  -> {result}");

    match timeout(Duration::from_millis(50), slow_op()).await {
        Ok(v) => println!("timeout -> ok: {v}"),
        Err(_) => println!("timeout -> elapsed"),
    }
}

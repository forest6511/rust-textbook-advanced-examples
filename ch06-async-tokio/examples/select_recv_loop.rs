use tokio::sync::mpsc;
use tokio::time::{Duration, sleep};

#[tokio::main]
async fn main() {
    let (tx_data, mut rx_data) = mpsc::channel::<u32>(8);
    let (tx_stop, mut rx_stop) = mpsc::channel::<()>(1);

    tokio::spawn(async move {
        for i in 0..3 {
            tx_data.send(i).await.expect("data closed");
            sleep(Duration::from_millis(20)).await;
        }
        tx_stop.send(()).await.expect("stop closed");
    });

    loop {
        tokio::select! {
            Some(n) = rx_data.recv() => println!("data: {n}"),
            Some(_) = rx_stop.recv() => {
                println!("stop received");
                break;
            }
            else => break,
        }
    }
}

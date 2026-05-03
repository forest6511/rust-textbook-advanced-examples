use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel::<u32>(8);

    for i in 0..3u32 {
        let tx = tx.clone();
        tokio::spawn(async move {
            tx.send(i * 10).await.expect("receiver dropped");
        });
    }
    drop(tx);

    while let Some(n) = rx.recv().await {
        println!("received: {n}");
    }
}

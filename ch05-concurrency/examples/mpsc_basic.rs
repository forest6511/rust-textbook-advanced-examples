use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel::<u32>();

    for i in 0..3 {
        let tx = tx.clone();
        thread::spawn(move || {
            tx.send(i * 10).expect("receiver disconnected");
        });
    }
    drop(tx);

    while let Ok(n) = rx.recv() {
        println!("received: {n}");
    }
}

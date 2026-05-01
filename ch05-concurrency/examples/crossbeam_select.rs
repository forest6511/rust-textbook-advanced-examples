use crossbeam_channel::{select, unbounded};
use std::thread;
use std::time::Duration;

fn main() {
    let (tx_data, rx_data) = unbounded::<u32>();
    let (tx_stop, rx_stop) = unbounded::<()>();

    thread::spawn(move || {
        for i in 0..3 {
            tx_data.send(i).expect("data channel closed");
            thread::sleep(Duration::from_millis(50));
        }
        tx_stop.send(()).expect("stop channel closed");
    });

    loop {
        select! {
            recv(rx_data) -> msg => {
                if let Ok(n) = msg {
                    println!("data: {n}");
                }
            },
            recv(rx_stop) -> _ => {
                println!("stop signal received");
                break;
            },
        }
    }
}

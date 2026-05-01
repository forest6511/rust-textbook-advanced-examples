use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::thread;

fn main() {
    let counter = Arc::new(AtomicU64::new(0));
    let mut handles = Vec::with_capacity(4);

    for _ in 0..4 {
        let counter = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            for _ in 0..1000 {
                counter.fetch_add(1, Ordering::Relaxed);
            }
        }));
    }

    for h in handles {
        h.join().expect("thread panicked");
    }

    println!("counter = {}", counter.load(Ordering::Relaxed));
}

use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::thread;

fn fetch_max(target: &AtomicU64, candidate: u64) {
    let mut current = target.load(Ordering::Relaxed);
    while candidate > current {
        match target.compare_exchange_weak(
            current, candidate, Ordering::Relaxed, Ordering::Relaxed,
        ) {
            Ok(_) => return,
            Err(observed) => current = observed,
        }
    }
}

fn main() {
    let high = Arc::new(AtomicU64::new(0));
    let mut handles = Vec::with_capacity(4);
    for i in 1..=4 {
        let high = Arc::clone(&high);
        handles.push(thread::spawn(move || fetch_max(&high, i * 100)));
    }
    for h in handles {
        h.join().expect("thread panicked");
    }
    println!("max = {}", high.load(Ordering::Relaxed));
}

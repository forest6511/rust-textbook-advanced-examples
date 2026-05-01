use parking_lot::Mutex;
use std::sync::Arc;
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0u64));
    let mut handles = Vec::with_capacity(4);

    for _ in 0..4 {
        let counter = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            for _ in 0..1000 {
                let mut guard = counter.lock();
                *guard += 1;
            }
        }));
    }

    for h in handles {
        h.join().expect("thread panicked");
    }

    println!("counter = {}", counter.lock());
}

use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0u64));
    let mut handles = Vec::with_capacity(4);

    for _ in 0..4 {
        let counter = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            for _ in 0..1000 {
                let mut guard = counter.lock().expect("mutex poisoned");
                *guard += 1;
            }
        }));
    }

    for h in handles {
        h.join().expect("thread panicked");
    }

    let final_value = counter.lock().expect("mutex poisoned");
    println!("counter = {}", *final_value);
}

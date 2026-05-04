use std::sync::{Arc, Mutex};
use std::thread;

fn incr(a: &Arc<Mutex<u64>>, b: &Arc<Mutex<u64>>) {
    let mut ga = a.lock().expect("mutex poisoned");
    let mut gb = b.lock().expect("mutex poisoned");
    *ga += 1;
    *gb += 1;
}

fn main() {
    let a = Arc::new(Mutex::new(0u64));
    let b = Arc::new(Mutex::new(0u64));

    let mut handles = Vec::with_capacity(2);
    for _ in 0..2 {
        let a = Arc::clone(&a);
        let b = Arc::clone(&b);
        handles.push(thread::spawn(move || incr(&a, &b)));
    }
    for h in handles {
        h.join().expect("thread panicked");
    }

    let ga = a.lock().expect("mutex poisoned");
    let gb = b.lock().expect("mutex poisoned");
    println!("a = {}, b = {}", *ga, *gb);
}

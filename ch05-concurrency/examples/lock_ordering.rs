use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let a = Arc::new(Mutex::new(0u64));
    let b = Arc::new(Mutex::new(0u64));

    let a1 = Arc::clone(&a);
    let b1 = Arc::clone(&b);
    let t1 = thread::spawn(move || {
        let mut ga = a1.lock().expect("mutex poisoned");
        let mut gb = b1.lock().expect("mutex poisoned");
        *ga += 1;
        *gb += 1;
    });

    let a2 = Arc::clone(&a);
    let b2 = Arc::clone(&b);
    let t2 = thread::spawn(move || {
        let mut ga = a2.lock().expect("mutex poisoned");
        let mut gb = b2.lock().expect("mutex poisoned");
        *ga += 1;
        *gb += 1;
    });

    t1.join().expect("t1 panicked");
    t2.join().expect("t2 panicked");

    let ga = a.lock().expect("mutex poisoned");
    let gb = b.lock().expect("mutex poisoned");
    println!("a = {}, b = {}", *ga, *gb);
}

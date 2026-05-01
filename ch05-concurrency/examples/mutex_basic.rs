use std::sync::Mutex;
use std::thread;

fn main() {
    let counter = Mutex::new(0u64);

    thread::scope(|s| {
        for _ in 0..4 {
            s.spawn(|| {
                let mut guard = counter.lock().expect("mutex poisoned");
                *guard += 1;
            });
        }
    });

    let final_value = counter.lock().expect("mutex poisoned");
    println!("counter = {}", *final_value);
}

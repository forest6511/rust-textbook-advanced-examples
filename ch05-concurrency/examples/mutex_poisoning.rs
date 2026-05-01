use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let data = Arc::new(Mutex::new(vec![1, 2, 3]));

    let data2 = Arc::clone(&data);
    let _ = thread::spawn(move || {
        let mut guard = data2.lock().expect("mutex poisoned");
        guard.push(4);
        panic!("worker fails after mutating data");
    })
    .join();

    let recovered = match data.lock() {
        Ok(g) => g,
        Err(poisoned) => {
            eprintln!("recovering from poisoned mutex");
            poisoned.into_inner()
        }
    };

    println!("recovered data = {:?}", *recovered);
}

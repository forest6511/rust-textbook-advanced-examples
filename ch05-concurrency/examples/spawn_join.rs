use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        for i in 0..3 {
            println!("worker: {i}");
        }
        42
    });

    let result = handle.join().expect("worker thread panicked");
    println!("worker returned: {result}");
}

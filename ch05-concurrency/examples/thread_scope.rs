use std::thread;

fn main() {
    let data = vec![1, 2, 3, 4, 5];
    let mut sum = 0i32;

    thread::scope(|s| {
        let handle = s.spawn(|| data.iter().sum::<i32>());
        sum = handle.join().expect("worker panicked");
    });

    println!("sum = {sum}");
}

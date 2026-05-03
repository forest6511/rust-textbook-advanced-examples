use std::sync::mpsc::channel;
use std::thread;

use rayon::iter::ParallelBridge;
use rayon::prelude::*;

fn main() {
    let (tx, rx) = channel::<i32>();

    let producer = thread::spawn(move || {
        for i in 0..1_000 {
            tx.send(i).expect("receiver closed");
        }
    });

    let sum: i64 = rx.into_iter().par_bridge().map(|x| x as i64).sum();
    producer.join().expect("producer panicked");

    println!("par_bridge sum    = {sum}");
    println!("expected (0..1000) sum = {}", (0..1_000_i64).sum::<i64>());
}

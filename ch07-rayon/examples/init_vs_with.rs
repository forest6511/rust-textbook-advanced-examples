use std::sync::mpsc::channel;

use rayon::prelude::*;

fn main() {
    let (tx, rx) = channel::<u32>();
    (0..5_u32)
        .into_par_iter()
        .for_each_with(tx, |s, x| s.send(x).expect("channel closed"));
    let mut got: Vec<u32> = rx.iter().collect();
    got.sort_unstable();
    println!("for_each_with collected = {got:?}");

    let mut buf = vec![0_u8; 32];
    buf.par_chunks_mut(8).for_each_init(
        || vec![0_u8; 8],
        |scratch, chunk| {
            for (i, slot) in chunk.iter_mut().enumerate() {
                scratch[i] = (i as u8).wrapping_mul(3);
                *slot = scratch[i];
            }
        },
    );
    println!("for_each_init buf       = {buf:?}");
}

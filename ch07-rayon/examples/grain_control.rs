use std::time::Instant;

use rayon::prelude::*;

fn cheap(x: u64) -> u64 {
    x.wrapping_mul(31).wrapping_add(7)
}

fn main() {
    let n: u64 = 10_000_000;
    let v: Vec<u64> = (0..n).collect();

    let t0 = Instant::now();
    let s_seq: u64 = v.iter().map(|&x| cheap(x)).sum();
    let d_seq = t0.elapsed();

    let t0 = Instant::now();
    let s_default: u64 = v.par_iter().map(|&x| cheap(x)).sum();
    let d_default = t0.elapsed();

    let t0 = Instant::now();
    let s_tuned: u64 = v
        .par_iter()
        .with_min_len(4096)
        .map(|&x| cheap(x))
        .sum();
    let d_tuned = t0.elapsed();

    assert_eq!(s_seq, s_default);
    assert_eq!(s_seq, s_tuned);

    println!("sequential        : {d_seq:?}");
    println!("par_iter (default): {d_default:?}");
    println!("with_min_len(4096): {d_tuned:?}");
}

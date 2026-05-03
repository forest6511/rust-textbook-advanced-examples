use rayon::prelude::*;

fn main() {
    let v: Vec<i32> = (0..1_000_000).collect();
    let sum: i64 = v.par_iter().map(|&x| x as i64).sum();
    println!("par_iter sum = {sum}");

    let mut w = vec![0_i32; 8];
    w.par_iter_mut().enumerate().for_each(|(i, x)| *x = i as i32);
    println!("par_iter_mut = {w:?}");

    let total: u64 = (1..=1_000_000_u64).into_par_iter().sum();
    println!("into_par_iter total = {total}");
}

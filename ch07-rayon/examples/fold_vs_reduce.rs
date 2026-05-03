use rayon::prelude::*;

fn main() {
    let n: i64 = (1..=1_000_000_i64)
        .into_par_iter()
        .reduce(|| 0, |a, b| a + b);
    println!("reduce sum = {n}");

    let s: String = ['a', 'b', 'c', 'd', 'e']
        .par_iter()
        .fold(String::new, |mut acc, c| {
            acc.push(*c);
            acc
        })
        .reduce(String::new, |mut a, b| {
            a.push_str(&b);
            a
        });
    println!("fold+reduce concat = {s}");

    let bad: String = ['a', 'b', 'c', 'd', 'e']
        .par_iter()
        .map(|c| c.to_string())
        .reduce(String::new, |mut a, b| {
            a.push_str(&b);
            a
        });
    println!("map+reduce concat  = {bad}");
}

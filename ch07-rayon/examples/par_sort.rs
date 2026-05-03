use rayon::prelude::*;

fn main() {
    let mut a: Vec<i32> = (0..100_000).rev().collect();
    a.par_sort_unstable();
    assert!(a.windows(2).all(|w| w[0] <= w[1]));
    println!("unstable sorted   = {:?}", &a[..5]);

    let mut b: Vec<(i32, &str)> = vec![
        (2, "alpha"),
        (1, "bravo"),
        (2, "charlie"),
        (1, "delta"),
    ];
    b.par_sort_by_key(|p| p.0);
    println!("stable by_key     = {b:?}");

    let mut c: Vec<String> = (0..1_000)
        .map(|i| format!("Item-{i:04}"))
        .collect();
    c.par_sort_by_cached_key(|s| s.to_lowercase());
    println!("by_cached_key[..3] = {:?}", &c[..3]);
}

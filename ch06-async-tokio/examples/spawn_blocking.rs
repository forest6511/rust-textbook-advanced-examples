use tokio::task;

fn cpu_heavy(n: u64) -> u64 {
    (0..n).fold(0u64, |acc, x| acc.wrapping_add(x.wrapping_mul(x)))
}

#[tokio::main]
async fn main() {
    let h1 = task::spawn_blocking(|| cpu_heavy(1_000_000));
    let h2 = task::spawn_blocking(|| cpu_heavy(2_000_000));

    let (a, b) = tokio::join!(h1, h2);
    let a = a.expect("blocking task panicked");
    let b = b.expect("blocking task panicked");

    println!("sum = {}", a.wrapping_add(b));
}

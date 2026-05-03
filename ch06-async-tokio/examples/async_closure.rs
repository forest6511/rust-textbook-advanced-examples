async fn run_each<F>(items: Vec<u32>, mut f: F) -> Vec<u32>
where
    F: AsyncFnMut(u32) -> u32,
{
    let mut out = Vec::with_capacity(items.len());
    for x in items {
        out.push(f(x).await);
    }
    out
}

#[tokio::main]
async fn main() {
    let mut total = 0u32;
    let collected = run_each(vec![1, 2, 3], async |x| {
        total += x;
        x * 10
    })
    .await;

    println!("collected = {collected:?}");
    println!("total     = {total}");
}

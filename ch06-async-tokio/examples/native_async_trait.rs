trait Worker {
    async fn run(&self, n: u32) -> u32;
}

struct Doubler;

impl Worker for Doubler {
    async fn run(&self, n: u32) -> u32 {
        tokio::time::sleep(std::time::Duration::from_millis(5)).await;
        n * 2
    }
}

async fn use_worker<W: Worker>(w: &W, n: u32) -> u32 {
    w.run(n).await
}

#[tokio::main]
async fn main() {
    let d = Doubler;
    let r = use_worker(&d, 21).await;
    println!("r = {r}");
}

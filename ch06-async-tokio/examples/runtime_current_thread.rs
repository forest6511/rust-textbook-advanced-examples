use tokio::runtime::Builder;
use tokio::time::{Duration, sleep};

fn main() {
    let rt = Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("failed to build runtime");

    let result = rt.block_on(async {
        sleep(Duration::from_millis(20)).await;
        "done"
    });

    println!("{result}");
}

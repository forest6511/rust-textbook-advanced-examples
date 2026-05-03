use tokio::io::{AsyncReadExt, AsyncWriteExt, duplex};
use tokio::time::{Duration, sleep};

#[tokio::main]
async fn main() {
    let (mut reader, mut writer) = duplex(64);

    tokio::spawn(async move {
        sleep(Duration::from_millis(20)).await;
        let _ = writer.write_all(b"hi").await;
    });

    let mut buf = [0u8; 4];
    tokio::select! {
        _ = reader.read_exact(&mut buf) => {
            println!("read_exact returned (data lost on cancel branch)");
        }
        _ = sleep(Duration::from_millis(5)) => {
            println!("timeout fired before read_exact filled buf");
        }
    }
}

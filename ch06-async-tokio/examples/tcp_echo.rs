use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:0").await?;
    let addr = listener.local_addr()?;
    println!("listening on {addr}");

    let client = tokio::spawn(async move {
        let mut s = tokio::net::TcpStream::connect(addr)
            .await
            .expect("connect failed");
        s.write_all(b"hello").await.expect("write failed");
        let mut buf = [0u8; 5];
        s.read_exact(&mut buf).await.expect("read failed");
        println!("client received: {}", String::from_utf8_lossy(&buf));
    });

    let (mut socket, _) = listener.accept().await?;
    let mut buf = [0u8; 5];
    socket.read_exact(&mut buf).await?;
    socket.write_all(&buf).await?;

    client.await.expect("client task panicked");
    Ok(())
}

use async_trait::async_trait;

#[async_trait]
trait Greeter: Send + Sync {
    async fn greet(&self, name: &str) -> String;
}

struct Plain;

#[async_trait]
impl Greeter for Plain {
    async fn greet(&self, name: &str) -> String {
        format!("hello, {name}")
    }
}

#[tokio::main]
async fn main() {
    let g: Box<dyn Greeter> = Box::new(Plain);
    println!("{}", g.greet("dyn").await);
}

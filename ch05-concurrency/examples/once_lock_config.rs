use std::sync::OnceLock;

struct Config {
    host: String,
    port: u16,
}

static CONFIG: OnceLock<Config> = OnceLock::new();

fn config() -> &'static Config {
    CONFIG.get_or_init(|| Config {
        host: std::env::var("APP_HOST").unwrap_or_else(|_| "localhost".into()),
        port: 8080,
    })
}

fn main() {
    let c = config();
    println!("host={} port={}", c.host, c.port);
}

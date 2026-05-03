use std::collections::HashMap;
use std::sync::LazyLock;

// 'static 領域の遅延初期化は Rust 1.80 で安定化された LazyLock を使う。
// 各スレッドからアクセスしても、初期化クロージャは一度しか実行されない。
static TABLE: LazyLock<HashMap<&str, u32>> = LazyLock::new(|| {
    let mut map = HashMap::new();
    map.insert("alpha", 1);
    map.insert("beta", 2);
    map
});

fn main() {
    println!("alpha = {}", TABLE["alpha"]);
    println!("beta  = {}", TABLE["beta"]);
    println!("entries = {}", TABLE.len());
}

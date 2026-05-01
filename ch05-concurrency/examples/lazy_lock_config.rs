use std::collections::HashMap;
use std::sync::LazyLock;

static FEATURE_FLAGS: LazyLock<HashMap<&'static str, bool>> =
    LazyLock::new(|| {
        let mut m = HashMap::new();
        m.insert("dark_mode", true);
        m.insert("beta_api", false);
        m
    });

fn main() {
    println!("dark_mode = {}", FEATURE_FLAGS["dark_mode"]);
    println!("beta_api = {}", FEATURE_FLAGS["beta_api"]);
}

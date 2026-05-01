use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::thread;

fn main() {
    let cache: Arc<RwLock<HashMap<String, u64>>> =
        Arc::new(RwLock::new(HashMap::new()));

    cache
        .write()
        .expect("rwlock poisoned")
        .insert("hits".into(), 0);

    let mut handles = Vec::with_capacity(4);
    for i in 0..4 {
        let cache = Arc::clone(&cache);
        handles.push(thread::spawn(move || {
            let guard = cache.read().expect("rwlock poisoned");
            let hits = guard.get("hits").copied().unwrap_or(0);
            println!("reader {i} sees hits = {hits}");
        }));
    }

    for h in handles {
        h.join().expect("thread panicked");
    }
}

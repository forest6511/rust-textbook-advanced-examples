use parking_lot::Mutex;
use std::collections::HashSet;
use url::Url;

pub struct VisitedSet {
    inner: Mutex<HashSet<Url>>,
}

impl VisitedSet {
    pub fn new() -> Self {
        Self { inner: Mutex::new(HashSet::new()) }
    }

    pub fn insert(&self, url: Url) -> bool {
        self.inner.lock().insert(url)
    }

    pub fn len(&self) -> usize {
        self.inner.lock().len()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.lock().is_empty()
    }
}

impl Default for VisitedSet {
    fn default() -> Self {
        Self::new()
    }
}

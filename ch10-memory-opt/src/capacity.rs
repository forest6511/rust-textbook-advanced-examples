pub fn collect_with_default(items: &[u32]) -> Vec<u32> {
    let mut v = Vec::new();
    for &x in items {
        v.push(x);
    }
    v
}

pub fn collect_with_capacity(items: &[u32]) -> Vec<u32> {
    let mut v = Vec::with_capacity(items.len());
    for &x in items {
        v.push(x);
    }
    v
}

pub fn split_per_iter(lines: &[&str]) -> usize {
    let mut total = 0;
    for line in lines {
        let parts: Vec<&str> = line.split(',').collect();
        total += parts.len();
    }
    total
}

pub fn split_reuse(lines: &[&str]) -> usize {
    let mut parts: Vec<&str> = Vec::with_capacity(8);
    let mut total = 0;
    for line in lines {
        parts.clear();
        parts.extend(line.split(','));
        total += parts.len();
    }
    total
}

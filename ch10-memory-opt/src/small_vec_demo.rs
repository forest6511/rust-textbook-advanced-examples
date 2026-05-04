use smallvec::{SmallVec, smallvec};
use std::mem::size_of;

pub fn show_layout() {
    println!(
        "size_of::<Vec<i32>>()           = {}",
        size_of::<Vec<i32>>(),
    );
    println!(
        "size_of::<SmallVec<[i32; 4]>>() = {}",
        size_of::<SmallVec<[i32; 4]>>(),
    );
}

pub fn measure_spill_rate(workloads: &[&[i32]]) -> (usize, usize) {
    let mut spilled = 0;
    let mut total = 0;
    for items in workloads {
        let mut v: SmallVec<[i32; 4]> = SmallVec::new();
        for &x in *items {
            v.push(x);
        }
        if v.spilled() {
            spilled += 1;
        }
        total += 1;
    }
    (spilled, total)
}

pub fn build_inline_demo() -> SmallVec<[i32; 4]> {
    smallvec![1, 2, 3]
}

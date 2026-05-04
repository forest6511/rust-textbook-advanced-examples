use std::mem::MaybeUninit;

pub fn make_array_from_fn() -> [String; 4] {
    std::array::from_fn(|i| format!("item{i}"))
}

pub fn make_array_maybe_uninit() -> [String; 4] {
    let mut arr: [MaybeUninit<String>; 4] =
        [const { MaybeUninit::uninit() }; 4];
    for (i, slot) in arr.iter_mut().enumerate() {
        slot.write(format!("item{i}"));
    }
    arr.map(|cell| unsafe { cell.assume_init() })
}

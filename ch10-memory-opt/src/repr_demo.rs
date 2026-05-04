use std::mem::size_of;

pub struct RustRepr {
    pub a: u8,
    pub b: u64,
    pub c: u8,
}

#[repr(C)]
pub struct CRepr {
    pub a: u8,
    pub b: u64,
    pub c: u8,
}

#[repr(transparent)]
pub struct UserId(pub u64);

pub fn print_repr_sizes() {
    println!("size_of::<RustRepr>()  = {}", size_of::<RustRepr>());
    println!("size_of::<CRepr>()     = {}", size_of::<CRepr>());
    println!("size_of::<UserId>()    = {}", size_of::<UserId>());
    println!("size_of::<u64>()       = {}", size_of::<u64>());
}

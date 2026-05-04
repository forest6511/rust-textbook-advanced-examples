use std::mem::{align_of, size_of};

pub fn print_primitive_sizes() {
    println!("size_of::<bool>()    = {}", size_of::<bool>());
    println!("size_of::<u32>()     = {}", size_of::<u32>());
    println!("size_of::<u64>()     = {}", size_of::<u64>());
    println!("size_of::<usize>()   = {}", size_of::<usize>());
    println!("size_of::<&u32>()    = {}", size_of::<&u32>());
    println!("size_of::<&[u8]>()   = {}", size_of::<&[u8]>());
    println!("size_of::<Vec<u8>>() = {}", size_of::<Vec<u8>>());
    println!("size_of::<String>()  = {}", size_of::<String>());
}

pub fn print_alignment() {
    println!("align_of::<u8>()  = {}", align_of::<u8>());
    println!("align_of::<u32>() = {}", align_of::<u32>());
    println!("align_of::<u64>() = {}", align_of::<u64>());
}

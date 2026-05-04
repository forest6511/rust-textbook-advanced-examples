use std::mem::size_of;

pub enum MessageBig {
    Small(u32),
    Large([u8; 64]),
}

pub enum MessageSmall {
    Small(u32),
    Large(Box<[u8; 64]>),
}

#[derive(Debug)]
pub struct DetailedError {
    pub code: u32,
    pub message: String,
    pub context: Vec<String>,
}

pub type FatResult = Result<u64, DetailedError>;
pub type SlimResult = Result<u64, Box<DetailedError>>;

pub fn print_enum_sizes() {
    println!("size_of::<MessageBig>()   = {}", size_of::<MessageBig>());
    println!("size_of::<MessageSmall>() = {}", size_of::<MessageSmall>());
    println!("size_of::<FatResult>()    = {}", size_of::<FatResult>());
    println!("size_of::<SlimResult>()   = {}", size_of::<SlimResult>());
}

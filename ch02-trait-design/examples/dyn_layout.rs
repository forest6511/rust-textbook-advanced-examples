// Box<dyn Trait> と &dyn Trait のメモリレイアウト計測。
//
// 64-bit プラットフォームでは、トレイトオブジェクトへのポインタは
// (データポインタ, vtable ポインタ) の 16 バイト fat pointer になる。
// 一方、具体型へのポインタや Box<具体型> は 8 バイトの thin pointer。

use std::mem;

trait Greet {
    fn hello(&self);
}

struct A;

impl Greet for A {
    fn hello(&self) {
        println!("hello from A");
    }
}

struct B {
    data: u64,
}

impl Greet for B {
    fn hello(&self) {
        println!("hello from B (data={})", self.data);
    }
}

fn main() {
    let a = A;
    let b = B { data: 42 };
    a.hello();
    b.hello();

    println!("size_of::<A>()              = {}", mem::size_of::<A>());
    println!("size_of::<B>()              = {}", mem::size_of::<B>());
    println!("size_of::<&A>()             = {}", mem::size_of::<&A>());
    println!(
        "size_of::<&dyn Greet>()     = {}",
        mem::size_of::<&dyn Greet>()
    );
    println!(
        "size_of::<Box<A>>()         = {}",
        mem::size_of::<Box<A>>()
    );
    println!(
        "size_of::<Box<dyn Greet>>() = {}",
        mem::size_of::<Box<dyn Greet>>()
    );
}

use std::marker::PhantomData;

struct ThreadLocalHandle {
    fd: i32,
    _not_send: PhantomData<*const ()>,
}

impl ThreadLocalHandle {
    fn new(fd: i32) -> Self {
        Self { fd, _not_send: PhantomData }
    }
}

fn assert_send<T: Send>() {}

fn main() {
    let h = ThreadLocalHandle::new(3);
    println!("fd = {}", h.fd);
    // assert_send::<ThreadLocalHandle>(); // コンパイルエラー
    let _ = assert_send::<i32>;
}

//! const ジェネリクスで「サイズが型に焼き込まれた」リングバッファを作る。
//!
//! - サイズが型レベルで固定されているので、ヒープアロケーションが不要。
//! - 異なる N を持つ RingBuffer は別の型として扱われる。
//! - const パラメータの型は u8〜i128, char, bool に限られる (公式 Reference)。

pub struct RingBuffer<const N: usize> {
    data: [Option<u32>; N],
    head: usize,
    len: usize,
}

impl<const N: usize> RingBuffer<N> {
    pub const fn new() -> Self {
        Self {
            data: [None; N],
            head: 0,
            len: 0,
        }
    }

    pub fn push(&mut self, value: u32) {
        let idx = (self.head + self.len) % N;
        self.data[idx] = Some(value);
        if self.len < N {
            self.len += 1;
        } else {
            self.head = (self.head + 1) % N;
        }
    }

    pub fn snapshot(&self) -> Vec<u32> {
        (0..self.len)
            .map(|i| self.data[(self.head + i) % N].unwrap())
            .collect()
    }
}

impl<const N: usize> Default for RingBuffer<N> {
    fn default() -> Self {
        Self::new()
    }
}

fn main() {
    let mut buf = RingBuffer::<4>::new();
    for v in [1, 2, 3, 4, 5, 6] {
        buf.push(v);
    }
    println!("snapshot: {:?}", buf.snapshot());

    let mut tiny = RingBuffer::<2>::new();
    for v in [10, 20, 30] {
        tiny.push(v);
    }
    println!("tiny    : {:?}", tiny.snapshot());
}

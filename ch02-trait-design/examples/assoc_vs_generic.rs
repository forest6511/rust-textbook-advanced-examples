// 関連型 (associated type) とジェネリクスの使い分け。
//
// 関連型: 1 つの型に対して 1 つだけ実装したい関係性
//   例: Iterator::Item — Vec<i32> のイテレータの Item は i32 一択
//
// ジェネリクス: 1 つの型に対して複数の実装を許したい関係性
//   例: From<T> — String は From<&str> も From<i32> (経由) も実装できる

trait Container {
    type Item;
    fn get(&self, index: usize) -> Option<&Self::Item>;
    fn len(&self) -> usize;
}

struct Stack<T> {
    data: Vec<T>,
}

impl<T> Container for Stack<T> {
    type Item = T;

    fn get(&self, index: usize) -> Option<&T> {
        self.data.get(index)
    }

    fn len(&self) -> usize {
        self.data.len()
    }
}

trait Convert<From> {
    fn convert(&self, value: From) -> String;
}

struct Stringifier;

impl Convert<i32> for Stringifier {
    fn convert(&self, value: i32) -> String {
        format!("int:{value}")
    }
}

impl Convert<&str> for Stringifier {
    fn convert(&self, value: &str) -> String {
        format!("str:{value}")
    }
}

fn main() {
    let stack = Stack {
        data: vec![1, 2, 3],
    };
    println!("stack.len()  = {}", stack.len());
    println!("stack.get(1) = {:?}", stack.get(1));

    let s = Stringifier;
    println!("{}", s.convert(42));
    println!("{}", s.convert("hello"));
}

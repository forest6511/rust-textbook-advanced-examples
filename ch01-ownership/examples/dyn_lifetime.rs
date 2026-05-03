// トレイトオブジェクトのデフォルトライフタイムは
// `Box<dyn Trait>` の場合 `'static`、`&dyn Trait` の場合は参照に紐づく。
trait Greeter {
    fn greet(&self) -> String;
}

struct StaticGreeter;

impl Greeter for StaticGreeter {
    fn greet(&self) -> String {
        "hello from static".into()
    }
}

struct BorrowedGreeter<'a> {
    name: &'a str,
}

impl<'a> Greeter for BorrowedGreeter<'a> {
    fn greet(&self) -> String {
        format!("hello, {}", self.name)
    }
}

// Box<dyn Greeter> は `Box<dyn Greeter + 'static>` と等価。
fn run_owned(g: Box<dyn Greeter>) -> String {
    g.greet()
}

// 借用付きトレイトオブジェクトはライフタイムを明示すると安全。
fn run_borrowed<'a>(g: &(dyn Greeter + 'a)) -> String {
    g.greet()
}

fn main() {
    let s: Box<dyn Greeter> = Box::new(StaticGreeter);
    println!("{}", run_owned(s));

    let name = String::from("world");
    let b = BorrowedGreeter { name: &name };
    println!("{}", run_borrowed(&b));
}
